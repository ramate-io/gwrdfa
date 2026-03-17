use aegeri_message::{AegeriSubcommittee, Index, PublicKey};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Bootstrap {
	bootstrapped: bool,
	peer_count_required: usize,
	bootstrap_peers: HashSet<PublicKey>,
	counts: HashMap<(Index, AegeriSubcommittee), HashSet<PublicKey>>,
}

impl Bootstrap {
	pub fn new() -> Self {
		// By default, we assume the node has already bootstrapped.
		Self {
			bootstrapped: true,
			peer_count_required: 0,
			bootstrap_peers: HashSet::new(),
			counts: HashMap::new(),
		}
	}

	pub fn with_bootstrapped(mut self, has_bootstrapped: bool) -> Self {
		self.bootstrapped = has_bootstrapped;
		self
	}

	pub fn has_bootstrapped(&self) -> bool {
		self.bootstrapped
	}

	pub fn contains_peer(&self, peer: &PublicKey) -> bool {
		self.bootstrap_peers.contains(peer)
	}

	pub fn add_peer(&mut self, peer: PublicKey) {
		self.bootstrap_peers.insert(peer);
	}

	pub fn remove_peer(&mut self, peer: PublicKey) {
		self.bootstrap_peers.remove(&peer);
	}

	pub fn with_bootstrap_peers(
		mut self,
		bootstrap_peers: impl IntoIterator<Item = PublicKey>,
	) -> Self {
		self.bootstrap_peers.extend(bootstrap_peers);
		self
	}
}
