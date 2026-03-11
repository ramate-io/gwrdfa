use super::ByzantineRequirement;
use crate::Transition;
use gwrdfa_resample::agreement::Condition;

impl Transition {
	/// Aggregates exact transition proposals with majority-style condition logic.
	pub fn consensus_condition<'a>(
		proposals: impl Iterator<Item = &'a Transition>,
		requirement: ByzantineRequirement,
	) -> Condition<Transition> {
		requirement.aggregate_majority(proposals)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{JoinSet, StateRoot, TransactionSet};

	#[test]
	fn test_consensus_condition_consensus_and_in_progress() {
		let t = Transition::new(TransactionSet::new(), StateRoot::new(Vec::new()), JoinSet::new());
		let consensus = Transition::consensus_condition(
			[&t, &t].into_iter(),
			ByzantineRequirement { total_voters: 3, quorum: 2 },
		);
		assert!(matches!(consensus, Condition::Consensus(_)));

		let in_progress = Transition::consensus_condition(
			[&t].into_iter(),
			ByzantineRequirement { total_voters: 3, quorum: 2 },
		);
		assert!(matches!(in_progress, Condition::InProgress));
	}
}
