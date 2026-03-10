use super::{Id, Message, PublicKey, Transaction};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// The index of a block in the system.
///
/// The system groups transactions into blocks which are indexed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Index {
	Availability(u64),
	Transition(u64),
}

impl Index {
	pub fn is_availability(&self) -> bool {
		matches!(self, Index::Availability(_))
	}

	pub fn is_transition(&self) -> bool {
		matches!(self, Index::Transition(_))
	}

	pub fn value(&self) -> u64 {
		match self {
			Index::Availability(index) => *index,
			Index::Transition(index) => *index,
		}
	}
}

/// The block itself which is a set of transactions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Block(BTreeSet<Message<Transaction>>);

impl Block {
	pub fn new(transactions: impl IntoIterator<Item = Message<Transaction>>) -> Self {
		Self(transactions.into_iter().collect())
	}

	pub fn transactions(&self) -> impl Iterator<Item = &Message<Transaction>> {
		self.0.iter()
	}
}

/// The header of the block which references the transactions in the block.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct TransactionSet(BTreeSet<Id>);

impl TransactionSet {
	pub fn new() -> Self {
		Self(BTreeSet::new())
	}

	pub fn ids(&self) -> &BTreeSet<Id> {
		&self.0
	}

	pub fn iter_ids(&self) -> impl Iterator<Item = &Id> {
		self.0.iter()
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn add_id(&mut self, id: Id) {
		self.0.insert(id);
	}

	pub fn intersection<'a>(&'a self, other: &'a TransactionSet) -> BTreeSet<&'a Id> {
		self.0.intersection(&other.0).collect()
	}

	pub fn intersect_all<'a>(iter: impl Iterator<Item = &'a TransactionSet>) -> BTreeSet<&'a Id> {
		iter.fold(BTreeSet::new(), |acc, set| {
			acc.intersection(&set.0.iter().collect::<BTreeSet<&Id>>()).cloned().collect()
		})
	}
}

/// The availability proposal from a given replica.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Availability(TransactionSet);

impl Availability {
	pub fn new() -> Self {
		Self(TransactionSet::new())
	}

	pub fn intersection<'a>(&'a self, other: &'a Availability) -> BTreeSet<&'a Id> {
		self.0.intersection(&other.0)
	}

	pub fn intersect_all<'a>(iter: impl Iterator<Item = &'a Availability>) -> BTreeSet<&'a Id> {
		iter.fold(BTreeSet::new(), |acc, set| {
			acc.intersection(&set.0 .0.iter().collect::<BTreeSet<&Id>>()).cloned().collect()
		})
	}
}

/// The state root produced by execution of the block.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct StateRoot(Vec<u8>);

impl StateRoot {
	pub fn new(root: impl Into<Vec<u8>>) -> Self {
		Self(root.into())
	}

	pub fn as_bytes(&self) -> &[u8] {
		&self.0
	}
}

/// The JoinSet for a block.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct JoinSet(Vec<PublicKey>);

impl JoinSet {
	pub fn new() -> Self {
		Self(Vec::new())
	}

	pub fn members(&self) -> &[PublicKey] {
		&self.0
	}

	pub fn add_member(&mut self, member: PublicKey) {
		self.0.push(member);
	}
}

/// The unified value of a certificate.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Transition {
	block: TransactionSet,
	state_root: StateRoot,
	join_set: JoinSet,
}

impl Transition {
	pub fn new(block: TransactionSet, state_root: StateRoot, join_set: JoinSet) -> Self {
		Self { block, state_root, join_set }
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialOrd, Ord)]
pub enum Proposal {
	/// The availability proposal for a given replica.
	Availability(Availability),
	/// The transition proposal for a given replica.
	Transition(Transition),
}

impl PartialEq for Proposal {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			// Availability proposals are always equal.
			// We only care that sufficient replicas submit a proposal.
			(Proposal::Availability(_), Proposal::Availability(_)) => true,
			// Transition proposals are equal if their inner values are equal.
			(Proposal::Transition(t), Proposal::Transition(u)) => t == u,
			_ => false,
		}
	}
}

impl Eq for Proposal {}

/// The certificate for a block.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Certificate {
	index: Index,
	value: Proposal,
}

impl Certificate {
	pub fn new(index: Index, value: Proposal) -> Self {
		Self { index, value }
	}

	pub fn index(&self) -> &Index {
		&self.index
	}

	pub fn value(&self) -> &Proposal {
		&self.value
	}
}
