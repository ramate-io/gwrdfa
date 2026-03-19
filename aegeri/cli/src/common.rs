use aegeri_message::PublicKey;
use clap::Parser;
use gossamer::Multiaddr;
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
pub struct PeerList {
	/// The list of public keys to join the cluster.
	#[clap(long)]
	pub peers: Vec<PublicKey>,
	/// The multiaddress to join the cluster on.
	#[clap(long)]
	pub multiaddr: Vec<Multiaddr>,
}

impl PeerList {
	pub fn new() -> Self {
		Self { peers: Vec::new(), multiaddr: Vec::new() }
	}

	pub fn add_peer(&mut self, peer: PublicKey) {
		self.peers.push(peer);
	}

	pub fn add_multiaddr(&mut self, multiaddr: Multiaddr) {
		self.multiaddr.push(multiaddr);
	}
}
