use super::{
	Certificate, CertificateSet, IndexSubcommitteeAgreement, ResampleAgreementConsensusUpdate,
	Sampler, Subcommittee,
};
use parabyzantine::NoOp;
use parabyzantine::{
	agreement::{ParabyzantineAgreementDataBinding, ParabyzantineAgreementDataSpec},
	buffer::query::{IntoQuery, Querylike},
};

/// A [ResampleAgreementSpec] is a specification for ResampleAgreement consensus.
pub trait ResampleAgreementSpec<Binding: ParabyzantineAgreementDataBinding>: Sized
where
	for<'a> &'a <Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementBuffer: IntoQuery<
		<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementEntity,
		Self::IndexSubcommitteeAgreementQueryPlan,
		Query = Self::IndexSubcommitteeAgreementQuery<'a>,
	>,
	for<'a> &'a <Binding::Spec as ParabyzantineAgreementDataSpec>::CertificateBuffer: IntoQuery<
		<Binding::Spec as ParabyzantineAgreementDataSpec>::CertificateEntity,
		Self::CertificateQueryPlan,
		Query = Self::CertificateQuery<'a>,
	>,
{
	/// The type of the index.
	type Index: Eq;

	/// The type of the value.
	type Value: Eq;

	/// The type of the sender of a certificate.
	type Sender: Eq;

	/// The type of the subcommittee.
	type Subcommittee: Subcommittee<Self::Sender>;

	/// The bundle of the agreement in the buffer.
	type IndexSubcommitteeAgreementQueryData<'a>;

	/// The query for the index subcommittee agreement.
	type IndexSubcommitteeAgreementQuery<'a>: Querylike<
		<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementEntity,
		Item = Self::IndexSubcommitteeAgreementQueryData<'a>,
	>;

	/// The query plan for the index subcommittee agreement.
	type IndexSubcommitteeAgreementQueryPlan;

	/// The type of the index subcommittee agreement.
	type IndexSubcommitteeAgreement: IndexSubcommitteeAgreement<Self::Index, Self::Sender, Self::Subcommittee>
		+ for<'a> From<&'a (
			<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementEntity,
			Self::IndexSubcommitteeAgreementQueryData<'a>,
		)>;

	/// The bundle of the certificate in the buffer.
	type CertificateQueryData<'a>;

	/// The query for the certificate.
	type CertificateQuery<'a>: Querylike<
		<Binding::Spec as ParabyzantineAgreementDataSpec>::CertificateEntity,
		Item = Self::CertificateQueryData<'a>,
	>;

	/// The query plan for the certificate.
	type CertificateQueryPlan;

	/// The type of the certificate.
	type Certificate: Certificate<Self::Index, Self::Value, Self::Sender>
		+ for<'a> From<&'a (
			<Binding::Spec as ParabyzantineAgreementDataSpec>::CertificateEntity,
			Self::CertificateQueryData<'a>,
		)>;

	/// The type of the certificate set.
	type CertificateSet: CertificateSet<
		Self::Index,
		Self::Value,
		Self::Sender,
		Self::Certificate,
		Self::Subcommittee,
	>;

	/// The type of the sampler.
	type Sampler: Sampler<
		Self::Index,
		Self::Value,
		Self::Sender,
		Self::Subcommittee,
		Self::IndexSubcommitteeAgreement,
		Binding,
	>;

	/// The type of the ResampleAgreement consensus update.
	type ResampleAgreementConsensusUpdate: ResampleAgreementConsensusUpdate<
		Self::Index,
		Self::Value,
		Self::Sender,
		Binding,
	>;
}

impl ResampleAgreementSpec<NoOp> for NoOp {
	type Index = NoOp;
	type Value = NoOp;
	type Sender = NoOp;
	type Subcommittee = NoOp;
	type IndexSubcommitteeAgreementQueryData<'a> = NoOp;
	type IndexSubcommitteeAgreementQuery<'a> = NoOp;
	type IndexSubcommitteeAgreementQueryPlan = NoOp;
	type IndexSubcommitteeAgreement = NoOp;
	type CertificateQueryData<'a> = NoOp;
	type CertificateQuery<'a> = NoOp;
	type CertificateQueryPlan = NoOp;
	type Certificate = NoOp;
	type CertificateSet = NoOp;
	type Sampler = NoOp;
	type ResampleAgreementConsensusUpdate = NoOp;
}
