use crate::agreement::test_util::{Index, Sub, Value};
use crate::agreement::{Resample, Subcommittee};
use gwrdfa_container::{Component, ContainerGiving, Delta, DeltasContainer};
use parabyzantine::agreement::Agreement;

/// A container for an agreement.
pub struct TestResampleAgreementContainer<I: Eq, V: Eq + 'static, S: Subcommittee<V>> {
	pub agreement: Component<Agreement>,
	pub resample: Component<Resample>,
	pub index: Component<Index<I>>,
	pub value: Component<Value<V>>,
	pub subcommittee: Component<Sub<S>>,
}

impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerGiving<Index<I>>
	for TestResampleAgreementContainer<I, V, S>
{
	fn as_component(&self) -> Component<&Index<I>> {
		self.index.as_ref()
	}
}

impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerGiving<Value<V>>
	for TestResampleAgreementContainer<I, V, S>
{
	fn as_component(&self) -> Component<&Value<V>> {
		self.value.as_ref()
	}
}

impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerGiving<Sub<S>>
	for TestResampleAgreementContainer<I, V, S>
{
	fn as_component(&self) -> Component<&Sub<S>> {
		self.subcommittee.as_ref()
	}
}
/// A [DeltasContainer] implementation for [TestResampleAgreementContainer<I, V, S>].
pub struct TestResampleAgreementDelta<I: Eq, V: Eq + 'static, S: Subcommittee<V>> {
	pub agreement: Delta<Agreement>,
	pub resample: Delta<Resample>,
	pub index: Delta<Index<I>>,
	pub value: Delta<Value<V>>,
	pub subcommittee: Delta<Sub<S>>,
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
