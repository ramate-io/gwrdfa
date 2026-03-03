use crate::agreement::{Resample, Subcommittee};
use gwrdfa_container::{Component, Delta, DeltasContainer};
use parabyzantine::agreement::Agreement;

/// A container for an agreement.
pub struct TestResampleAgreementContainer<Index: Eq, Value: Eq + 'static, Sub: Subcommittee<Value>>
{
	pub agreement: Component<Agreement>,
	pub resample: Component<Resample>,
	pub index: Component<Index>,
	pub value: Component<Value>,
	pub subcommittee: Component<Sub>,
}

/// A [DeltasContainer] implementation for [TestResampleAgreementContainer<I, V, S>].
pub struct TestResampleAgreementDelta<Index: Eq, Value: Eq + 'static, Sub: Subcommittee<Value>> {
	pub agreement: Delta<Agreement>,
	pub resample: Delta<Resample>,
	pub index: Delta<Index>,
	pub value: Delta<Value>,
	pub subcommittee: Delta<Sub>,
}

/// A [DeltasContainer] implementation for [TestResampleAgreementContainer<I, V, S>].
impl<Index: Eq, Value: Eq + 'static, Sub: Subcommittee<Value>>
	DeltasContainer<TestResampleAgreementContainer<Index, Value, Sub>>
	for TestResampleAgreementDelta<Index, Value, Sub>
{
	fn apply_deltas(self, container: &mut TestResampleAgreementContainer<Index, Value, Sub>) {
		self.agreement.apply(&mut container.agreement);
		self.resample.apply(&mut container.resample);
		self.index.apply(&mut container.index);
		self.value.apply(&mut container.value);
		self.subcommittee.apply(&mut container.subcommittee);
	}

	fn into_container(self) -> TestResampleAgreementContainer<Index, Value, Sub> {
		TestResampleAgreementContainer {
			agreement: self.agreement.into_component(),
			resample: self.resample.into_component(),
			index: self.index.into_component(),
			value: self.value.into_component(),
			subcommittee: self.subcommittee.into_component(),
		}
	}
}
