use super::Condition;

pub trait Subcommittee<Sender: Eq>: Eq {
	fn condition<'a, Value: 'a + Eq>(
		&'a self,
		partials: impl Iterator<Item = (&'a Self, &'a Value)> + 'a,
	) -> Condition<Value>;
}

pub trait IndexSubcommitteeAgreement<Index: Eq, Sender: Eq, Sub: Subcommittee<Sender>>: Eq {
	/// The index of the agreement.
	fn index(&self) -> Index;

	/// The subcommittee of the agreement.
	fn subcommittee(&self) -> Sub;
}
