use super::ByzantineRequirement;
use crate::Transition;
use gwrdfa_resample::agreement::Condition;

impl Transition {
	/// Aggregates exact transition proposals with majority-style condition logic.
	pub fn aggregate<'a>(
		proposals: impl Iterator<Item = &'a Transition>,
		requirement: ByzantineRequirement,
	) -> Condition<Transition> {
		requirement.aggregate_majority(proposals)
	}
}
