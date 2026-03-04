use super::{Condition, Subcommittee};
use parabyzantine::NoOp;

pub trait Sampler<Index: Eq, Value: Eq + 'static, Sub: Subcommittee<Value>>: Sized {
	/// Given a value and the subcommittee agreeement which gave that value,
	/// the sampler has the option to insert agreements into the buffer.
	///
	/// Note, that this is an offline rule set,
	/// so it does not depend on a consensus value itself.
	/// If you want to write a protocol which changes the rule set based on a consensus value,
	/// there are two major patterns:
	/// 1. Write your [Sampler] so that values themselves can effectively update the sampler to reflect the new rule set.
	/// 2. Write your [Sampler] so that it inserts agreements which defer the actual subcommittee election to a later stage, e.g., ParabyzantineTask.
	fn elect_subcommittee_from_condition(
		&mut self,
		index: &Index,
		subcommittee: &Sub,
		value: &Condition<Value>,
	) -> Option<(Index, Sub)>;
}

impl<Index: Eq, Value: Eq + 'static, Sub: Subcommittee<Value>> Sampler<Index, Value, Sub> for NoOp {
	fn elect_subcommittee_from_condition(
		&mut self,
		_index: &Index,
		_subcommittee: &Sub,
		_value: &Condition<Value>,
	) -> Option<(Index, Sub)> {
		None
	}
}

#[cfg(any(test, feature = "std"))]
pub mod test {
	use super::*;
	use crate::agreement::std::NextRound;

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
				Condition::Consensus(_value) => {
					index.next().map(|index| (index, subcommittee.clone()))
				}
				Condition::Hung => None,
				Condition::InProgress => None,
			}
		}
	}

	#[test]
	fn test_test_sampler() {
		use crate::agreement::subcommittee::test::Committee;

		let mut sampler = ConstantCommittee;
		let index = 0;
		let mut subcommittee = Committee::new();
		subcommittee.add_member(1);
		subcommittee.add_member(2);
		subcommittee.add_member(3);

		let value = Condition::Consensus(0);
		let next_subcommittee =
			sampler.elect_subcommittee_from_condition(&index, &subcommittee, &value);
		assert_eq!(next_subcommittee, Some((1, subcommittee.clone())));

		let value: Condition<u32> = Condition::Hung;
		let next_subcommittee =
			sampler.elect_subcommittee_from_condition(&index, &subcommittee, &value);
		assert_eq!(next_subcommittee, None);

		let value: Condition<u32> = Condition::InProgress;
		let next_subcommittee =
			sampler.elect_subcommittee_from_condition(&index, &subcommittee, &value);
		assert_eq!(next_subcommittee, None);
	}

	pub type TestSampler = ConstantCommittee;
}
