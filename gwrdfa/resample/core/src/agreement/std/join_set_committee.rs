use crate::agreement::std::NextRound;
use crate::agreement::std::{Index, Subcom, Value};
use crate::agreement::{Condition, Sampler, Subcommittee};
use std::hash::Hash;

pub trait TakesJoinSet<I: Eq, V: Eq + Hash + Clone + 'static>: Subcommittee<V> + Clone {
	fn update_with_join_set(
		&mut self,
		index: &I,
		joiners: impl Iterator<Item = Self>,
		leavers: impl Iterator<Item = Self>,
	);
}

impl<I: Eq, S: TakesJoinSet<I, V> + 'static, V: Eq + Hash + Clone + 'static>
	TakesJoinSet<Index<I>, Value<V>> for Subcom<S>
{
	fn update_with_join_set(
		&mut self,
		index: &Index<I>,
		joiners: impl Iterator<Item = Self>,
		leavers: impl Iterator<Item = Self>,
	) {
		self.0.update_with_join_set(
			&index.0,
			joiners.map(|subcom| subcom.0),
			leavers.map(|subcom| subcom.0),
		);
	}
}

pub trait GivesJoinSet<S: Subcommittee<Self> + Clone>: Eq + Hash + Clone + 'static {
	fn joiners_and_leavers(&self) -> Option<(impl Iterator<Item = S>, impl Iterator<Item = S>)>;
}

impl<S: Subcommittee<V> + Clone + 'static, V: GivesJoinSet<S> + 'static> GivesJoinSet<Subcom<S>>
	for Value<V>
{
	fn joiners_and_leavers(
		&self,
	) -> Option<(impl Iterator<Item = Subcom<S>>, impl Iterator<Item = Subcom<S>>)> {
		self.0.joiners_and_leavers().map(|(joiners, leavers)| {
			(joiners.map(|subcom| Subcom::new(subcom)), leavers.map(|subcom| Subcom::new(subcom)))
		})
	}
}

#[derive(Debug, Clone, Default)]
pub struct JoinSetCommittee;

impl JoinSetCommittee {
	pub fn new() -> Self {
		Self
	}
}

impl<Index: Eq + NextRound, Value: GivesJoinSet<Sub>, Sub: TakesJoinSet<Index, Value>>
	Sampler<Index, Value, Sub> for JoinSetCommittee
{
	fn elect_subcommittee_from_condition(
		&mut self,
		index: &Index,
		subcommittee: &Sub,
		value: &Condition<Value>,
	) -> Option<(Index, Sub)> {
		match value {
			Condition::Consensus(value) => index.next().map(|index| {
				let mut new_subcommittee = subcommittee.clone();

				if let Some((joiners, leavers)) = value.joiners_and_leavers() {
					new_subcommittee.update_with_join_set(&index, joiners, leavers);
				}
				(index, new_subcommittee)
			}),
			Condition::Hung | Condition::InProgress => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use anyhow::Result;
	use std::collections::BTreeSet;

	#[derive(Debug, Clone, PartialEq, Eq, Hash)]
	struct TestValue {
		joiners: Vec<TestCommittee>,
		leavers: Vec<TestCommittee>,
	}

	impl GivesJoinSet<TestCommittee> for TestValue {
		fn joiners_and_leavers(
			&self,
		) -> Option<(impl Iterator<Item = TestCommittee>, impl Iterator<Item = TestCommittee>)> {
			Some((self.joiners.clone().into_iter(), self.leavers.clone().into_iter()))
		}
	}

	#[derive(Debug, Clone, PartialEq, Eq, Hash)]
	struct TestCommittee {
		members: BTreeSet<u32>,
	}

	impl TestCommittee {
		fn from_members(members: impl IntoIterator<Item = u32>) -> Self {
			Self { members: members.into_iter().collect() }
		}
	}

	impl Subcommittee<TestValue> for TestCommittee {
		fn condition<'a>(
			&'a self,
			_partials: impl Iterator<Item = (&'a Self, &'a TestValue)> + 'a,
		) -> Condition<TestValue> {
			Condition::InProgress
		}
	}

	impl<I: Eq> TakesJoinSet<I, TestValue> for TestCommittee {
		fn update_with_join_set(
			&mut self,
			_index: &I,
			joiners: impl Iterator<Item = Self>,
			leavers: impl Iterator<Item = Self>,
		) {
			for join in joiners {
				for member in join.members {
					self.members.insert(member);
				}
			}
			for leave in leavers {
				for member in leave.members {
					self.members.remove(&member);
				}
			}
		}
	}

	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	struct TerminalIndex;

	impl NextRound for TerminalIndex {
		fn next(&self) -> Option<Self> {
			None
		}
	}

	#[test]
	fn test_join_set_committee_advances_and_updates_subcommittee() -> Result<()> {
		let mut sampler = JoinSetCommittee::new();
		let index = 4u32;
		let subcommittee = TestCommittee::from_members([1, 2, 4]);
		let value = TestValue {
			joiners: vec![TestCommittee::from_members([3])],
			leavers: vec![TestCommittee::from_members([2])],
		};

		let result = sampler.elect_subcommittee_from_condition(
			&index,
			&subcommittee,
			&Condition::Consensus(value),
		);
		let (next_index, next_subcommittee) = match result {
			Some(value) => value,
			None => anyhow::bail!("expected next subcommittee on consensus"),
		};
		assert_eq!(next_index, 5);
		assert_eq!(next_subcommittee, TestCommittee::from_members([1, 3, 4]));
		Ok(())
	}

	#[test]
	fn test_join_set_committee_returns_none_on_hung_or_in_progress() {
		let mut sampler = JoinSetCommittee::new();
		let index = 9u32;
		let subcommittee = TestCommittee::from_members([7, 8]);
		assert_eq!(
			sampler.elect_subcommittee_from_condition(
				&index,
				&subcommittee,
				&Condition::<TestValue>::Hung
			),
			None
		);
		assert_eq!(
			sampler.elect_subcommittee_from_condition(
				&index,
				&subcommittee,
				&Condition::<TestValue>::InProgress
			),
			None
		);
	}

	#[test]
	fn test_join_set_committee_returns_none_when_no_next_index() {
		let mut sampler = JoinSetCommittee::new();
		let index = TerminalIndex;
		let subcommittee = TestCommittee::from_members([1]);
		let value = TestValue { joiners: vec![TestCommittee::from_members([2])], leavers: vec![] };
		assert_eq!(
			sampler.elect_subcommittee_from_condition(
				&index,
				&subcommittee,
				&Condition::Consensus(value)
			),
			None
		);
	}
}
