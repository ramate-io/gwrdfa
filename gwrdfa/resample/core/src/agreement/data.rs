use super::ResampleAgreementSpec;
use parabyzantine::agreement::{ParabyzantineAgreementBinding, ParabyzantineAgreementSpec};

pub trait ResampleAgreementData<
	Binding: ParabyzantineAgreementBinding,
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
	fn index_subcommittee_agreement_query(&mut self) -> Spec::IndexSubcommitteeAgreementQuery;

	/// ResampleAgreement data must be able to produce a [Spec::CertificateQuery]
	fn certificate_query(
		&mut self,
		index: &(
			<Binding::Spec as ParabyzantineAgreementSpec>::AgreementEntity,
			Spec::IndexSubcommitteeAgreementBundle,
		),
	) -> Spec::CertificateQuery;
}
