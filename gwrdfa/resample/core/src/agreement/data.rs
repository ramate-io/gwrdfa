use super::ResampleAgreementSpec;
use parabyzantine::agreement::{ParabyzantineAgreementDataBinding, ParabyzantineAgreementDataSpec};
use parabyzantine::NoOp;
use parabyzantine::NoOpData;

pub trait ResampleAgreementData<
	Binding: ParabyzantineAgreementDataBinding,
	Spec: ResampleAgreementSpec<Binding>,
>: Sized
{
	/// A [ResampleAgreement] data must be able to provide a [CertificateSet]
	fn certificate_set(&self) -> &Spec::CertificateSet;

	/// A [ResampleAgreement] data must be able to provide a mutable [CertificateSet]
	fn certificate_set_mut(&mut self) -> &mut Spec::CertificateSet;

	/// ResampleAgreement data must be able to provide a [Sampler]
	fn sampler(&self) -> &Spec::Sampler;

	/// ResampleAgreement data must be able to provide a mutable [Sampler]
	fn sampler_mut(&mut self) -> &mut Spec::Sampler;

	/// ResampleAgreement data must be able to produce a [Spec::ResampleAgreementConsensusUpdate]
	fn resample_agreement_consensus_update(&self) -> &Spec::ResampleAgreementConsensusUpdate;

	/// ResampleAgreement data must be able to produce a mutable [Spec::ResampleAgreementConsensusUpdate]
	fn resample_agreement_consensus_update_mut(
		&mut self,
	) -> &mut Spec::ResampleAgreementConsensusUpdate;

	/// ResampleAgreement data must be able to prduce a [Spec::IndexSubcommitteeAgreementQuery]
	fn index_subcommittee_agreement_query_plan(
		&mut self,
	) -> Spec::IndexSubcommitteeAgreementQueryPlan;

	/// ResampleAgreement data must be able to produce a [Spec::CertificateQuery]
	fn certificate_query_plan(
		&mut self,
		index: &(
			<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementEntity,
			Spec::IndexSubcommitteeAgreementQueryData,
		),
	) -> Spec::CertificateQueryPlan;
}

impl ResampleAgreementData<NoOp, NoOp> for NoOpData {
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
	fn resample_agreement_consensus_update(&self) -> &NoOp {
		&self.no_op
	}
	fn resample_agreement_consensus_update_mut(&mut self) -> &mut NoOp {
		&mut self.no_op
	}
	fn index_subcommittee_agreement_query_plan(&mut self) -> NoOp {
		NoOp
	}

	fn certificate_query_plan(
		&mut self,
		_index: &(
			<NoOp as ParabyzantineAgreementDataSpec>::AgreementEntity,
			<NoOp as ResampleAgreementSpec<NoOp>>::IndexSubcommitteeAgreementQueryData,
		),
	) -> <NoOp as ResampleAgreementSpec<NoOp>>::CertificateQueryPlan {
		NoOp
	}
}
