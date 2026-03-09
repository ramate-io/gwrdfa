use super::{Id, Message, PublicKey, Transaction};
use serde::{Deserialize, Serialize};

/// The index of a block in the system.
///
/// The system groups transactions into blocks which are indexed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Index(u64);

impl Index {
	pub fn new(index: u64) -> Self {
		Self(index)
	}

	pub fn get(&self) -> u64 {
		self.0
	}
}

/// The block itself which is a set of transactions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Block(Vec<Message<Transaction>>);

impl Block {
	pub fn new(transactions: impl Into<Vec<Message<Transaction>>>) -> Self {
		Self(transactions.into())
	}

	pub fn transactions(&self) -> &[Message<Transaction>] {
		&self.0
	}
}

/// The header of the block which references the transactions in the block.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockHeader(Vec<Id>);

impl BlockHeader {
	pub fn new() -> Self {
		Self(Vec::new())
	}

	pub fn ids(&self) -> &[Id] {
		&self.0
	}

	pub fn add_id(&mut self, id: Id) {
		self.0.push(id);
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
pub struct Value {
	block: BlockHeader,
	state_root: StateRoot,
	join_set: JoinSet,
}

impl Value {
	pub fn new(block: BlockHeader, state_root: StateRoot, join_set: JoinSet) -> Self {
		Self { block, state_root, join_set }
	}

	pub fn block(&self) -> &BlockHeader {
		&self.block
	}

	pub fn state_root(&self) -> &StateRoot {
		&self.state_root
	}

	pub fn join_set(&self) -> &JoinSet {
		&self.join_set
	}
}

/// The certificate for a block.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Certificate {
	index: Index,
	value: Value,
}

impl Certificate {
	pub fn new(index: Index, value: Value) -> Self {
		Self { index, value }
	}

	pub fn index(&self) -> &Index {
		&self.index
	}

	pub fn value(&self) -> &Value {
		&self.value
	}
}
