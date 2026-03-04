use super::Condition;
use parabyzantine::NoOp;

pub trait Subcommittee<Value: Eq + 'static>: Eq {
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

