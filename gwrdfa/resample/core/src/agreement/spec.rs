use super::{CertificateSet, ResampleAgreementStorage, Sampler, Subcommittee};
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

impl<Binding: ParabyzantineAgreementDataBinding> ResampleAgreementSpec<Binding> for NoOp
where
	<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementDraftBuffer:
		ResampleAgreementStorage<
			<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementEntity,
			NoOp,
			NoOp,
			NoOp,
		>,
{
	type Index = NoOp;
	type Value = NoOp;
	type Subcommittee = NoOp;
	type IndexSubcommitteeAgreementQuery<'a> = NoOp;
	type IndexSubcommitteeAgreementQueryPlan = NoOp;
	type CertificateQuery<'a> = NoOp;
	type CertificateQueryPlan = NoOp;
	type CertificateSet = NoOp;
	type Sampler = NoOp;
}

#[cfg(test)]
pub mod test {
	use super::*;
	use crate::agreement::test_util::container::{
		TestResampleAgreementContainer, TestResampleParabyzantineData,
	};
	use crate::agreement::test_util::{Index, Sub, Value};
	use core::marker::PhantomData;
	use gwrdfa_container::query::matching_tuple::{MatchingTuple, MatchingTupleQuery};

	pub struct TestResampleAgreementSpec<Index: Eq, Value: Eq + 'static, Sub: Subcommittee<Value>> {
		__marker: PhantomData<(Index, Value, Sub)>,
	}

	impl<I: Eq + Clone, V: Eq + 'static + Clone, S: Subcommittee<V> + Clone>
		ResampleAgreementSpec<TestResampleParabyzantineData<Index<I>, Value<V>, Sub<S>>>
		for TestResampleAgreementSpec<Index<I>, Value<V>, Sub<S>>
	{
		type Index = Index<I>;
		type Value = Value<V>;
		type Subcommittee = Sub<S>;
		type IndexSubcommitteeAgreementQuery<'a> =
			MatchingTupleQuery<'a, TestResampleAgreementContainer<I, V, S>, (I, S)>;
		type IndexSubcommitteeAgreementQueryPlan = MatchingTuple<(I, S)>;
		type CertificateQuery<'a> = NoOp;
		type CertificateQueryPlan = NoOp;
		type CertificateSet = TestCertificateSet<Index, Value, Sub>;
		type Sampler = TestSampler;
	}
}
