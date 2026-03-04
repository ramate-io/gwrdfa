//! Resample agreement protocol.
//!
//! This module wires together:
//! - certificate projection (`ResampleAgreementData::certificate_query_plan`),
//! - condition evaluation (`Subcommittee::condition`), and
//! - next-step election (`Sampler::elect_subcommittee_from_condition`).

pub mod certificate;
pub mod consensus;
pub mod data;
pub mod sampler;
pub mod storage;
pub mod subcommittee;

#[cfg(any(test, feature = "std"))]
pub mod std;

use crate::Resample;
pub use certificate::CertificateSet;
pub use consensus::Condition;
use core::marker::PhantomData;
pub use data::ResampleAgreementData;
use parabyzantine::agreement::{
	Agreement, AgreementWorld, ParabyzantineAgreement, ParabyzantineAgreementData,
};
pub use sampler::Sampler;
pub use storage::ResampleAgreementStorage;
pub use subcommittee::Subcommittee;

/// [ResampleAgreement] wraps around resample agreement data for a given parabyzantine data type.
///
/// This is mainly used s.t. we can implement the foreign trait for [ParabyzantineAgreement].
///
/// [ResampleAgreement] does not enforce countability restrictions on the [Sampler].
/// It is intentionally a lower-level abstraction that can be wrapped by
/// countability-constrained protocols.
#[derive(Debug, Clone)]
pub struct ResampleAgreement<
	Data: ParabyzantineAgreementData,
	ResampleData: ResampleAgreementData<Data>,
>(pub ResampleData, PhantomData<Data>)
where
	Data::AgreementDraftBuffer: ResampleAgreementStorage<
		Data::AgreementEntity,
		ResampleData::Index,
		ResampleData::Subcommittee,
		ResampleData::Value,
	>;
// Because where bounds are not inferred on traits we need to manually specify them,
// this is incredibly ugly and we should find a way to improve this

impl<Data: ParabyzantineAgreementData, ResampleData: ResampleAgreementData<Data>>
	ResampleAgreement<Data, ResampleData>
where
	Data::AgreementDraftBuffer: ResampleAgreementStorage<
		Data::AgreementEntity,
		ResampleData::Index,
		ResampleData::Subcommittee,
		ResampleData::Value,
	>,
	// Because where bounds are not inferred on traits we need to manually specify them,
	// this is incredibly ugly and we should find a way to improve this.
{
	/// Creates a protocol wrapper over concrete resample agreement data.
	pub fn new(data: ResampleData) -> Self {
		Self(data, PhantomData)
	}

	/// Immutable access to the underlying resample data implementation.
	pub fn data(&self) -> &ResampleData {
		&self.0
	}

	/// Mutable access to the underlying resample data implementation.
	pub fn data_mut(&mut self) -> &mut ResampleData {
		&mut self.0
	}
}

impl<Data: ParabyzantineAgreementData, ResampleData: ResampleAgreementData<Data>>
	ParabyzantineAgreement<Data> for ResampleAgreement<Data, ResampleData>
where
	Data::AgreementDraftBuffer: ResampleAgreementStorage<
		Data::AgreementEntity,
		ResampleData::Index,
		ResampleData::Subcommittee,
		ResampleData::Value,
	>,
{
	fn update_parabyzantine_agreement(&mut self, agreement_world: &mut AgreementWorld<Data>) {
		// over all the index subcommittee agreements
		let index_query = self.data_mut().index_subcommittee_agreement_query_plan();
		for (_agreement_entity, (index, subcommittee)) in
			agreement_world.agreement_facts.query(index_query)
		{
			// Insert all of the certificates for this index into the certificate set
			let certificate_query_plan = self.data_mut().certificate_query_plan(index);
			// insert all of the certificates for this index into the certificate set
			for (_certificate_entity, (_for_resample, index, value, subcommittee)) in
				agreement_world.certificate_facts.query(certificate_query_plan)
			{
				// This is just for moving the certificate into the certificate set.
				self.data_mut().certificate_set_mut().insert(
					index.clone(),
					value.clone(),
					subcommittee.clone(),
				);
			}

			// check the subcommittee condition
			let subcommittee_condition = {
				let certificate_set = self.data().certificate_set();
				subcommittee.condition(certificate_set.partial_subcommittees_for_index(index))
			};

			// elect the next subcommittee from the condition
			let next_subcommittee = self
				.data_mut()
				.sampler_mut()
				.elect_subcommittee_from_condition(index, subcommittee, &subcommittee_condition);

			if let Some((next_index, next_subcommittee)) = next_subcommittee {
				// insert the next subcommittee into the agreement world
				agreement_world.agreement_inferences.insert(
					None,
					(Agreement, Resample, next_index.clone(), next_subcommittee.clone()),
				);
			}

			match subcommittee_condition {
				Condition::Consensus(value) => {
					// insert the value into the agreement world
					agreement_world
						.agreement_inferences
						.insert(None, (Agreement, Resample, index.clone(), value));
				}
				Condition::Hung => {
					// do nothing
				}
				Condition::InProgress => {
					// do nothing
				}
			}
		}
	}
}

impl<Data: ParabyzantineAgreementData, ResampleData: ResampleAgreementData<Data>>
	ResampleAgreement<Data, ResampleData>
where
	Data::AgreementDraftBuffer: ResampleAgreementStorage<
		Data::AgreementEntity,
		ResampleData::Index,
		ResampleData::Subcommittee,
		ResampleData::Value,
	>,
{
	/// A direct implementation of resampling on an agreement world.
	///
	/// This is most useful for experimenting and testing.
	pub fn resample_agreement(&mut self, agreement_data: &mut Data) {
		let mut agreement_world = agreement_data.parabyzantine_agreement_world();

		self.update_parabyzantine_agreement(&mut agreement_world);

		agreement_data.commit_parabyzantine_agreement(agreement_world.into());
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::agreement::std::{
		AgreementContainer, AgreementParabyzantineData, CertificateContainer, Index,
		MemoryAgreementData, Subcom, Value, VoterSet,
	};
	use crate::task::ResampleTask;
	use crate::ForResample;
	use ::std::collections::BTreeSet;
	use ::std::vec;
	use gwrdfa_container::Component;
	use parabyzantine::{agreement::Agreement, NoOpData, Parabyzantine};

	fn insert_certificate(
		agreement_data: &mut AgreementParabyzantineData<u32, u32, VoterSet<u32>>,
		index: u32,
		value: u32,
		subcommittee: Subcom<VoterSet<u32>>,
	) {
		agreement_data
			.parabyzantine_agreement_certificate_buffer_mut()
			.insert_container(CertificateContainer {
				for_resample: Component::Present(ForResample),
				index: Component::Present(Index::new(index)),
				value: Component::Present(Value::new(value)),
				subcommittee: Component::Present(subcommittee),
				..Default::default()
			});
	}

	#[test]
	fn test_noop_resample_agreement_noops() {
		let resample_agreement = ResampleAgreement::new(NoOpData::new());
		let mut parabyzantine = Parabyzantine::new(
			NoOpData::new(),
			resample_agreement,
			ResampleTask::new(NoOpData::new()),
		);
		parabyzantine.update_agreement(Agreement);
	}

	#[test]
	fn test_resample_agreement_with_std_support() {
		let mut resample_agreement = ResampleAgreement::<
			AgreementParabyzantineData<u32, u32, VoterSet<u32>>,
			MemoryAgreementData<u32, u32, VoterSet<u32>>,
		>::new(MemoryAgreementData::new());

		// Insert genesis agreement
		let genesis: Index<u32> = Index::new(0);
		let genesis_subcommittee: Subcom<VoterSet<u32>> =
			Subcom::new(VoterSet::new().with_members(vec![1, 2, 3, 4, 5, 6, 7].into_iter()));
		let mut agreement_data = AgreementParabyzantineData::<u32, u32, VoterSet<u32>>::new();

		let genesis_agreement_container = AgreementContainer {
			agreement: Component::Present(Agreement),
			index: Component::Present(genesis),
			subcommittee: Component::Present(genesis_subcommittee.clone()),
			..Default::default()
		};
		agreement_data
			.parabyzantine_agreement_agreement_buffer_mut()
			.insert_container(genesis_agreement_container.clone());

		// Insert a certificate from the genesis subcommittee
		insert_certificate(&mut agreement_data, 0, 1, genesis_subcommittee.clone());

		// Run the resample agreement
		resample_agreement.resample_agreement(&mut agreement_data);

		// We should now be able to query for a new agreement on a value for that index
		let agreement_containers = agreement_data
			.parabyzantine_agreement_agreement_buffer()
			.iter()
			.map(|(_entity, container)| container.clone())
			.collect::<BTreeSet<_>>();
		assert_eq!(agreement_containers.len(), 3);

		let reference_agreement_containers = vec![
			genesis_agreement_container.clone(),
			// Next subcommittee agreement
			AgreementContainer {
				agreement: Component::Present(Agreement),
				resample: Component::Present(Resample),
				// Next index
				index: Component::Present(Index::new(1)),
				// still the same committee by the rule of the [ConstantCommittee]
				subcommittee: Component::Present(genesis_subcommittee.clone()),
				..Default::default()
			},
			AgreementContainer {
				agreement: Component::Present(Agreement),
				resample: Component::Present(Resample),
				index: Component::Present(Index::new(0)),
				value: Component::Present(Value::new(1)),
				..Default::default()
			},
		]
		.into_iter()
		.collect::<BTreeSet<_>>();
		assert_eq!(agreement_containers, reference_agreement_containers);
	}

	#[test]
	fn test_resample_agreement_transitions_across_updates() {
		let mut resample_agreement = ResampleAgreement::<
			AgreementParabyzantineData<u32, u32, VoterSet<u32>>,
			MemoryAgreementData<u32, u32, VoterSet<u32>>,
		>::new(MemoryAgreementData::new());
		let mut agreement_data = AgreementParabyzantineData::<u32, u32, VoterSet<u32>>::new();

		let all_voters: Subcom<VoterSet<u32>> =
			Subcom::new(VoterSet::new().with_members(vec![1, 2, 3, 4, 5, 6, 7].into_iter()));
		let first_half: Subcom<VoterSet<u32>> =
			Subcom::new(VoterSet::new().with_members(vec![1, 2, 3].into_iter()));
		let second_half: Subcom<VoterSet<u32>> =
			Subcom::new(VoterSet::new().with_members(vec![4, 5, 6].into_iter()));

		let genesis_agreement = AgreementContainer {
			agreement: Component::Present(Agreement),
			index: Component::Present(Index::new(0)),
			subcommittee: Component::Present(all_voters.clone()),
			..Default::default()
		};
		agreement_data
			.parabyzantine_agreement_agreement_buffer_mut()
			.insert_container(genesis_agreement.clone());

		// Transition 1: InProgress (insufficient support) -> no new agreements.
		insert_certificate(&mut agreement_data, 0, 10, first_half.clone());
		resample_agreement.resample_agreement(&mut agreement_data);
		let agreement_containers = agreement_data
			.parabyzantine_agreement_agreement_buffer()
			.iter()
			.map(|(_entity, container)| container.clone())
			.collect::<BTreeSet<_>>();
		assert_eq!(
			agreement_containers,
			vec![genesis_agreement.clone()].into_iter().collect::<BTreeSet<_>>()
		);

		// Transition 2: Consensus at index 0 -> emit value agreement and next index committee.
		insert_certificate(&mut agreement_data, 0, 10, second_half.clone());
		resample_agreement.resample_agreement(&mut agreement_data);
		let agreement_containers = agreement_data
			.parabyzantine_agreement_agreement_buffer()
			.iter()
			.map(|(_entity, container)| container.clone())
			.collect::<BTreeSet<_>>();
		let expected_after_index_0_consensus = vec![
			genesis_agreement.clone(),
			AgreementContainer {
				agreement: Component::Present(Agreement),
				resample: Component::Present(Resample),
				index: Component::Present(Index::new(0)),
				value: Component::Present(Value::new(10)),
				..Default::default()
			},
			AgreementContainer {
				agreement: Component::Present(Agreement),
				resample: Component::Present(Resample),
				index: Component::Present(Index::new(1)),
				subcommittee: Component::Present(all_voters.clone()),
				..Default::default()
			},
		]
		.into_iter()
		.collect::<BTreeSet<_>>();
		assert_eq!(agreement_containers, expected_after_index_0_consensus);

		// Transition 3: InProgress at index 1 -> no additional agreements yet.
		insert_certificate(&mut agreement_data, 1, 20, first_half);
		resample_agreement.resample_agreement(&mut agreement_data);
		let agreement_containers = agreement_data
			.parabyzantine_agreement_agreement_buffer()
			.iter()
			.map(|(_entity, container)| container.clone())
			.collect::<BTreeSet<_>>();
		assert_eq!(agreement_containers, expected_after_index_0_consensus);

		// Transition 4: Consensus at index 1 -> emit value agreement and index 2 committee.
		insert_certificate(&mut agreement_data, 1, 20, second_half);
		resample_agreement.resample_agreement(&mut agreement_data);
		let agreement_containers = agreement_data
			.parabyzantine_agreement_agreement_buffer()
			.iter()
			.map(|(_entity, container)| container.clone())
			.collect::<BTreeSet<_>>();
		let expected_after_index_1_consensus = vec![
			genesis_agreement,
			AgreementContainer {
				agreement: Component::Present(Agreement),
				resample: Component::Present(Resample),
				index: Component::Present(Index::new(0)),
				value: Component::Present(Value::new(10)),
				..Default::default()
			},
			AgreementContainer {
				agreement: Component::Present(Agreement),
				resample: Component::Present(Resample),
				index: Component::Present(Index::new(1)),
				subcommittee: Component::Present(all_voters.clone()),
				..Default::default()
			},
			AgreementContainer {
				agreement: Component::Present(Agreement),
				resample: Component::Present(Resample),
				index: Component::Present(Index::new(1)),
				value: Component::Present(Value::new(20)),
				..Default::default()
			},
			AgreementContainer {
				agreement: Component::Present(Agreement),
				resample: Component::Present(Resample),
				index: Component::Present(Index::new(2)),
				subcommittee: Component::Present(all_voters),
				..Default::default()
			},
		]
		.into_iter()
		.collect::<BTreeSet<_>>();
		assert_eq!(agreement_containers, expected_after_index_1_consensus);
	}
}
