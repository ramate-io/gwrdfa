use super::Id;
use serde::{Deserialize, Serialize};

/// The index of a block in the system.
///
/// The system groups transactions into blocks which are indexed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index(u64);

/// The block itself which is a set of transaction ids.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block(Vec<Id>);

/// The state root produced by execution of the block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateRoot(Vec<u8>);

/// The unified value of a certificate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Value {
	block: Block,
	state_root: StateRoot,
}

/// The certificate for a block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Certificate {
	index: Index,
	value: Value,
}
