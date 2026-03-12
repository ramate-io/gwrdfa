use crate::agreement::std::{Index, Subcom, Value};
use crate::agreement::Subcommittee;
use crate::ForResample;
use gwrdfa_container::{Component, ContainerGiving, ContainerStores, Delta, DeltasContainer};

/// Certificate-side container used by agreement ingestion queries.
///
/// `for_resample` is an explicit semantic marker to scope which certificates are
/// considered by the resample pipeline.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CertificateContainer<I: Eq, V: Eq + 'static, S: Subcommittee<V>> {
	pub for_resample: Component<ForResample>,
	pub index: Component<Index<I>>,
	pub value: Component<Value<V>>,
	pub subcommittee: Component<Subcom<S>>,
}

impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerGiving<ForResample>
	for CertificateContainer<I, V, S>
{
	fn as_component(&self) -> Component<&ForResample> {
		self.for_resample.as_ref()
	}
}

impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerGiving<Index<I>>
	for CertificateContainer<I, V, S>
{
	fn as_component(&self) -> Component<&Index<I>> {
		self.index.as_ref()
	}
}

impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerGiving<Value<V>>
	for CertificateContainer<I, V, S>
{
	fn as_component(&self) -> Component<&Value<V>> {
		self.value.as_ref()
	}
}

impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerGiving<Subcom<S>>
	for CertificateContainer<I, V, S>
{
	fn as_component(&self) -> Component<&Subcom<S>> {
		self.subcommittee.as_ref()
	}
}

/// Delta representation for [`CertificateContainer`].
///
/// The marker field is also delta-tracked so routing semantics can be added or
/// removed without rebuilding entities.
pub struct CertificateDelta<I: Eq, V: Eq + 'static, S: Subcommittee<V>> {
	pub for_resample: Delta<ForResample>,
	pub index: Delta<Index<I>>,
	pub value: Delta<Value<V>>,
	pub subcommittee: Delta<Subcom<S>>,
}

impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> DeltasContainer<CertificateContainer<I, V, S>>
	for CertificateDelta<I, V, S>
{
	fn apply_deltas(self, container: &mut CertificateContainer<I, V, S>) {
		self.for_resample.apply(&mut container.for_resample);
		self.index.apply(&mut container.index);
		self.value.apply(&mut container.value);
		self.subcommittee.apply(&mut container.subcommittee);
	}

	fn into_container(self) -> CertificateContainer<I, V, S> {
		CertificateContainer {
			for_resample: self.for_resample.into_component(),
			index: self.index.into_component(),
			value: self.value.into_component(),
			subcommittee: self.subcommittee.into_component(),
		}
	}
}

impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerStores<ForResample>
	for CertificateDelta<I, V, S>
{
	fn from_data(data: ForResample) -> Self {
		Self {
			for_resample: Delta::Modified(data),
			index: Delta::Unchanged,
			value: Delta::Unchanged,
			subcommittee: Delta::Unchanged,
		}
	}

	fn from_removed_data() -> Self {
		Self {
			for_resample: Delta::Removed,
			index: Delta::Unchanged,
			value: Delta::Unchanged,
			subcommittee: Delta::Unchanged,
		}
	}

	fn update_with_data(&mut self, data: ForResample) {
		self.for_resample = Delta::Modified(data);
	}

	fn remove_from_container(&mut self) {
		self.for_resample = Delta::Removed;
	}
}

impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerStores<Index<I>>
	for CertificateDelta<I, V, S>
{
	fn from_data(data: Index<I>) -> Self {
		Self {
			for_resample: Delta::Unchanged,
			index: Delta::Modified(data),
			value: Delta::Unchanged,
			subcommittee: Delta::Unchanged,
		}
	}

	fn from_removed_data() -> Self {
		Self {
			for_resample: Delta::Unchanged,
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
	for CertificateDelta<I, V, S>
{
	fn from_data(data: Value<V>) -> Self {
		Self {
			for_resample: Delta::Unchanged,
			index: Delta::Unchanged,
			value: Delta::Modified(data),
			subcommittee: Delta::Unchanged,
		}
	}

	fn from_removed_data() -> Self {
		Self {
			for_resample: Delta::Unchanged,
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

impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerStores<Subcom<S>>
	for CertificateDelta<I, V, S>
{
	fn from_data(data: Subcom<S>) -> Self {
		Self {
			for_resample: Delta::Unchanged,
			index: Delta::Unchanged,
			value: Delta::Unchanged,
			subcommittee: Delta::Modified(data),
		}
	}

	fn from_removed_data() -> Self {
		Self {
			for_resample: Delta::Unchanged,
			index: Delta::Unchanged,
			value: Delta::Unchanged,
			subcommittee: Delta::Removed,
		}
	}

	fn update_with_data(&mut self, data: Subcom<S>) {
		self.subcommittee = Delta::Modified(data);
	}

	fn remove_from_container(&mut self) {
		self.subcommittee = Delta::Removed;
	}
}
