use crate::{PublicKey, TransactionSet};
use serde::{Deserialize, Serialize};

/// State root produced by execution.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateRoot(Vec<u8>);

impl StateRoot {
	pub fn new(root: impl Into<Vec<u8>>) -> Self {
		Self(root.into())
	}

	pub fn as_bytes(&self) -> &[u8] {
		&self.0
	}
}

/// Joiners included in a transition.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct JoinSet {
	joiners: Vec<PublicKey>,
	leavers: Vec<PublicKey>,
}

impl JoinSet {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn add_joiner(&mut self, joiner: PublicKey) {
		self.joiners.push(joiner);
	}

	pub fn add_leaver(&mut self, leaver: PublicKey) {
		self.leavers.push(leaver);
	}

	pub fn joiners(&self) -> &Vec<PublicKey> {
		&self.joiners
	}

	pub fn leavers(&self) -> &Vec<PublicKey> {
		&self.leavers
	}
}

/// Transition proposal: exact post-state commitment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Transition {
	block: TransactionSet,
	state_root: StateRoot,
	join_set: JoinSet,
}

impl Transition {
	pub fn new(block: TransactionSet, state_root: StateRoot, join_set: JoinSet) -> Self {
		Self { block, state_root, join_set }
	}

	pub fn genesis() -> Self {
		Self::new(TransactionSet::new(), StateRoot::new(Vec::new()), JoinSet::new())
	}

	pub fn block(&self) -> &TransactionSet {
		&self.block
	}

	pub fn state_root(&self) -> &StateRoot {
		&self.state_root
	}

	pub fn join_set(&self) -> &JoinSet {
		&self.join_set
	}
}
