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

#[cfg(test)]
pub mod test {
	// use std::collections::HashSet;

	pub struct TestSampler {}
}
