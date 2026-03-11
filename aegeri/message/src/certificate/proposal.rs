use super::Transition;
use crate::TransactionSet;
use serde::{Deserialize, Serialize};

/// Availability proposal from one replica.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Availability(TransactionSet);

impl Availability {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn transactions(&self) -> &TransactionSet {
		&self.0
	}
}

/// Confirmation proposal from one replica.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Confirmation(TransactionSet);

impl Confirmation {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn transactions(&self) -> &TransactionSet {
		&self.0
	}
}

/// Exact block-header proposal from one replica.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct BlockHeader(TransactionSet);

impl BlockHeader {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn transactions(&self) -> &TransactionSet {
		&self.0
	}
}

/// Layered proposal family for certificates.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Proposal {
	/// Availability values are merged to maximize candidate visibility.
	Availability(Availability),
	/// Confirmation values narrow candidates toward quorum-observed content.
	Confirmation(Confirmation),
	/// Block-header proposals finalize exact transaction ids before transition agreement.
	BlockHeader(BlockHeader),
	/// Transition proposals finalize post-state commitment.
	Transition(Transition),
}
