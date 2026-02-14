use super::{
	Certificate, CertificateSet, IndexSubcommitteeAgreement, ResampleAgreementConsensusUpdate,
	Sampler, Subcommittee,
};
use parabyzantine::NoOp;
use parabyzantine::{
	agreement::{ParabyzantineAgreementBinding, ParabyzantineAgreementSpec},
	buffer::{Bundle, Querylike},
};

/// A [ResampleAgreementSpec] is a specification for ResampleAgreement consensus.
pub trait ResampleAgreementSpec<Binding: ParabyzantineAgreementBinding>: Sized {
	/// The type of the index.
	type Index: Eq;

	/// The type of the value.
	type Value: Eq;

	/// The type of the sender of a certificate.
	type Sender: Eq;

	/// The type of the subcommittee.
	type Subcommittee: Subcommittee<Self::Sender>;

	/// The bundle of the agreement in the buffer.
	type IndexSubcommitteeAgreementBundle: Bundle;

	/// The query for the index subcommittee agreement.
	type IndexSubcommitteeAgreementQuery: Querylike<
		<Binding::Spec as ParabyzantineAgreementSpec>::AgreementEntity,
		<Binding::Spec as ParabyzantineAgreementSpec>::AgreementBuffer,
		Self::IndexSubcommitteeAgreementBundle,
	>;

	/// The type of the index subcommittee agreement.
	type IndexSubcommitteeAgreement: IndexSubcommitteeAgreement<Self::Index, Self::Sender, Self::Subcommittee>
		+ for<'a> From<&'a (
			<Binding::Spec as ParabyzantineAgreementSpec>::AgreementEntity,
			Self::IndexSubcommitteeAgreementBundle,
		)>;

	/// The bundle of the certificate in the buffer.
	type CertificateBundle: Bundle;

	/// The query for the certificate.
	type CertificateQuery: Querylike<
		<Binding::Spec as ParabyzantineAgreementSpec>::CertificateEntity,
		<Binding::Spec as ParabyzantineAgreementSpec>::CertificateBuffer,
		Self::CertificateBundle,
	>;

	/// The type of the certificate.
	type Certificate: Certificate<Self::Index, Self::Value, Self::Sender>
		+ for<'a> From<&'a (
			<Binding::Spec as ParabyzantineAgreementSpec>::CertificateEntity,
			Self::CertificateBundle,
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
	type IndexSubcommitteeAgreementBundle = NoOp;
	type IndexSubcommitteeAgreementQuery = NoOp;
	type IndexSubcommitteeAgreement = NoOp;
	type CertificateBundle = NoOp;
	type CertificateQuery = NoOp;
	type Certificate = NoOp;
	type CertificateSet = NoOp;
	type Sampler = NoOp;
	type ResampleAgreementConsensusUpdate = NoOp;
}
