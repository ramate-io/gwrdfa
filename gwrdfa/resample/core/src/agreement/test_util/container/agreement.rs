use crate::agreement::test_util::{Index, Sub, Value};
use crate::agreement::{Resample, Subcommittee};
use gwrdfa_container::{Component, ContainerGiving, ContainerStores, Delta, DeltasContainer};
use parabyzantine::agreement::Agreement;

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A container for an agreement.
pub struct TestResampleAgreementContainer<I: Eq, V: Eq + 'static, S: Subcommittee<V>> {
	pub agreement: Component<Agreement>,
	pub resample: Component<Resample>,
	pub index: Component<Index<I>>,
	pub value: Component<Value<V>>,
	pub subcommittee: Component<Sub<S>>,
}

/// A [ContainerGiving] implementation for [Agreement].
impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerGiving<Agreement>
	for TestResampleAgreementContainer<I, V, S>
{
	fn as_component(&self) -> Component<&Agreement> {
		self.agreement.as_ref()
	}
}

/// A [ContainerGiving] implementation for [Resample].
impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerGiving<Resample>
	for TestResampleAgreementContainer<I, V, S>
{
	fn as_component(&self) -> Component<&Resample> {
		self.resample.as_ref()
	}
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

impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerStores<Agreement>
	for TestResampleAgreementDelta<I, V, S>
{
	fn from_data(data: Agreement) -> Self {
		Self {
			agreement: Delta::Modified(data),
			resample: Delta::Unchanged,
			index: Delta::Unchanged,
			value: Delta::Unchanged,
			subcommittee: Delta::Unchanged,
		}
	}

	fn from_removed_data() -> Self {
		Self {
			agreement: Delta::Removed,
			resample: Delta::Unchanged,
			index: Delta::Unchanged,
			value: Delta::Unchanged,
			subcommittee: Delta::Unchanged,
		}
	}

	fn update_with_data(&mut self, data: Agreement) {
		self.agreement = Delta::Modified(data);
	}

	fn remove_from_container(&mut self) {
		self.agreement = Delta::Removed;
	}
}

impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerStores<Resample>
	for TestResampleAgreementDelta<I, V, S>
{
	fn from_data(data: Resample) -> Self {
		Self {
			agreement: Delta::Unchanged,
			resample: Delta::Modified(data),
			index: Delta::Unchanged,
			value: Delta::Unchanged,
			subcommittee: Delta::Unchanged,
		}
	}

	fn from_removed_data() -> Self {
		Self {
			agreement: Delta::Unchanged,
			resample: Delta::Removed,
			index: Delta::Unchanged,
			value: Delta::Unchanged,
			subcommittee: Delta::Unchanged,
		}
	}

	fn update_with_data(&mut self, data: Resample) {
		self.resample = Delta::Modified(data);
	}

	fn remove_from_container(&mut self) {
		self.resample = Delta::Removed;
	}
}

impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerStores<Index<I>>
	for TestResampleAgreementDelta<I, V, S>
{
	fn from_data(data: Index<I>) -> Self {
		Self {
			agreement: Delta::Unchanged,
			resample: Delta::Unchanged,
			index: Delta::Modified(data),
			value: Delta::Unchanged,
			subcommittee: Delta::Unchanged,
		}
	}

	fn from_removed_data() -> Self {
		Self {
			agreement: Delta::Unchanged,
			resample: Delta::Unchanged,
			index: Delta::Removed,
			value: Delta::Unchanged,
			subcommittee: Delta::Unchanged,
		}
	}

	fn update_with_data(&mut self, data: Index<I>) {
		self.index = Delta::Modified(data);
	}

	fn remove_from_container(&mut self) {
		self.index = Delta::Removed;
	}
}

impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerStores<Value<V>>
	for TestResampleAgreementDelta<I, V, S>
{
	fn from_data(data: Value<V>) -> Self {
		Self {
			agreement: Delta::Unchanged,
			resample: Delta::Unchanged,
			index: Delta::Unchanged,
			value: Delta::Modified(data),
			subcommittee: Delta::Unchanged,
		}
	}

	fn from_removed_data() -> Self {
		Self {
			agreement: Delta::Unchanged,
			resample: Delta::Unchanged,
			index: Delta::Unchanged,
			value: Delta::Removed,
			subcommittee: Delta::Unchanged,
		}
	}

	fn update_with_data(&mut self, data: Value<V>) {
		self.value = Delta::Modified(data);
	}

	fn remove_from_container(&mut self) {
		self.value = Delta::Removed;
	}
}

impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerStores<Sub<S>>
	for TestResampleAgreementDelta<I, V, S>
{
	fn from_data(data: Sub<S>) -> Self {
		Self {
			agreement: Delta::Unchanged,
			resample: Delta::Unchanged,
			index: Delta::Unchanged,
			value: Delta::Unchanged,
			subcommittee: Delta::Modified(data),
		}
	}

	fn from_removed_data() -> Self {
		Self {
			agreement: Delta::Unchanged,
			resample: Delta::Unchanged,
			index: Delta::Unchanged,
			value: Delta::Unchanged,
			subcommittee: Delta::Removed,
		}
	}

	fn update_with_data(&mut self, data: Sub<S>) {
		self.subcommittee = Delta::Modified(data);
	}

	fn remove_from_container(&mut self) {
		self.subcommittee = Delta::Removed;
	}
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
