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
	use crate::agreement::certificate::test::TestCertificateSet;
	use crate::agreement::sampler::test::TestSampler;
	use crate::agreement::spec::test::TestResampleAgreementSpec;
	use crate::agreement::test_util::container::*;
	use crate::agreement::test_util::{Index, Sub, TextIndexabled, Value};
	use crate::agreement::Subcommittee;
	use gwrdfa_container::query::matching_tuple::MatchingTuple;
	use std::hash::Hash;

	pub struct TestResampleAgreementData<
		I: Eq + Hash + Clone + TextIndexabled + 'static,
		V: Eq + Hash + Clone + 'static,
		S: Subcommittee<V> + Hash + Clone + 'static,
	> {
		pub certificate_set: TestCertificateSet<Index<I>, Value<V>, Sub<S>>,
		pub sampler: TestSampler,
	}

	impl<
			I: Eq + Hash + Clone + TextIndexabled + 'static,
			V: Eq + Hash + Clone + 'static,
			S: Subcommittee<V> + Hash + Clone + 'static,
		>
		ResampleAgreementData<
			TestResampleParabyzantineData<I, V, S>,
			TestResampleAgreementSpec<I, V, S>,
		> for TestResampleAgreementData<I, V, S>
	{
		fn certificate_set(&self) -> &TestCertificateSet<Index<I>, Value<V>, Sub<S>> {
			&self.certificate_set
		}
		fn certificate_set_mut(&mut self) -> &mut TestCertificateSet<Index<I>, Value<V>, Sub<S>> {
			&mut self.certificate_set
		}

		fn certificate_query_plan(
			&mut self,
			_index: &<TestResampleAgreementSpec<I, V, S> as ResampleAgreementSpec<
				TestResampleParabyzantineData<I, V, S>,
			>>::Index,
		) -> <TestResampleAgreementSpec<I, V, S> as ResampleAgreementSpec<
			TestResampleParabyzantineData<I, V, S>,
		>>::CertificateQueryPlan {
			MatchingTuple::new()
		}

		fn sampler(&self) -> &TestSampler {
			&self.sampler
		}
		fn sampler_mut(&mut self) -> &mut TestSampler {
			&mut self.sampler
		}
		fn index_subcommittee_agreement_query_plan(&mut self) -> MatchingTuple<(Index<I>, Sub<S>)> {
			MatchingTuple::new()
		}
	}
}
