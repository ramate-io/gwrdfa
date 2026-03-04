use crate::agreement::std::{Index, Subcom, Value};
use crate::agreement::Subcommittee;
use gwrdfa_container::{Component, ContainerGiving, Delta, DeltasContainer};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CertificateContainer<I: Eq, V: Eq + 'static, S: Subcommittee<V>> {
	pub index: Component<Index<I>>,
	pub value: Component<Value<V>>,
	pub subcommittee: Component<Subcom<S>>,
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

pub struct CertificateDelta<I: Eq, V: Eq + 'static, S: Subcommittee<V>> {
	pub index: Delta<Index<I>>,
	pub value: Delta<Value<V>>,
	pub subcommittee: Delta<Subcom<S>>,
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
