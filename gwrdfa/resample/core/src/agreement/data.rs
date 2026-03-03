use super::{ResampleAgreementSpec, ResampleAgreementStorage};
use parabyzantine::agreement::{ParabyzantineAgreementDataBinding, ParabyzantineAgreementDataSpec};
use parabyzantine::{NoOp, NoOpData};

pub trait ResampleAgreementData<
	Binding: ParabyzantineAgreementDataBinding,
	Spec: ResampleAgreementSpec<Binding>,
>: Sized where
	<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementDraftBuffer:
		ResampleAgreementStorage<
			<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementEntity,
			Spec::Index,
			Spec::Subcommittee,
			Spec::Value,
		>,
{
	/// A [ResampleAgreement] data must be able to provide a [CertificateSet]
	fn certificate_set(&self) -> &Spec::CertificateSet;

	/// A [ResampleAgreement] data must be able to provide a mutable [CertificateSet]
	fn certificate_set_mut(&mut self) -> &mut Spec::CertificateSet;

	/// ResampleAgreement data must be able to provide a [Sampler]
	fn sampler(&self) -> &Spec::Sampler;

	/// ResampleAgreement data must be able to provide a mutable [Sampler]
	fn sampler_mut(&mut self) -> &mut Spec::Sampler;

	/// ResampleAgreement data must be able to prduce a [Spec::IndexSubcommitteeAgreementQuery]
	fn index_subcommittee_agreement_query_plan(
		&mut self,
	) -> Spec::IndexSubcommitteeAgreementQueryPlan;

	/// ResampleAgreement data must be able to produce a [Spec::CertificateQuery]
	fn certificate_query_plan(&mut self, index: &Spec::Index) -> Spec::CertificateQueryPlan;
}

impl<Binding: ParabyzantineAgreementDataBinding> ResampleAgreementData<Binding, NoOp> for NoOpData
where
	<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementDraftBuffer:
		ResampleAgreementStorage<
			<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementEntity,
			NoOp,
			NoOp,
			NoOp,
		>,
{
	fn certificate_set(&self) -> &NoOp {
		&self.no_op
	}
	fn certificate_set_mut(&mut self) -> &mut NoOp {
		&mut self.no_op
	}

	fn sampler(&self) -> &NoOp {
		&self.no_op
	}
	fn sampler_mut(&mut self) -> &mut NoOp {
		&mut self.no_op
	}
	fn index_subcommittee_agreement_query_plan(&mut self) -> NoOp {
		NoOp
	}
	fn certificate_query_plan(&mut self, _index: &NoOp) -> NoOp {
		NoOp
	}
}

#[cfg(test)]
pub mod test {
	use super::*;
	use crate::{agreement::Subcommittee, Resample};
	use core::marker::PhantomData;
	use gwrdfa_container::{
		Component, ContainerEntity, ContainerEntityBuffer, ContainerEntityDraftBuffer,
		ContainerGiving, Delta, DeltasContainer,
	};
	use parabyzantine::agreement::{Agreement, ParabyzantineAgreementDataSpec};

	#[derive(Debug, Clone, Default, PartialEq, Eq)]
	pub struct Index<T: Eq>(pub T);

	#[derive(Debug, Clone, Default, PartialEq, Eq)]
	pub struct Value<T: Eq + 'static>(pub T);

	#[derive(Debug, Clone, Default, PartialEq, Eq)]
	pub struct Sub<T: Eq>(pub T);

	pub struct TestResampleCertificateContainer<I: Eq, V: Eq + 'static, S: Subcommittee<V>> {
		pub index: Component<Index<I>>,
		pub value: Component<Value<V>>,
		pub subcommittee: Component<Sub<S>>,
	}

	impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerGiving<Index<I>>
		for TestResampleCertificateContainer<I, V, S>
	{
		fn as_component(&self) -> Component<&Index<I>> {
			self.index.as_ref()
		}
	}

	impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerGiving<Value<V>>
		for TestResampleCertificateContainer<I, V, S>
	{
		fn as_component(&self) -> Component<&Value<V>> {
			self.value.as_ref()
		}
	}

	impl<I: Eq, V: Eq + 'static, S: Subcommittee<V>> ContainerGiving<Sub<S>>
		for TestResampleCertificateContainer<I, V, S>
	{
		fn as_component(&self) -> Component<&Sub<S>> {
			self.subcommittee.as_ref()
		}
	}

	pub struct TestResampleCertificateDelta<I: Eq, V: Eq + 'static, S: Subcommittee<V>> {
		pub index: Delta<Index<I>>,
		pub value: Delta<Value<V>>,
		pub subcommittee: Delta<Sub<S>>,
	}

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

	pub struct TestResampleAgreementContainer<
		Index: Eq,
		Value: Eq + 'static,
		Sub: Subcommittee<Value>,
	> {
		pub agreement: Component<Agreement>,
		pub resample: Component<Resample>,
		pub index: Component<Index>,
		pub value: Component<Value>,
		pub subcommittee: Component<Sub>,
	}

	pub struct TestResampleAgreementDelta<Index: Eq, Value: Eq + 'static, Sub: Subcommittee<Value>> {
		pub agreement: Delta<Agreement>,
		pub resample: Delta<Resample>,
		pub index: Delta<Index>,
		pub value: Delta<Value>,
		pub subcommittee: Delta<Sub>,
	}

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

	pub struct TestResampleParabyzantineSpec<
		Index: Eq,
		Value: Eq + 'static,
		Sub: Subcommittee<Value>,
	> {
		__marker: PhantomData<(Index, Value, Sub)>,
	}

	impl<Index: Eq, Value: Eq + 'static, Sub: Subcommittee<Value>> ParabyzantineAgreementDataSpec
		for TestResampleParabyzantineSpec<Index, Value, Sub>
	{
		type CertificateEntity = NoOp;
		type CertificateBuffer = NoOp;
		type CertificateDraftBuffer = NoOp;
		type AgreementEntity = ContainerEntity;
		type AgreementBuffer =
			ContainerEntityBuffer<TestResampleAgreementContainer<Index, Value, Sub>>;
		type AgreementDraftBuffer =
			ContainerEntityDraftBuffer<TestResampleAgreementDelta<Index, Value, Sub>>;
	}
}
