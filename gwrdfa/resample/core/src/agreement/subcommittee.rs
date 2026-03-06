//! Subcommittee semantics for evaluating agreement progress.

use super::Condition;
use parabyzantine::NoOp;

/// A set of voters/participants that can evaluate partial evidence and produce
/// an agreement condition for a value.
pub trait Subcommittee<Value: Eq + 'static>: Eq {
	/// Evaluates partial observations and returns consensus state.
	fn condition<'a>(
		&'a self,
		partials: impl Iterator<Item = (&'a Self, &'a Value)> + 'a,
	) -> Condition<Value>;
}

/// A [Subcommittee] for the [NoOp] struct.
impl<T: Eq + 'static> Subcommittee<T> for NoOp {
	fn condition<'a>(
		&'a self,
		_partials: impl Iterator<Item = (&'a NoOp, &'a T)> + 'a,
	) -> Condition<T> {
		Condition::InProgress
	}
}

