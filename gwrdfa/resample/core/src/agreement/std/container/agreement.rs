use crate::agreement::std::{CommitteeRef, Round, Vote};
use crate::agreement::{Resample, Subcommittee};
use gwrdfa_container::{Component, ContainerGiving, ContainerStores, Delta, DeltasContainer};
use parabyzantine::agreement::Agreement;

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AgreementContainer<I: Eq, V: Eq + 'static, S: Subcommittee<V>> {
	pub agreement: Component<Agreement>,
	pub resample: Component<Resample>,
	pub index: Component<Round<I>>,
	pub value: Component<Vote<V>>,
	pub subcommittee: Component<CommitteeRef<S>>,
}

impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerGiving<Agreement>
	for AgreementContainer<I, V, S>
{
	fn as_component(&self) -> Component<&Agreement> {
		self.agreement.as_ref()
	}
}

impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerGiving<Resample>
	for AgreementContainer<I, V, S>
{
	fn as_component(&self) -> Component<&Resample> {
		self.resample.as_ref()
	}
}

impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerGiving<Round<I>>
	for AgreementContainer<I, V, S>
{
	fn as_component(&self) -> Component<&Round<I>> {
		self.index.as_ref()
	}
}

impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerGiving<Vote<V>>
	for AgreementContainer<I, V, S>
{
	fn as_component(&self) -> Component<&Vote<V>> {
		self.value.as_ref()
	}
}

impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerGiving<CommitteeRef<S>>
	for AgreementContainer<I, V, S>
{
	fn as_component(&self) -> Component<&CommitteeRef<S>> {
		self.subcommittee.as_ref()
	}
}

pub struct AgreementDelta<I: Eq, V: Eq + 'static, S: Subcommittee<V>> {
	pub agreement: Delta<Agreement>,
	pub resample: Delta<Resample>,
	pub index: Delta<Round<I>>,
	pub value: Delta<Vote<V>>,
	pub subcommittee: Delta<CommitteeRef<S>>,
}

impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerStores<Agreement> for AgreementDelta<I, V, S> {
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

impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerStores<Resample> for AgreementDelta<I, V, S> {
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

impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerStores<Round<I>> for AgreementDelta<I, V, S> {
	fn from_data(data: Round<I>) -> Self {
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

	fn update_with_data(&mut self, data: Round<I>) {
		self.index = Delta::Modified(data);
	}

	fn remove_from_container(&mut self) {
		self.index = Delta::Removed;
	}
}

impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerStores<Vote<V>> for AgreementDelta<I, V, S> {
	fn from_data(data: Vote<V>) -> Self {
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

	fn update_with_data(&mut self, data: Vote<V>) {
		self.value = Delta::Modified(data);
	}

	fn remove_from_container(&mut self) {
		self.value = Delta::Removed;
	}
}

impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerStores<CommitteeRef<S>>
	for AgreementDelta<I, V, S>
{
	fn from_data(data: CommitteeRef<S>) -> Self {
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

	fn update_with_data(&mut self, data: CommitteeRef<S>) {
		self.subcommittee = Delta::Modified(data);
	}

	fn remove_from_container(&mut self) {
		self.subcommittee = Delta::Removed;
	}
}

impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> DeltasContainer<AgreementContainer<I, V, S>>
	for AgreementDelta<I, V, S>
{
	fn apply_deltas(self, container: &mut AgreementContainer<I, V, S>) {
		self.agreement.apply(&mut container.agreement);
		self.resample.apply(&mut container.resample);
		self.index.apply(&mut container.index);
		self.value.apply(&mut container.value);
		self.subcommittee.apply(&mut container.subcommittee);
	}

	fn into_container(self) -> AgreementContainer<I, V, S> {
		AgreementContainer {
			agreement: self.agreement.into_component(),
			resample: self.resample.into_component(),
			index: self.index.into_component(),
			value: self.value.into_component(),
			subcommittee: self.subcommittee.into_component(),
		}
	}
}
