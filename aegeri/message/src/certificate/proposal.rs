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

/// Stage-level quorum requirement for proposal aggregation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ByzantineRequirement {
	pub total_voters: usize,
	pub quorum: usize,
}

impl ByzantineRequirement {
	pub fn byzantine_quorum(total_voters: usize) -> Self {
		let quorum = (total_voters * 2).div_ceil(3) + 1;
		Self { total_voters, quorum }
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

impl Proposal {
	/// Aggregates proposals for the given stage index.
	///
	/// Proposals that do not match the index stage are ignored.
	pub fn aggregate_for_index<'a>(
		index: &Index,
		proposals: impl Iterator<Item = &'a Proposal>,
		requirement: ByzantineRequirement,
	) -> Condition<Proposal> {
		match index {
			Index::Availability(_) => Availability::aggregate(
				proposals.filter_map(|proposal| match proposal {
					Proposal::Availability(value) => Some(value),
					_ => None,
				}),
				requirement,
			)
			.map(Proposal::Availability),
			Index::Confirmation(_) => Confirmation::aggregate(
				proposals.filter_map(|proposal| match proposal {
					Proposal::Confirmation(value) => Some(value),
					_ => None,
				}),
				requirement,
			)
			.map(Proposal::Confirmation),
			Index::Block(_) => BlockHeader::aggregate(
				proposals.filter_map(|proposal| match proposal {
					Proposal::BlockHeader(value) => Some(value),
					_ => None,
				}),
				requirement,
			)
			.map(Proposal::BlockHeader),
			Index::Transition(_) => Transition::aggregate(
				proposals.filter_map(|proposal| match proposal {
					Proposal::Transition(value) => Some(value),
					_ => None,
				}),
				requirement,
			)
			.map(Proposal::Transition),
		}
	}
}
