use super::{CertificateSet, ResampleAgreementStorage, Sampler, Subcommittee};
use parabyzantine::agreement::ParabyzantineAgreementData;
use parabyzantine::buffer::query::{QueryPlanlike, Querylike};
use parabyzantine::{NoOp, NoOpData};

pub trait ResampleAgreementData<Data: ParabyzantineAgreementData>: Sized
where
	Data::AgreementDraftBuffer:
		ResampleAgreementStorage<
			Data::AgreementEntity,
			Self::Index,
			Self::Subcommittee,
			Self::Value,
		>,
{
	type Index: Clone + Eq;
	type Value: Clone + Eq + 'static;
	type Subcommittee: Subcommittee<Self::Value> + Clone;
	type IndexSubcommitteeAgreementQuery<'a>: Querylike<
		Data::AgreementEntity,
		(&'a Self::Index, &'a Self::Subcommittee),
	>
	where
		Self::Index: 'a,
		Self::Subcommittee: 'a;
	type IndexSubcommitteeAgreementQueryPlan: for<'a> QueryPlanlike<
		Data::AgreementEntity,
		&'a Data::AgreementBuffer,
		(&'a Self::Index, &'a Self::Subcommittee),
		Self::IndexSubcommitteeAgreementQuery<'a>,
	>;
	type CertificateQuery<'a>: Querylike<
		Data::CertificateEntity,
		(&'a Self::Index, &'a Self::Value, &'a Self::Subcommittee),
	>
	where
		Self::Index: 'a,
		Self::Value: 'a,
		Self::Subcommittee: 'a;
	type CertificateQueryPlan: for<'a> QueryPlanlike<
		Data::CertificateEntity,
		&'a Data::CertificateBuffer,
		(&'a Self::Index, &'a Self::Value, &'a Self::Subcommittee),
		Self::CertificateQuery<'a>,
	>;
	type CertificateSet: CertificateSet<Self::Index, Self::Value, Self::Subcommittee>;
	type Sampler: Sampler<Self::Index, Self::Value, Self::Subcommittee>;

	fn certificate_set(&self) -> &Self::CertificateSet;
	fn certificate_set_mut(&mut self) -> &mut Self::CertificateSet;
	fn sampler(&self) -> &Self::Sampler;
	fn sampler_mut(&mut self) -> &mut Self::Sampler;
	fn index_subcommittee_agreement_query_plan(&mut self) -> Self::IndexSubcommitteeAgreementQueryPlan;
	fn certificate_query_plan(&mut self, index: &Self::Index) -> Self::CertificateQueryPlan;
}

impl ResampleAgreementData<NoOpData> for NoOpData
where
	NoOp: ResampleAgreementStorage<NoOp, NoOp, NoOp, NoOp>,
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

#[cfg(any(test, feature = "std"))]
pub mod test {
	use super::*;
	use crate::agreement::certificate::test::MemoryCertificateSet;
	use crate::agreement::sampler::test::ConstantCommittee;
	use crate::agreement::std::{
		AgreementContainer, AgreementParabyzantineData, CertificateContainer, Index, NextRound,
		Subcom, Value,
	};
	use crate::agreement::Subcommittee;
	use gwrdfa_container::query::matching_tuple::{MatchingTuple, MatchingTupleQuery};
	use std::hash::Hash;

	pub struct AgreementData<
		I: Eq + Hash + Clone + NextRound + 'static,
		V: Eq + Hash + Clone + 'static,
		S: Subcommittee<V> + Hash + Clone + 'static,
	> {
		pub certificate_set: MemoryCertificateSet<Index<I>, Value<V>, Subcom<S>>,
		pub sampler: ConstantCommittee,
	}

	impl<
			I: Eq + Hash + Clone + NextRound + 'static,
			V: Eq + Hash + Clone + 'static,
			S: Subcommittee<V> + Hash + Clone + 'static,
		> AgreementData<I, V, S>
	{
		pub fn new() -> Self {
			Self { certificate_set: MemoryCertificateSet::new(), sampler: ConstantCommittee::new() }
		}
	}

	impl<
			I: Eq + Hash + Clone + NextRound + 'static,
			V: Eq + Hash + Clone + 'static,
			S: Subcommittee<V> + Hash + Clone + 'static,
		>
		ResampleAgreementData<AgreementParabyzantineData<I, V, S>>
		for AgreementData<I, V, S>
	{
		type Index = Index<I>;
		type Value = Value<V>;
		type Subcommittee = Subcom<S>;
		type IndexSubcommitteeAgreementQuery<'a> =
			MatchingTupleQuery<'a, AgreementContainer<I, V, S>, (Index<I>, Subcom<S>)>;
		type IndexSubcommitteeAgreementQueryPlan = MatchingTuple<(Index<I>, Subcom<S>)>;
		type CertificateQuery<'a> = MatchingTupleQuery<
			'a,
			CertificateContainer<I, V, S>,
			(Index<I>, Value<V>, Subcom<S>),
		>;
		type CertificateQueryPlan = MatchingTuple<(Index<I>, Value<V>, Subcom<S>)>;
		type CertificateSet = MemoryCertificateSet<Index<I>, Value<V>, Subcom<S>>;
		type Sampler = ConstantCommittee;

		fn certificate_set(&self) -> &MemoryCertificateSet<Index<I>, Value<V>, Subcom<S>> {
			&self.certificate_set
		}
		fn certificate_set_mut(
			&mut self,
		) -> &mut MemoryCertificateSet<Index<I>, Value<V>, Subcom<S>> {
			&mut self.certificate_set
		}

		fn certificate_query_plan(&mut self, _index: &Index<I>) -> MatchingTuple<(Index<I>, Value<V>, Subcom<S>)> {
			MatchingTuple::new()
		}

		fn sampler(&self) -> &ConstantCommittee {
			&self.sampler
		}
		fn sampler_mut(&mut self) -> &mut ConstantCommittee {
			&mut self.sampler
		}
		fn index_subcommittee_agreement_query_plan(
			&mut self,
		) -> MatchingTuple<(Index<I>, Subcom<S>)> {
			MatchingTuple::new()
		}
	}

	#[test]
	fn test_esample_agreement_data() {
		use crate::agreement::subcommittee::test::Committee;

		let mut data = AgreementData::<u32, u32, Committee<u32>>::new();
		let index = Index::new(0);
		let value = Value::new(0);
		let subcommittee = Subcom::new(Committee::new());

		data.certificate_set_mut().insert(index.clone(), value, subcommittee);
		assert_eq!(data.certificate_set().partial_subcommittees_for_index(&index).count(), 1);
	}

	pub type TestResampleAgreementData<I, V, S> = AgreementData<I, V, S>;
}
