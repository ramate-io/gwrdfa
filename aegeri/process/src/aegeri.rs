pub mod data;
pub use data::AegeriData;

use aegeri_message::{AegeriSubcommittee, Index as AegeriIndex, Proposal as AegeriProposal};
use gwrdfa_container::query::matching_tuple::{MatchingTuple, MatchingTupleQuery};
use gwrdfa_resample::{
	agreement::{
		std::{
			AgreementContainer, CertificateContainer, ConstantCommittee, Index,
			MemoryAgreementData, MemoryCertificateSet, Subcom, Value,
		},
		ResampleAgreement, ResampleAgreementData,
	},
	ForResample,
};

pub struct Aegeri {
	/// This is where the parbyzantine data will go.
	data: AegeriData,
	///
	reample_agreement: ResampleAgreement<
		AegeriData,
		MemoryAgreementData<AegeriIndex, AegeriProposal, AegeriSubcommittee, ConstantCommittee>,
	>,
}

impl ResampleAgreementData<AegeriData>
	for MemoryAgreementData<AegeriIndex, AegeriProposal, AegeriSubcommittee, ConstantCommittee>
{
	type Index = Index<AegeriIndex>;
	type Value = Value<AegeriProposal>;
	type Subcommittee = Subcom<AegeriSubcommittee>;
	type IndexSubcommitteeAgreementQuery<'a> = MatchingTupleQuery<
		'a,
		AgreementContainer<AegeriIndex, AegeriProposal, AegeriSubcommittee>,
		(Index<AegeriIndex>, Subcom<AegeriSubcommittee>),
	>;
	type IndexSubcommitteeAgreementQueryPlan =
		MatchingTuple<(Index<AegeriIndex>, Subcom<AegeriSubcommittee>)>;
	type CertificateQuery<'a> = MatchingTupleQuery<
		'a,
		CertificateContainer<AegeriIndex, AegeriProposal, AegeriSubcommittee>,
		(ForResample, Index<AegeriIndex>, Value<AegeriProposal>, Subcom<AegeriSubcommittee>),
	>;
	type CertificateQueryPlan = MatchingTuple<(
		ForResample,
		Index<AegeriIndex>,
		Value<AegeriProposal>,
		Subcom<AegeriSubcommittee>,
	)>;
	type CertificateSet =
		MemoryCertificateSet<Index<AegeriIndex>, Value<AegeriProposal>, Subcom<AegeriSubcommittee>>;
	type Sampler = ConstantCommittee;

	fn certificate_set(
		&self,
	) -> &MemoryCertificateSet<Index<AegeriIndex>, Value<AegeriProposal>, Subcom<AegeriSubcommittee>>
	{
		&self.certificate_set
	}

	fn certificate_set_mut(
		&mut self,
	) -> &mut MemoryCertificateSet<
		Index<AegeriIndex>,
		Value<AegeriProposal>,
		Subcom<AegeriSubcommittee>,
	> {
		&mut self.certificate_set
	}

	fn certificate_query_plan(
		&mut self,
		_index: &Index<AegeriIndex>,
	) -> MatchingTuple<(
		ForResample,
		Index<AegeriIndex>,
		Value<AegeriProposal>,
		Subcom<AegeriSubcommittee>,
	)> {
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
	) -> MatchingTuple<(Index<AegeriIndex>, Subcom<AegeriSubcommittee>)> {
		MatchingTuple::new()
	}
}
