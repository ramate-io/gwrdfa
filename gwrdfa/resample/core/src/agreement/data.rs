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
	use parabyzantine::agreement::{
		Agreement, ParabyzantineAgreementData, ParabyzantineAgreementDataSpec,
	};
}
