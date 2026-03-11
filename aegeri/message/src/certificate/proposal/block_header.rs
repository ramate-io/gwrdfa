use super::ByzantineRequirement;
use crate::TransactionSet;
use gwrdfa_resample::agreement::Condition;
use serde::{Deserialize, Serialize};

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

	/// Aggregates exact block-header proposals with majority-style condition logic.
	pub fn aggregate<'a>(
		proposals: impl Iterator<Item = &'a BlockHeader>,
		requirement: ByzantineRequirement,
	) -> Condition<BlockHeader> {
		requirement.aggregate_majority(proposals)
	}
}
