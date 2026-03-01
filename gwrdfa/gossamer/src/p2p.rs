use libp2p::{
	gossipsub,
	kad::{self, store::MemoryStore},
	ping,
	swarm::NetworkBehaviour,
};

#[derive(NetworkBehaviour)]
pub struct GossamerBehaviour {
	pub gossipsub: gossipsub::Behaviour,
	pub kad: kad::Behaviour<MemoryStore>,
	pub ping: ping::Behaviour,
}
