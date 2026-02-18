use futures::{
	task::{Context, Poll},
	Future, Stream, StreamExt,
};
use libp2p::{
	gossipsub::{self, IdentTopic, MessageAuthenticity},
	identity::Keypair,
	kad::{self, store::MemoryStore},
	multiaddr::Protocol,
	noise,
	swarm::{NetworkBehaviour, SwarmEvent},
	tcp, yamux, Multiaddr, PeerId, Swarm,
};
use std::pin::Pin;
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
		println!("Peer ID: {peer_id}");

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

		// Listen on random local port
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

		// Allocate the channel
		let (sender, receiver) = unbounded_channel();

		Ok((GossamerTask { sender, swarm }, Gossamer { receiver }))
	}
}

pub struct GossamerTask {
	sender: UnboundedSender<Vec<u8>>,
	swarm: Swarm<GossamerBehaviour>,
}

#[derive(Debug, thiserror::Error)]
pub enum GossamerTaskError {
	#[error("Error sending message: {0}")]
	SendError(#[from] tokio::sync::mpsc::error::SendError<Vec<u8>>),
}

/// A Gossamer task simply forwards bytes from the gossip to gossamer.
impl GossamerTask {
	pub async fn run(&mut self) -> Result<(), GossamerTaskError> {
		loop {
			match self.swarm.select_next_some().await {
				SwarmEvent::Behaviour(GossamerBehaviourEvent::Gossipsub(
					gossipsub::Event::Message { message, .. },
				)) => {
					self.sender.send(message.data).map_err(|e| GossamerTaskError::SendError(e))?;
				}
				_ => (),
			}
		}
	}
}

impl Future for GossamerTask {
	type Output = Result<(), GossamerTaskError>;

	fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		loop {
			match Pin::new(&mut self.swarm).poll_next(cx) {
				Poll::Ready(Some(SwarmEvent::Behaviour(GossamerBehaviourEvent::Gossipsub(
					gossipsub::Event::Message { message, .. },
				)))) => {
					if let Err(e) = self.sender.send(message.data) {
						return Poll::Ready(Err(GossamerTaskError::SendError(e)));
					}
				}

				Poll::Ready(Some(_)) => continue,

				Poll::Ready(None) => return Poll::Ready(Ok(())),

				Poll::Pending => return Poll::Pending,
			}
		}
	}
}

pub struct Gossamer {
	receiver: UnboundedReceiver<Vec<u8>>,
}
