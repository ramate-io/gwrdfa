use super::{Certificate, CertificateSet, ResampleAgreementStorage, Sampler, Subcommittee};
use parabyzantine::NoOp;
use parabyzantine::{
	agreement::{ParabyzantineAgreementDataBinding, ParabyzantineAgreementDataSpec},
	buffer::query::{QueryPlanlike, Querylike},
};

/// A [ResampleAgreementSpec] is a specification for ResampleAgreement consensus.
pub trait ResampleAgreementSpec<Binding: ParabyzantineAgreementDataBinding>: Sized
where
	<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementDraftBuffer:
		ResampleAgreementStorage<
			<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementEntity,
			Self::Index,
			Self::Subcommittee,
			Self::Value,
		>,
{
	/// The type of the index.
	/// The index must be clonable in order to be able to insert index subcommittee agreement facts.
	type Index: Clone + Eq;

	/// The type of the value.
	/// The value must be clonable in order to be able to insert value agreement facts.
	type Value: Clone + Eq + 'static;

	/// The type of the subcommittee.
	/// The subcommittee must be clonable in order to be able to insert subcommittee agreement facts.
	type Subcommittee: Subcommittee<Self::Value> + Clone;

	/// The bundle of the agreement in the buffer.
	type IndexSubcommitteeAgreementQueryData<'a>;

	/// The query for the index subcommittee agreement.
	type IndexSubcommitteeAgreementQuery<'a>: Querylike<
		<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementEntity,
		Item = Self::IndexSubcommitteeAgreementQueryData<'a>,
	>;

	/// The query plan for the index subcommittee agreement.
	type IndexSubcommitteeAgreementQueryPlan: for<'a> QueryPlanlike<
		<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementEntity,
		&'a <Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementBuffer,
		Query = Self::IndexSubcommitteeAgreementQuery<'a>,
	>;

	/// The bundle of the certificate in the buffer.
	type CertificateQueryData<'a>;

	/// The query for the certificate.
	type CertificateQuery<'a>: Querylike<
		<Binding::Spec as ParabyzantineAgreementDataSpec>::CertificateEntity,
		Item = Self::CertificateQueryData<'a>,
	>;

	/// The query plan for the certificate.
	type CertificateQueryPlan: for<'a> QueryPlanlike<
		<Binding::Spec as ParabyzantineAgreementDataSpec>::CertificateEntity,
		&'a <Binding::Spec as ParabyzantineAgreementDataSpec>::CertificateBuffer,
		Query = Self::CertificateQuery<'a>,
	>;

	/// The type of the certificate.
	type Certificate: Certificate<Self::Index, Self::Value, Self::Subcommittee>
		+ for<'a> From<(
			<Binding::Spec as ParabyzantineAgreementDataSpec>::CertificateEntity,
			Self::CertificateQueryData<'a>,
		)>;

	/// The type of the certificate set.
	type CertificateSet: CertificateSet<
		Self::Index,
		Self::Value,
		Self::Certificate,
		Self::Subcommittee,
	>;

	/// The type of the sampler.
	type Sampler: Sampler<Self::Index, Self::Value, Self::Subcommittee>;
}

impl ResampleAgreementSpec<NoOp> for NoOp {
	type Index = NoOp;
	type Value = NoOp;
	type Subcommittee = NoOp;
	type IndexSubcommitteeAgreementQueryData<'a> = NoOp;
	type IndexSubcommitteeAgreementQuery<'a> = NoOp;
	type IndexSubcommitteeAgreementQueryPlan = NoOp;
	type CertificateQueryData<'a> = NoOp;
	type CertificateQuery<'a> = NoOp;
	type CertificateQueryPlan = NoOp;
	type Certificate = NoOp;
	type CertificateSet = NoOp;
	type Sampler = NoOp;
}
