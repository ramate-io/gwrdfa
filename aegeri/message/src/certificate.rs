use super::{Id, PublicKey, Transaction};
use serde::{Deserialize, Serialize};

/// The index of a block in the system.
///
/// The system groups transactions into blocks which are indexed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Index(u64);

/// The block itself which is a set of transactions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Block(Vec<Transaction>);

/// The header of the block which references the transactions in the block.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockHeader(Vec<Id>);

/// The state root produced by execution of the block.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct StateRoot(Vec<u8>);

/// The JoinSet for a block.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct JoinSet(Vec<PublicKey>);

/// The unified value of a certificate.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Value {
	block: BlockHeader,
	state_root: StateRoot,
	join_set: JoinSet,
}

/// The certificate for a block.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Certificate {
	index: Index,
	value: Value,
}
