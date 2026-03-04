use crate::agreement::std::{CommitteeRef, Round, Vote};
use crate::agreement::Subcommittee;
use gwrdfa_container::{Component, ContainerGiving, Delta, DeltasContainer};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CertificateContainer<I: Eq, V: Eq + 'static, S: Subcommittee<V>> {
	pub index: Component<Round<I>>,
	pub value: Component<Vote<V>>,
	pub subcommittee: Component<CommitteeRef<S>>,
}

impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerGiving<Round<I>>
	for CertificateContainer<I, V, S>
{
	fn as_component(&self) -> Component<&Round<I>> {
		self.index.as_ref()
	}
}

impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerGiving<Vote<V>>
	for CertificateContainer<I, V, S>
{
	fn as_component(&self) -> Component<&Vote<V>> {
		self.value.as_ref()
	}
}

impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerGiving<CommitteeRef<S>>
	for CertificateContainer<I, V, S>
{
	fn as_component(&self) -> Component<&CommitteeRef<S>> {
		self.subcommittee.as_ref()
	}
}

pub struct CertificateDelta<I: Eq, V: Eq + 'static, S: Subcommittee<V>> {
	pub index: Delta<Round<I>>,
	pub value: Delta<Vote<V>>,
	pub subcommittee: Delta<CommitteeRef<S>>,
}

impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> DeltasContainer<CertificateContainer<I, V, S>>
	for CertificateDelta<I, V, S>
{
	fn apply_deltas(self, container: &mut CertificateContainer<I, V, S>) {
		self.index.apply(&mut container.index);
		self.value.apply(&mut container.value);
		self.subcommittee.apply(&mut container.subcommittee);
	}

	fn into_container(self) -> CertificateContainer<I, V, S> {
		CertificateContainer {
			index: self.index.into_component(),
			value: self.value.into_component(),
			subcommittee: self.subcommittee.into_component(),
		}
	}
}
