use super::ByzantineRequirement;
use crate::Transition;
use gwrdfa_resample::agreement::Condition;
use std::collections::BTreeMap;

impl Transition {
	/// Aggregates exact transition proposals with majority-style condition logic.
	pub fn aggregate<'a>(
		proposals: impl Iterator<Item = &'a Transition>,
		requirement: ByzantineRequirement,
	) -> Condition<Transition> {
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
