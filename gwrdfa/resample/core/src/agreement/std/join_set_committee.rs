use crate::agreement::std::NextRound;
use crate::agreement::{Condition, Sampler, Subcommittee};
use std::collections::HashSet;
use std::hash::Hash;
use std::marker::PhantomData;

pub trait TakesJoinSet<V: Eq + Hash + Clone + 'static>: Subcommittee<V> + Clone {
	fn update_with_join_set(
		&mut self,
		joiners: impl Iterator<Item = Self>,
		leavers: impl Iterator<Item = Self>,
	);
}

pub trait GivesJoinSet<S: Subcommittee<Self> + Clone>: Eq + Hash + Clone + 'static {
	fn joiners_and_leavers(&self) -> (impl Iterator<Item = S>, impl Iterator<Item = S>);
}

#[derive(Debug, Clone, Default)]
pub struct JoinSetCommittee<V: Eq + Hash + Clone + 'static, S: Subcommittee<V> + Clone> {
	members: HashSet<S>,
	__marker: PhantomData<V>,
}

impl<V: Eq + Hash + Clone + 'static, S: Subcommittee<V> + Clone> JoinSetCommittee<V, S> {
	pub fn new() -> Self {
		Self { members: HashSet::new(), __marker: PhantomData }
	}
}

impl<Index: Eq + NextRound, Value: GivesJoinSet<Sub>, Sub: TakesJoinSet<Value>>
	Sampler<Index, Value, Sub> for JoinSetCommittee<Value, Sub>
{
	fn elect_subcommittee_from_condition(
		&mut self,
		index: &Index,
		subcommittee: &Sub,
		value: &Condition<Value>,
	) -> Option<(Index, Sub)> {
		match value {
			Condition::Consensus(value) => {
				let mut new_subcommittee = subcommittee.clone();

				let (joiners, leavers) = value.joiners_and_leavers();
				new_subcommittee.update_with_join_set(joiners, leavers);

				index.next().map(|index| (index, subcommittee.clone()))
			}
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
			sampler.elect_subcommittee_from_condition(
				&index,
				&voters,
				&Condition::<u32>::InProgress
			),
			None
		);
	}
}
