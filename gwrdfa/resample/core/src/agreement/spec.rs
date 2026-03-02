use super::{CertificateSet, ResampleAgreementStorage, Sampler, Subcommittee};
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

	/// The query for the index subcommittee agreement.
	type IndexSubcommitteeAgreementQuery<'a>: Querylike<
		<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementEntity,
		(&'a Self::Index, &'a Self::Subcommittee),
	>
	where
		Self::Index: 'a,
		Self::Subcommittee: 'a;

	/// The query plan for the index subcommittee agreement.
	type IndexSubcommitteeAgreementQueryPlan: for<'a> QueryPlanlike<
		<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementEntity,
		&'a <Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementBuffer,
		(&'a Self::Index, &'a Self::Subcommittee),
		Self::IndexSubcommitteeAgreementQuery<'a>,
	>;

	/// The query for the certificate.
	type CertificateQuery<'a>: Querylike<
		<Binding::Spec as ParabyzantineAgreementDataSpec>::CertificateEntity,
		(&'a Self::Index, &'a Self::Value, &'a Self::Subcommittee),
	>
	where
		Self::Index: 'a,
		Self::Value: 'a,
		Self::Subcommittee: 'a;

	/// The query plan for the certificate.
	type CertificateQueryPlan: for<'a> QueryPlanlike<
		<Binding::Spec as ParabyzantineAgreementDataSpec>::CertificateEntity,
		&'a <Binding::Spec as ParabyzantineAgreementDataSpec>::CertificateBuffer,
		(&'a Self::Index, &'a Self::Value, &'a Self::Subcommittee),
		Self::CertificateQuery<'a>,
	>;

	/// The type of the certificate set.
	type CertificateSet: CertificateSet<Self::Index, Self::Value, Self::Subcommittee>;

	/// The type of the sampler.
	type Sampler: Sampler<Self::Index, Self::Value, Self::Subcommittee>;
}
