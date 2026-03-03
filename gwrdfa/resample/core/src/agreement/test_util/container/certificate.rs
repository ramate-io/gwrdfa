use crate::agreement::test_util::{Index, Sub, Value};
use crate::agreement::Subcommittee;
use gwrdfa_container::{Component, ContainerGiving, Delta, DeltasContainer};

/// A container for a certificate.
pub struct TestResampleCertificateContainer<I: Eq, V: Eq + 'static, S: Subcommittee<V>> {
	pub index: Component<Index<I>>,
	pub value: Component<Value<V>>,
	pub subcommittee: Component<Sub<S>>,
}

/// A [ContainerGiving] implementation for [Index<I>].
impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerGiving<Index<I>>
	for TestResampleCertificateContainer<I, V, S>
{
	fn as_component(&self) -> Component<&Index<I>> {
		self.index.as_ref()
	}
}

/// A [ContainerGiving] implementation for [Value<V>].
impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerGiving<Value<V>>
	for TestResampleCertificateContainer<I, V, S>
{
	fn as_component(&self) -> Component<&Value<V>> {
		self.value.as_ref()
	}
}

/// A [ContainerGiving] implementation for [Sub<S>].
impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerGiving<Sub<S>>
	for TestResampleCertificateContainer<I, V, S>
{
	fn as_component(&self) -> Component<&Sub<S>> {
		self.subcommittee.as_ref()
	}
}

/// We want the delta buffer to be able to hold the deltas for the index, value, and subcommittee.
pub struct TestResampleCertificateDelta<I: Eq, V: Eq + 'static, S: Subcommittee<V>> {
	pub index: Delta<Index<I>>,
	pub value: Delta<Value<V>>,
	pub subcommittee: Delta<Sub<S>>,
}

/// A [DeltasContainer] implementation for [TestResampleCertificateContainer<I, V, S>].
impl<Index: Eq, Value: Eq + 'static, Sub: Subcommittee<Value>>
	DeltasContainer<TestResampleCertificateContainer<Index, Value, Sub>>
	for TestResampleCertificateDelta<Index, Value, Sub>
{
	fn apply_deltas(self, container: &mut TestResampleCertificateContainer<Index, Value, Sub>) {
		self.index.apply(&mut container.index);
		self.value.apply(&mut container.value);
		self.subcommittee.apply(&mut container.subcommittee);
	}

	fn into_container(self) -> TestResampleCertificateContainer<Index, Value, Sub> {
		TestResampleCertificateContainer {
			index: self.index.into_component(),
			value: self.value.into_component(),
			subcommittee: self.subcommittee.into_component(),
		}
	}
}
