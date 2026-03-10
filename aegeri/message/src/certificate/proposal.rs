use super::{TransactionSet, Transition};
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

/// Exact block-content proposal from one replica.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct BlockProposal(TransactionSet);

impl BlockProposal {
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
	/// Block proposals finalize exact transaction ids before transition agreement.
	BlockProposal(BlockProposal),
	/// Transition proposals finalize post-state commitment.
	Transition(Transition),
}
