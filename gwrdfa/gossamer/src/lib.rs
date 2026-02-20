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
pub struct GossamerConfig {
	pub identity: Keypair,
	pub topic: String,
	pub listen_on: Multiaddr,
	pub bootstrap_peers: Vec<Multiaddr>,
}

impl Default for GossamerConfig {
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

impl GossamerConfig {
	pub fn with_identity(mut self, identity: Keypair) -> Self {
		self.identity = identity;
		self
	}

	pub fn with_topic(mut self, topic: String) -> Self {
		self.topic = topic;
		self
	}

	pub fn with_listen_on(mut self, listen_on: Multiaddr) -> Self {
		self.listen_on = listen_on;
		self
	}

	pub fn with_bootstrap_peers(mut self, bootstrap_peers: Vec<Multiaddr>) -> Self {
		self.bootstrap_peers = bootstrap_peers;
		self
	}

	pub async fn build<Entity: Send + Sync + 'static>(
		self,
	) -> Result<(GossamerTask<Entity>, Gossamer<Entity>), GossamerConfigError> {
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
		let (message_into_gossamer_sender, message_into_gossamer_receiver) = unbounded_channel();
		let (entity_message_from_gossamer_sender, entity_message_from_gossamer_receiver) =
			unbounded_channel();
		let (entity_into_gossamer_sender, entity_into_gossamer_receiver) = unbounded_channel();

		Ok((
			GossamerTask {
				message_into_gossamer_sender,
				entity_message_from_gossamer_receiver,
				entity_into_gossamer_sender,
				topic_hash: topic.hash(),
				swarm,
			},
			Gossamer {
				message_into_gossamer_receiver,
				entity_message_from_gossamer_sender,
				entity_into_gossamer_receiver,
			},
		))
	}
}

pub struct GossamerTask<Entity: Send + Sync + 'static> {
	message_into_gossamer_sender: UnboundedSender<Vec<u8>>,
	entity_message_from_gossamer_receiver: UnboundedReceiver<(Entity, Vec<u8>)>,
	entity_into_gossamer_sender: UnboundedSender<Entity>,
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

impl<Entity: Send + Sync + 'static> Future for GossamerTask<Entity> {
	type Output = Result<(), GossamerTaskError>;

	fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		// Broadcast messages to the swarm.
		// Drain the receiver_from_gossamer while there are messages to broadcast.

		// Ingest messages from the swarm.
		loop {
			let mut progressed = false;
			let topic_hash = self.topic_hash.clone();

			// 1. Poll outbound channel
			match Pin::new(&mut self.entity_message_from_gossamer_receiver).poll_recv(cx) {
				Poll::Ready(Some((entity, msg))) => {
					self.swarm
						.behaviour_mut()
						.gossipsub
						.publish(topic_hash, msg)
						.map_err(|e| GossamerTaskError::BroadcastError(e.to_string()))?;
					self.entity_into_gossamer_sender
						.send(entity)
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
					if let Err(e) = self.message_into_gossamer_sender.send(message.data) {
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

pub struct Gossamer<Entity: Send + Sync> {
	message_into_gossamer_receiver: UnboundedReceiver<Vec<u8>>,
	entity_message_from_gossamer_sender: UnboundedSender<(Entity, Vec<u8>)>,
	entity_into_gossamer_receiver: UnboundedReceiver<Entity>,
}

#[derive(Debug, thiserror::Error)]
pub enum GossamerMessageError {
	#[error("Error serializing message: {0:?}")]
	SerializeError((String, Vec<u8>)),
	#[error("Error deserializing message: {0:?}")]
	DeserializeError((String, Vec<u8>)),
	#[error("Error sending message from Gossamer to the swarm")]
	RelayToSwarm,
	#[error("Error receiving message from the swarm: {0}")]
	ReceiveFromSwarmError(#[from] tokio::sync::mpsc::error::TryRecvError),
}

pub trait GossamerMessage: Sized {
	fn to_goassamer_bytes(&self) -> Result<Vec<u8>, GossamerMessageError>;
	fn from_gossamer_bytes(bytes: Vec<u8>) -> Result<Self, GossamerMessageError>;
}

impl<Entity: Send + Sync + 'static> Gossamer<Entity> {
	/// Spawns a Gossamer task in a tokio runtime.
	pub async fn spawn_tokio(
		config: GossamerConfig,
	) -> Result<Gossamer<Entity>, GossamerConfigError> {
		let (gossamer_task, gossamer) = config.build().await?;
		tokio::spawn(gossamer_task);
		Ok(gossamer)
	}

	/// Produces a mock instance, mostly used for testing purposes.
	pub fn mock() -> (
		Self,
		UnboundedSender<Vec<u8>>,
		UnboundedReceiver<(Entity, Vec<u8>)>,
		UnboundedSender<Entity>,
	) {
		let (message_into_gossamer_sender, message_into_gossamer_receiver) = unbounded_channel();
		let (entity_message_from_gossamer_sender, entity_message_from_gossamer_receiver) =
			unbounded_channel();
		let (entity_into_gossamer_sender, entity_into_gossamer_receiver) = unbounded_channel();

		(
			Self {
				message_into_gossamer_receiver,
				entity_message_from_gossamer_sender,
				entity_into_gossamer_receiver,
			},
			message_into_gossamer_sender,
			entity_message_from_gossamer_receiver,
			entity_into_gossamer_sender,
		)
	}

	pub fn try_recv_message<M: GossamerMessage>(
		&mut self,
	) -> Result<Option<M>, GossamerMessageError> {
		match self.message_into_gossamer_receiver.try_recv() {
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

	pub fn send_message<M: GossamerMessage>(
		&mut self,
		entity: Entity,
		message: M,
	) -> Result<(), GossamerMessageError> {
		let bytes = message.to_goassamer_bytes()?;
		self.entity_message_from_gossamer_sender
			.send((entity, bytes))
			.map_err(|_| GossamerMessageError::RelayToSwarm)?;
		Ok(())
	}

	pub fn try_recv_confirmation(&mut self) -> Result<Option<Entity>, GossamerMessageError> {
		match self.entity_into_gossamer_receiver.try_recv() {
			Ok(entity) => Ok(Some(entity)),
			Err(TryRecvError::Empty) => Ok(None),
			Err(TryRecvError::Disconnected) => {
				Err(GossamerMessageError::ReceiveFromSwarmError(TryRecvError::Disconnected))
			}
		}
	}
}

/// Marks that an entity has flowed in through Gossamer In
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct In;

/// Marks that an entity has been dispatched for Gossamer out
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Out;

/// Marks an entity that has been pushed in flight for Gossamer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InFlight;

/// Marks that an entity has flowed through Gossamer broadcast.
///
/// Often these are entities that are ready to be removed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Broadcast;

#[cfg(test)]
mod tests {
	use super::*;

	#[derive(Debug, Clone, PartialEq, Eq, Hash)]
	pub struct TestMessage(Vec<u8>);

	impl TestMessage {
		pub fn new(data: Vec<u8>) -> Self {
			Self(data)
		}
	}

	impl GossamerMessage for TestMessage {
		fn to_goassamer_bytes(&self) -> Result<Vec<u8>, GossamerMessageError> {
			Ok(self.0.clone())
		}
		fn from_gossamer_bytes(bytes: Vec<u8>) -> Result<Self, GossamerMessageError> {
			Ok(TestMessage(bytes))
		}
	}

	#[tokio::test]
	async fn test_mock_flow() -> Result<(), anyhow::Error> {
		// Build the mock Gossamer instance.
		let (
			mut gossamer,
			message_into_gossamer_sender,
			mut entity_message_from_gossamer_receiver,
			entity_into_gossamer_sender,
		) = Gossamer::<u32>::mock();

		// Send a message into the Gossamer instance.
		let message1 = TestMessage::new(vec![1, 2, 3]);
		let message1_bytes = message1.to_goassamer_bytes()?;
		message_into_gossamer_sender.send(message1_bytes)?;

		// Receive the message from the Gossamer instance.
		let message = gossamer.try_recv_message::<TestMessage>()?;
		assert_eq!(message, Some(TestMessage(vec![1, 2, 3])));

		// Send an out message from the Gossamer instance.
		let entity1 = 1;
		let message2 = TestMessage::new(vec![4, 5, 6]);
		let message2_bytes = message2.to_goassamer_bytes()?;
		gossamer.send_message(entity1, message2)?;
		let (entity, message) = entity_message_from_gossamer_receiver
			.recv()
			.await
			.ok_or(anyhow::anyhow!("Failed to receive message"))?;
		assert_eq!(entity, entity1);
		assert_eq!(message, message2_bytes);

		// Send a confirmation back into the Gossamer instance.
		let entity1 = 1;
		entity_into_gossamer_sender.send(entity1)?;
		let confirmation = gossamer.try_recv_confirmation()?;
		assert_eq!(confirmation, Some(entity1));

		Ok(())
	}
}
