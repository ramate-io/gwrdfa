use super::Condition;
use parabyzantine::NoOp;

pub trait Subcommittee<Sender: Eq>: Eq {
	fn condition<'a, Value: 'a + Eq>(
		&'a self,
		partials: impl Iterator<Item = (&'a Self, &'a Value)> + 'a,
	) -> Condition<Value>;
}

/// A [Subcommittee] for the [NoOp] struct.
impl Subcommittee<NoOp> for NoOp {
	fn condition<'a, Value: 'a + Eq>(
		&'a self,
		_partials: impl Iterator<Item = (&'a NoOp, &'a Value)> + 'a,
	) -> Condition<Value> {
		Condition::InProgress
	}
}

pub trait IndexSubcommitteeAgreement<Index: Eq, Sender: Eq, Sub: Subcommittee<Sender>>: Eq {
	/// The index of the agreement.
	fn index(&self) -> Index;

	/// The subcommittee of the agreement.
	fn subcommittee(&self) -> Sub;
}

/// A [IndexSubcommitteeAgreement] for the [NoOp] struct.
impl IndexSubcommitteeAgreement<NoOp, NoOp, NoOp> for NoOp {
	fn index(&self) -> NoOp {
		NoOp
	}
	fn subcommittee(&self) -> NoOp {
		NoOp
	}
}
