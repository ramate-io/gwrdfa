pub mod hart;

use futures::{
	task::{Context, Poll},
	Future, Stream,
};
use libp2p::{
	gossipsub::{self, IdentTopic, MessageAuthenticity, TopicHash},
	identity::Keypair,
	kad::{self, store::MemoryStore},
	multiaddr::Protocol,
	noise,
	swarm::{NetworkBehaviour, SwarmEvent},
	tcp, yamux, Multiaddr, PeerId, Swarm,
};
use std::pin::Pin;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

#[derive(NetworkBehaviour)]
struct GossamerBehaviour {
	gossipsub: gossipsub::Behaviour,
	kad: kad::Behaviour<MemoryStore>,
}

#[derive(Debug, Clone)]
pub struct Config {
	pub identity: Keypair,
	pub topic: String,
	pub listen_on: Multiaddr,
	pub bootstrap_peers: Vec<Multiaddr>,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			identity: Keypair::generate_ed25519(),
			topic: "gossamer".to_string(),
			listen_on: "/ip4/0.0.0.0/tcp/0".parse().unwrap(),
			bootstrap_peers: vec![],
		}
	}
}

#[derive(Debug, thiserror::Error)]
pub enum GossamerConfigError {
	#[error("Error building Gossamer: {0}")]
	BuildError(String),
}

impl Config {
	pub async fn build(self) -> Result<(GossamerTask, Gossamer), GossamerConfigError> {
		let peer_id = PeerId::from(self.identity.public());

		// ---- GOSSIPSUB ----
		let gossipsub_config = gossipsub::Config::default();

		let mut gossipsub = gossipsub::Behaviour::new(
			MessageAuthenticity::Signed(self.identity.clone()),
			gossipsub_config,
		)
		.map_err(|e| GossamerConfigError::BuildError(e.to_string()))?;

		let topic = IdentTopic::new(self.topic);
		gossipsub
			.subscribe(&topic)
			.map_err(|e| GossamerConfigError::BuildError(e.to_string()))?;

		// ---- KADEMLIA ----
		let store = MemoryStore::new(peer_id);
		let kad = kad::Behaviour::new(peer_id, store);

		let behaviour = GossamerBehaviour { gossipsub, kad };

		let mut swarm = libp2p::SwarmBuilder::with_existing_identity(self.identity)
			.with_async_std()
			.with_tcp(tcp::Config::default(), noise::Config::new, yamux::Config::default)
			.map_err(|e| GossamerConfigError::BuildError(e.to_string()))?
			.with_dns()
			.await
			.map_err(|e| GossamerConfigError::BuildError(e.to_string()))?
			.with_behaviour(|_| behaviour)
			.map_err(|e| GossamerConfigError::BuildError(e.to_string()))?
			.build();

		// Listen on local port
		swarm
			.listen_on(self.listen_on)
			.map_err(|e| GossamerConfigError::BuildError(e.to_string()))?;

		// Bootstrap with the provided peers
		for peer in self.bootstrap_peers {
			swarm
				.dial(peer.clone())
				.map_err(|e| GossamerConfigError::BuildError(e.to_string()))?;

			// Extract peer id from multiaddr
			if let Some(Protocol::P2p(peer_id)) = peer.iter().last() {
				swarm.behaviour_mut().kad.add_address(&peer_id, peer);
			}
		}

		// Subscribe to the topic
		swarm
			.behaviour_mut()
			.gossipsub
			.subscribe(&topic)
			.map_err(|e| GossamerConfigError::BuildError(e.to_string()))?;

		// Allocate the channels
		let (sender_into_gossamer, receiver_into_gossamer) = unbounded_channel();
		let (sender_from_gossamer, receiver_from_gossamer) = unbounded_channel();

		Ok((
			GossamerTask {
				sender_into_gossamer,
				receiver_from_gossamer,
				topic_hash: topic.hash(),
				swarm,
			},
			Gossamer { receiver_into_gossamer, sender_from_gossamer },
		))
	}
}

pub struct GossamerTask {
	sender_into_gossamer: UnboundedSender<Vec<u8>>,
	receiver_from_gossamer: UnboundedReceiver<Vec<u8>>,
	topic_hash: TopicHash,
	swarm: Swarm<GossamerBehaviour>,
}

#[derive(Debug, thiserror::Error)]
pub enum GossamerTaskError {
	#[error("Error relaying message to Gossamer from swarm: {0}")]
	RelayToGossamerError(#[from] tokio::sync::mpsc::error::SendError<Vec<u8>>),
	#[error("Error broadcasting message: {0}")]
	BroadcastError(String),
	#[error("The broadcast receiver is disconnected")]
	BroadcastReceiverDisconnected,
	#[error("The swarm stream is disconnected")]
	SwarmStreamDisconnected,
}

impl Future for GossamerTask {
	type Output = Result<(), GossamerTaskError>;

	fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		// Broadcast messages to the swarm.
		// Drain the receiver_from_gossamer while there are messages to broadcast.

		// Ingest messages from the swarm.
		loop {
			let mut progressed = false;
			let topic_hash = self.topic_hash.clone();

			// 1. Poll outbound channel
			match Pin::new(&mut self.receiver_from_gossamer).poll_recv(cx) {
				Poll::Ready(Some(msg)) => {
					self.swarm
						.behaviour_mut()
						.gossipsub
						.publish(topic_hash, msg)
						.map_err(|e| GossamerTaskError::BroadcastError(e.to_string()))?;
					progressed = true;
				}
				Poll::Ready(None) => {
					return Poll::Ready(Err(GossamerTaskError::BroadcastReceiverDisconnected))
				}
				Poll::Pending => {}
			}

			// Drain while there are messages to receive.
			match Pin::new(&mut self.swarm).poll_next(cx) {
				Poll::Ready(Some(SwarmEvent::Behaviour(GossamerBehaviourEvent::Gossipsub(
					gossipsub::Event::Message { message, .. },
				)))) => {
					if let Err(e) = self.sender_into_gossamer.send(message.data) {
						return Poll::Ready(Err(GossamerTaskError::RelayToGossamerError(e)));
					}
					progressed = true;
				}

				Poll::Ready(Some(_)) => continue,

				Poll::Ready(None) => {
					return Poll::Ready(Err(GossamerTaskError::SwarmStreamDisconnected))
				}

				Poll::Pending => {}
			}

			if !progressed {
				return Poll::Pending;
			}
		}
	}
}

pub struct Gossamer {
	receiver_into_gossamer: UnboundedReceiver<Vec<u8>>,
	sender_from_gossamer: UnboundedSender<Vec<u8>>,
}

#[derive(Debug, thiserror::Error)]
pub enum GossamerMessageError {
	#[error("Error serializing message: {0:?}")]
	SerializeError((String, Vec<u8>)),
	#[error("Error deserializing message: {0:?}")]
	DeserializeError((String, Vec<u8>)),
	#[error("Error sending message from Gossamer to the swarm: {0}")]
	RelayToSwarmError(#[from] tokio::sync::mpsc::error::SendError<Vec<u8>>),
	#[error("Error receiving message from the swarm: {0}")]
	ReceiveFromSwarmError(#[from] tokio::sync::mpsc::error::TryRecvError),
}

pub trait GossamerMessage: Sized {
	fn to_goassamer_bytes(&self) -> Result<Vec<u8>, GossamerMessageError>;
	fn from_gossamer_bytes(bytes: Vec<u8>) -> Result<Self, GossamerMessageError>;
}

impl Gossamer {
	pub fn send_message<M: GossamerMessage>(
		&mut self,
		message: M,
	) -> Result<(), GossamerMessageError> {
		let bytes = message.to_goassamer_bytes()?;
		self.sender_from_gossamer
			.send(bytes)
			.map_err(|e| GossamerMessageError::RelayToSwarmError(e))?;
		Ok(())
	}

	pub fn try_recv_message<M: GossamerMessage>(
		&mut self,
	) -> Result<Option<M>, GossamerMessageError> {
		match self.receiver_into_gossamer.try_recv() {
			Ok(bytes) => {
				let message = GossamerMessage::from_gossamer_bytes(bytes)?;
				Ok(Some(message))
			}
			Err(TryRecvError::Empty) => Ok(None),
			Err(TryRecvError::Disconnected) => {
				Err(GossamerMessageError::ReceiveFromSwarmError(TryRecvError::Disconnected))
			}
		}
	}
}
