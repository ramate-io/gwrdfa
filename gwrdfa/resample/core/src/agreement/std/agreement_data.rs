use crate::agreement::std::MemoryCertificateSet;
use crate::agreement::std::{
	AgreementContainer, AgreementParabyzantineData, CertificateContainer, ConstantCommittee, Index,
	NextRound, Subcom, Value,
};
use crate::agreement::{ResampleAgreementData, Sampler, Subcommittee};
use crate::ForResample;
use gwrdfa_container::query::matching_tuple::{MatchingTuple, MatchingTupleQuery};
use parabyzantine::agreement::ParabyzantineAgreementData;
use std::hash::Hash;

/// In-memory `ResampleAgreementData` implementation backed by container buffers.
///
/// Design notes:
/// - Uses strongly-typed wrappers (`Index`, `Value`, `Subcom`) to avoid accidental
///   tuple-position errors across agreement/certificate flows.
/// - Uses `MatchingTuple` queries so the same container shape can be queried by
///   different tuple projections.
/// - Keeps sampler pluggable via `Sm` (defaulting to [`ConstantCommittee`]),
///   which is useful for testing alternate election strategies.
///
/// This type is intended as a reusable reference implementation for std/testing
/// contexts, not as the only production storage strategy.
pub struct MemoryAgreementData<
	I: Eq + Hash + Clone + NextRound + 'static,
	V: Eq + Hash + Clone + 'static,
	S: Subcommittee<V> + Hash + Clone + 'static,
	Sm: Sampler<Index<I>, Value<V>, Subcom<S>> = ConstantCommittee,
> {
	pub certificate_set: MemoryCertificateSet<Index<I>, Value<V>, Subcom<S>>,
	pub sampler: Sm,
}

impl<
		I: Eq + Hash + Clone + NextRound + 'static,
		V: Eq + Hash + Clone + 'static,
		S: Subcommittee<V> + Hash + Clone + 'static,
		Sm: Sampler<Index<I>, Value<V>, Subcom<S>> + Default,
	> MemoryAgreementData<I, V, S, Sm>
{
	/// Builds with empty in-memory certificate state and `Sm::default()`.
	pub fn new() -> Self {
		Self { certificate_set: MemoryCertificateSet::new(), sampler: Sm::default() }
	}
}

impl<
		I: Eq + Hash + Clone + NextRound + 'static,
		V: Eq + Hash + Clone + 'static,
		S: Subcommittee<V> + Hash + Clone + 'static,
		Sm: Sampler<Index<I>, Value<V>, Subcom<S>>,
	> MemoryAgreementData<I, V, S, Sm>
{
	/// Builds with empty in-memory certificate state and a caller-provided sampler.
	pub fn with_sampler(sampler: Sm) -> Self {
		Self { certificate_set: MemoryCertificateSet::new(), sampler }
	}
}

impl<
		I: Eq + Hash + Clone + NextRound + 'static,
		V: Eq + Hash + Clone + 'static,
		S: Subcommittee<V> + Hash + Clone + 'static,
		Sm: Sampler<Index<I>, Value<V>, Subcom<S>>,
	> ResampleAgreementData<AgreementParabyzantineData<I, V, S>> for MemoryAgreementData<I, V, S, Sm>
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
		(ForResample, Index<I>, Value<V>, Subcom<S>),
	>;
	type CertificateQueryPlan = MatchingTuple<(ForResample, Index<I>, Value<V>, Subcom<S>)>;
	type CertificateSet = MemoryCertificateSet<Index<I>, Value<V>, Subcom<S>>;
	type Sampler = Sm;

	fn certificate_set(&self) -> &MemoryCertificateSet<Index<I>, Value<V>, Subcom<S>> {
		&self.certificate_set
	}

	fn certificate_set_mut(&mut self) -> &mut MemoryCertificateSet<Index<I>, Value<V>, Subcom<S>> {
		&mut self.certificate_set
	}

	fn certificate_query_plan(
		&mut self,
		_index: &Index<I>,
	) -> MatchingTuple<(ForResample, Index<I>, Value<V>, Subcom<S>)> {
		MatchingTuple::new()
	}

	fn sampler(&self) -> &Sm {
		&self.sampler
	}

	fn sampler_mut(&mut self) -> &mut Sm {
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
		let mut data = MemoryAgreementData::<u32, u32, VoterSet<u32>>::new();
		let index = Index::new(0);
		let value = Value::new(0);
		let subcom = Subcom::new(VoterSet::new());
		data.certificate_set_mut().insert(index.clone(), value, subcom);
		assert_eq!(data.certificate_set().partial_subcommittees_for_index(&index).count(), 1);
	}
}
