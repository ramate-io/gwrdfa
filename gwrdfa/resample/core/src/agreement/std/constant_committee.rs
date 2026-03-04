use crate::agreement::std::NextRound;
use crate::agreement::{Condition, Sampler, Subcommittee};

#[derive(Debug, Clone, Default)]
pub struct ConstantCommittee;

impl ConstantCommittee {
	pub fn new() -> Self {
		Self
	}
}

impl<Index: Eq + NextRound, Value: Eq + 'static, Sub: Subcommittee<Value> + Clone>
	Sampler<Index, Value, Sub> for ConstantCommittee
{
	fn elect_subcommittee_from_condition(
		&mut self,
		index: &Index,
		subcommittee: &Sub,
		value: &Condition<Value>,
	) -> Option<(Index, Sub)> {
		match value {
			Condition::Consensus(_) => index.next().map(|index| (index, subcommittee.clone())),
			Condition::Hung | Condition::InProgress => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::agreement::std::VoterSet;

	#[test]
	fn constant_committee_advances_on_consensus_only() {
		let mut sampler = ConstantCommittee;
		let index = 0;
		let mut voters = VoterSet::new();
		voters.add_member(1);
		voters.add_member(2);
		voters.add_member(3);

		assert_eq!(
			sampler.elect_subcommittee_from_condition(&index, &voters, &Condition::Consensus(0)),
			Some((1, voters.clone()))
		);
		assert_eq!(
			sampler.elect_subcommittee_from_condition(&index, &voters, &Condition::<u32>::Hung),
			None
		);
		assert_eq!(
			sampler.elect_subcommittee_from_condition(&index, &voters, &Condition::<u32>::InProgress),
			None
		);
	}
}
