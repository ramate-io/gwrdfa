use super::ByzantineRequirement;
use crate::TransactionSet;
use gwrdfa_resample::agreement::Condition;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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
		let mut votes = BTreeMap::new();
		let mut observed_votes = 0usize;
		for proposal in proposals {
			observed_votes += 1;
			*votes.entry(proposal.clone()).or_insert(0usize) += 1;
		}

		let mut max_votes = 0usize;
		for (proposal, count) in votes {
			max_votes = max_votes.max(count);
			if count >= requirement.quorum {
				return Condition::Consensus(proposal);
			}
		}

		let remaining = requirement.total_voters.saturating_sub(observed_votes);
		if remaining < requirement.quorum.saturating_sub(max_votes) {
			Condition::Hung
		} else {
			Condition::InProgress
		}
	}
}
