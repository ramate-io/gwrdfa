use crate::agreement::certificate::test::MemoryCertificateSet;
use crate::agreement::std::{
	AgreementContainer, AgreementParabyzantineData, CertificateContainer, ConstantCommittee, Index,
	NextRound, Subcom, Value,
};
use crate::agreement::{ResampleAgreementData, Subcommittee};
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
	> ResampleAgreementData<AgreementParabyzantineData<I, V, S>> for AgreementData<I, V, S>
{
	type Index = Index<I>;
	type Value = Value<V>;
	type Subcommittee = Subcom<S>;
	type IndexSubcommitteeAgreementQuery<'a> =
		MatchingTupleQuery<'a, AgreementContainer<I, V, S>, (Index<I>, Subcom<S>)>;
	type IndexSubcommitteeAgreementQueryPlan = MatchingTuple<(Index<I>, Subcom<S>)>;
	type CertificateQuery<'a> =
		MatchingTupleQuery<'a, CertificateContainer<I, V, S>, (Index<I>, Value<V>, Subcom<S>)>;
	type CertificateQueryPlan = MatchingTuple<(Index<I>, Value<V>, Subcom<S>)>;
	type CertificateSet = MemoryCertificateSet<Index<I>, Value<V>, Subcom<S>>;
	type Sampler = ConstantCommittee;

	fn certificate_set(&self) -> &MemoryCertificateSet<Index<I>, Value<V>, Subcom<S>> {
		&self.certificate_set
	}

	fn certificate_set_mut(&mut self) -> &mut MemoryCertificateSet<Index<I>, Value<V>, Subcom<S>> {
		&mut self.certificate_set
	}

	fn certificate_query_plan(
		&mut self,
		_index: &Index<I>,
	) -> MatchingTuple<(Index<I>, Value<V>, Subcom<S>)> {
		MatchingTuple::new()
	}

	fn sampler(&self) -> &ConstantCommittee {
		&self.sampler
	}

	fn sampler_mut(&mut self) -> &mut ConstantCommittee {
		&mut self.sampler
	}

	fn index_subcommittee_agreement_query_plan(&mut self) -> MatchingTuple<(Index<I>, Subcom<S>)> {
		MatchingTuple::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::agreement::std::VoterSet;
	use crate::agreement::CertificateSet;

	#[test]
	fn agreement_data_uses_memory_certificate_set() {
		let mut data = AgreementData::<u32, u32, VoterSet<u32>>::new();
		let index = Index::new(0);
		let value = Value::new(0);
		let subcom = Subcom::new(VoterSet::new());
		data.certificate_set_mut().insert(index.clone(), value, subcom);
		assert_eq!(data.certificate_set().partial_subcommittees_for_index(&index).count(), 1);
	}
}
