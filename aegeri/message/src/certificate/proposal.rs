mod availability;
mod block_header;
mod confirmation;
mod transition;

pub use availability::Availability;
pub use block_header::BlockHeader;
pub use confirmation::Confirmation;

use super::{Index, Transition};
use gwrdfa_resample::agreement::Condition;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::AegeriSubcommittee;

use gwrdfa_resample::agreement::std::join_set_committee::GivesJoinSet;

/// Stage-level quorum requirement for proposal aggregation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ByzantineRequirement {
	pub total_voters: usize,
	pub quorum: usize,
}

impl ByzantineRequirement {
	pub fn byzantine_quorum(total_voters: usize) -> Self {
		let quorum = ((total_voters * 2) / 3) + 1;
		Self { total_voters, quorum }
	}

	pub fn reaches_quorum(&self, votes: usize) -> bool {
		votes >= self.quorum
	}

	/// Majority-style condition evaluator shared by exact-value stages.
	pub fn aggregate_majority<'a, T: Clone + Ord + 'a>(
		&self,
		proposals: impl Iterator<Item = &'a T>,
	) -> Condition<T> {
		let mut votes = BTreeMap::new();
		let mut observed_votes = 0usize;
		for proposal in proposals {
			observed_votes += 1;
			*votes.entry(proposal.clone()).or_insert(0usize) += 1;
		}

		let mut max_votes = 0usize;
		for (proposal, count) in votes {
			max_votes = max_votes.max(count);
			if self.reaches_quorum(count) {
				return Condition::Consensus(proposal);
			}
		}

		let remaining = self.total_voters.saturating_sub(observed_votes);
		if remaining < self.quorum.saturating_sub(max_votes) {
			Condition::Hung
		} else {
			Condition::InProgress
		}
	}
}

/// Layered proposal family for certificates.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl GivesJoinSet<AegeriSubcommittee> for Proposal {
	fn joiners_and_leavers(
		&self,
	) -> Option<(impl Iterator<Item = AegeriSubcommittee>, impl Iterator<Item = AegeriSubcommittee>)>
	{
		match self {
			Proposal::Transition(value) => {
				let joiners = value.join_set().joiners().iter().map(|public_key| {
					AegeriSubcommittee::new(Index::Unassigned)
						.with_members(std::iter::once(public_key.clone()))
				});
				let leavers = value.join_set().leavers().iter().map(|public_key| {
					AegeriSubcommittee::new(Index::Unassigned)
						.with_members(std::iter::once(public_key.clone()))
				});
				Some((joiners, leavers))
			}
			_ => None,
		}
	}
}

impl Proposal {
	pub fn genesis() -> Self {
		Proposal::Availability(Availability::genesis())
	}

	/// Aggregates proposals for the given stage index.
	///
	/// Proposals that do not match the index stage are ignored.
	pub fn consensus_condition_for_index<'a>(
		index: &Index,
		proposals: impl Iterator<Item = &'a Proposal>,
		requirement: ByzantineRequirement,
	) -> Condition<Proposal> {
		match index {
			Index::Availability(_) => Availability::consensus_condition(
				proposals.filter_map(|proposal| match proposal {
					Proposal::Availability(value) => Some(value),
					_ => None,
				}),
				requirement,
			)
			.map(Proposal::Availability),
			Index::Confirmation(_) => Confirmation::consensus_condition(
				proposals.filter_map(|proposal| match proposal {
					Proposal::Confirmation(value) => Some(value),
					_ => None,
				}),
				requirement,
			)
			.map(Proposal::Confirmation),
			Index::Block(_) => BlockHeader::consensus_condition(
				proposals.filter_map(|proposal| match proposal {
					Proposal::BlockHeader(value) => Some(value),
					_ => None,
				}),
				requirement,
			)
			.map(Proposal::BlockHeader),
			Index::Transition(_) => Transition::consensus_condition(
				proposals.filter_map(|proposal| match proposal {
					Proposal::Transition(value) => Some(value),
					_ => None,
				}),
				requirement,
			)
			.map(Proposal::Transition),
			Index::Unassigned => Condition::InProgress,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::super::IndexValue;
	use super::*;
	use crate::{JoinSet, StateRoot, TransactionSet};

	#[test]
	fn test_aggregate_for_index_filters_to_matching_stage() {
		let availability = Proposal::Availability(Availability::new());
		let transition = Proposal::Transition(Transition::new(
			TransactionSet::new(),
			StateRoot::new(Vec::new()),
			JoinSet::new(),
		));
		let condition = Proposal::consensus_condition_for_index(
			&Index::Transition(IndexValue(0)),
			[&availability, &transition].into_iter(),
			ByzantineRequirement { total_voters: 2, quorum: 1 },
		);
		assert_eq!(condition, Condition::Consensus(transition));
	}
}
