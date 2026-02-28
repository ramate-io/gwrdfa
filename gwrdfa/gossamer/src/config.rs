use crate::{Gossamer, GossamerBehaviour, GossamerTask};
use libp2p::{
	gossipsub::{self, IdentTopic, MessageAuthenticity},
	identity::Keypair,
	kad::{self, store::MemoryStore},
	multiaddr::Protocol,
	noise, ping, tcp, yamux, Multiaddr, PeerId,
};
use tokio::sync::mpsc::unbounded_channel;
use tokio::sync::oneshot::{self, Receiver};

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
	#[error("Error receiving listen address from the Gossamer task: {0}")]
	ReceiveFromGossamerTaskError(#[from] tokio::sync::oneshot::error::RecvError),
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
	) -> Result<(GossamerTask<Entity>, Receiver<Multiaddr>, Gossamer<Entity>), GossamerConfigError>
	{
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

		let ping = ping::Behaviour::new(ping::Config::default());

		let behaviour = GossamerBehaviour { gossipsub, kad, ping };

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

		// Allocate the listen address sender
		let (listen_addr_sender, listen_addr_receiver) = oneshot::channel();

		Ok((
			GossamerTask {
				message_into_gossamer_sender,
				entity_message_from_gossamer_receiver,
				entity_into_gossamer_sender,
				topic_hash: topic.hash(),
				swarm,
				listen_addr_sender: Some(listen_addr_sender),
			},
			listen_addr_receiver,
			Gossamer {
				message_into_gossamer_receiver,
				entity_message_from_gossamer_sender,
				entity_into_gossamer_receiver,
			},
		))
	}
}
