pub mod certificate;
pub mod consensus;
pub mod countable;
pub mod data;
pub mod sampler;
pub mod spec;
pub mod storage;
pub mod subcommittee;

#[cfg(test)]
pub mod test_util;

use crate::Resample;
pub use certificate::CertificateSet;
pub use consensus::Condition;
pub use data::ResampleAgreementData;
use parabyzantine::agreement::{
	Agreement, AgreementWorld, ParabyzantineAgreement, ParabyzantineAgreementData,
	ParabyzantineAgreementDataBinding, ParabyzantineAgreementDataSpec,
};
use parabyzantine::{NoOp, NoOpData};
pub use sampler::Sampler;
pub use spec::ResampleAgreementSpec;
pub use storage::ResampleAgreementStorage;
pub use subcommittee::Subcommittee;

/// A [ResampleAgreementBinding] is a binding for the [ResampleAgreement] protocol.
///
/// It binds between the [ParabyzantineAgreementDataBinding] and the [ResampleAgreementSpec] and the [ResampleAgreementData].
pub trait ResampleAgreementBinding: Sized where <<Self::ParabyzantineAgreementDataBinding as ParabyzantineAgreementDataBinding>::Spec as ParabyzantineAgreementDataSpec>::AgreementDraftBuffer: ResampleAgreementStorage<
			<<Self::ParabyzantineAgreementDataBinding as ParabyzantineAgreementDataBinding>::Spec as ParabyzantineAgreementDataSpec>::AgreementEntity,
			<Self::ResampleAgreementSpec as ResampleAgreementSpec<Self::ParabyzantineAgreementDataBinding>>::Index,
			<Self::ResampleAgreementSpec as ResampleAgreementSpec<Self::ParabyzantineAgreementDataBinding>>::Subcommittee,
			<Self::ResampleAgreementSpec as ResampleAgreementSpec<Self::ParabyzantineAgreementDataBinding>>::Value,
		>,
{
	type ParabyzantineAgreementDataBinding: ParabyzantineAgreementDataBinding;
	type ResampleAgreementSpec: ResampleAgreementSpec<Self::ParabyzantineAgreementDataBinding>;
	type ResampleAgreementData: ResampleAgreementData<
		Self::ParabyzantineAgreementDataBinding,
		Self::ResampleAgreementSpec,
	>;
}

/// [ResampleAgreement] wraps around the ResampleAgreement data indicated by the binding.
///
/// This is mainly used s.t. we can implement the foreign trait for [ParabyzantineAgreement].
///
/// [ResampleAgreement] does not enforce countability restrictions on the [Sampler].
/// Hence, it is sort of an abstraction that exists before the more common [CountableResampleAgreement] implementation.
#[derive(Debug, Clone)]
pub struct ResampleAgreement<Binding: ResampleAgreementBinding>(pub Binding::ResampleAgreementData);
// Because where bounds are not inferred on traits we need to manually specify them,
// this is incredibly ugly and we should find a way to improve this

impl<Binding: ResampleAgreementBinding> ResampleAgreement<Binding>
// Because where bounds are not inferred on traits we need to manually specify them,
// this is incredibly ugly and we should find a way to improve this.
{
	pub fn data(&self) -> &Binding::ResampleAgreementData {
		&self.0
	}

	pub fn data_mut(&mut self) -> &mut Binding::ResampleAgreementData {
		&mut self.0
	}
}

impl<Binding: ResampleAgreementBinding>
	ResampleAgreementData<
		Binding::ParabyzantineAgreementDataBinding,
		Binding::ResampleAgreementSpec,
	> for ResampleAgreement<Binding>
{
	/// A [ResampleAgreement] data must be able to provide a [CertificateSet]
	fn certificate_set(
		&self,
	) -> &<Binding::ResampleAgreementSpec as ResampleAgreementSpec<
		Binding::ParabyzantineAgreementDataBinding,
	>>::CertificateSet {
		self.data().certificate_set()
	}

	/// A [ResampleAgreement] data must be able to provide a mutable [CertificateSet]
	fn certificate_set_mut(
		&mut self,
	) -> &mut <Binding::ResampleAgreementSpec as ResampleAgreementSpec<
		Binding::ParabyzantineAgreementDataBinding,
	>>::CertificateSet {
		self.data_mut().certificate_set_mut()
	}

	/// A [ResampleAgreement] data must be able to provide a [Sampler]
	fn sampler(
		&self,
	) -> &<Binding::ResampleAgreementSpec as ResampleAgreementSpec<
		Binding::ParabyzantineAgreementDataBinding,
	>>::Sampler {
		self.data().sampler()
	}

	/// A [ResampleAgreement] data must be able to provide a mutable [Sampler]
	fn sampler_mut(
		&mut self,
	) -> &mut <Binding::ResampleAgreementSpec as ResampleAgreementSpec<
		Binding::ParabyzantineAgreementDataBinding,
	>>::Sampler {
		self.data_mut().sampler_mut()
	}

	/// A [ResampleAgreement] data must be able to provide a [IndexSubcommitteeAgreementQuery]
	fn index_subcommittee_agreement_query_plan(
		&mut self,
	) -> <Binding::ResampleAgreementSpec as ResampleAgreementSpec<
		Binding::ParabyzantineAgreementDataBinding,
	>>::IndexSubcommitteeAgreementQueryPlan {
		self.data_mut().index_subcommittee_agreement_query_plan()
	}

	/// A [ResampleAgreement] data must be able to provide a [CertificateQuery]
	fn certificate_query_plan(
		&mut self,
		index: &<Binding::ResampleAgreementSpec as ResampleAgreementSpec<
			Binding::ParabyzantineAgreementDataBinding,
		>>::Index,
	) -> <Binding::ResampleAgreementSpec as ResampleAgreementSpec<
		Binding::ParabyzantineAgreementDataBinding,
	>>::CertificateQueryPlan {
		self.data_mut().certificate_query_plan(index)
	}
}

impl<Binding: ResampleAgreementBinding> ParabyzantineAgreement for ResampleAgreement<Binding> {
	type Binding = Binding::ParabyzantineAgreementDataBinding;

	fn update_parabyzantine_agreement(
		&mut self,
		agreement_world: &mut AgreementWorld<
			<Binding::ParabyzantineAgreementDataBinding as ParabyzantineAgreementDataBinding>::Spec,
		>,
	) {
		// over all the index subcommittee agreements
		let index_query = self.index_subcommittee_agreement_query_plan();
		for (_agreement_entity, (index, subcommittee)) in
			agreement_world.agreement_facts.query(index_query)
		{
			// Insert all of the certificates for this index into the certificate set
			let certificate_query_plan = self.certificate_query_plan(index);
			// insert all of the certificates for this index into the certificate set
			for (_certificate_entity, (index, value, subcommittee)) in
				agreement_world.certificate_facts.query(certificate_query_plan)
			{
				// This is just for moving the certificate into the certificate set.
				self.certificate_set_mut().insert(
					index.clone(),
					value.clone(),
					subcommittee.clone(),
				);
			}

			// check the subcommittee condition
			let subcommittee_condition = subcommittee
				.condition(self.certificate_set().partial_subcommittees_for_index(index));

			// elect the next subcommittee from the condition
			let next_subcommittee = self.sampler_mut().elect_subcommittee_from_condition(
				index,
				subcommittee,
				&subcommittee_condition,
			);

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

impl<Binding: ResampleAgreementBinding> ResampleAgreement<Binding> {
	/// A direct implementation of resampling on an agreement world.
	///
	/// This is most useful for experimenting and testing.
	pub fn resample_agreement(
		&mut self,
		agreement_data: &mut <Binding::ParabyzantineAgreementDataBinding as ParabyzantineAgreementDataBinding>::Data,
	) {
		let mut agreement_world = agreement_data.parabyzantine_agreement_world();

		self.update_parabyzantine_agreement(&mut agreement_world);

		agreement_data.commit_parabyzantine_agreement(agreement_world.into());
	}
}

/// A [ResampleAgreementBinding] for the [NoOp] struct.
impl ResampleAgreementBinding for NoOp {
	type ParabyzantineAgreementDataBinding = NoOp;
	type ResampleAgreementSpec = NoOp;
	type ResampleAgreementData = NoOpData;
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::agreement::data::test::TestResampleAgreementData;
	use crate::agreement::spec::test::TestResampleAgreementSpec;
	use crate::agreement::subcommittee::test::TestSubcommittee;
	use crate::agreement::test_util::container::TestResampleAgreementContainer;
	use crate::agreement::test_util::container::TestResampleCertificateContainer;
	use crate::agreement::test_util::container::TestResampleParabyzantineData;
	use crate::agreement::test_util::*;
	use gwrdfa_container::query::matching_tuple::MatchingTuple;
	use gwrdfa_container::{Component, ContainerEntity};
	use parabyzantine::buffer::{Bufferlike, Stores};
	use parabyzantine::{
		agreement::Agreement, task::Task, AgreementAction, AgreementHandler, DataBinding,
		Parabyzantine, Spec, TaskAction, TaskHandler,
	};
	use std::vec;
	use std::vec::Vec;

	impl ResampleAgreementBinding for TestResampleAgreementData<u32, u32, TestSubcommittee<u32>> {
		type ParabyzantineAgreementDataBinding =
			TestResampleParabyzantineData<u32, u32, TestSubcommittee<u32>>;
		type ResampleAgreementSpec = TestResampleAgreementSpec<u32, u32, TestSubcommittee<u32>>;
		type ResampleAgreementData = TestResampleAgreementData<u32, u32, TestSubcommittee<u32>>;
	}

	#[test]
	fn test_noop_resample_agreement_noops() {
		let resample_agreement = ResampleAgreement::<NoOp>(NoOpData::new());
		let mut parabyzantine: Parabyzantine<
			Spec<(
				DataBinding<NoOp>,
				AgreementAction<Agreement>,
				AgreementHandler<ResampleAgreement<NoOp>>,
				TaskAction<Task>,
				TaskHandler<NoOp>,
			)>,
		> = Parabyzantine {
			data: NoOpData::new(),
			agreement_handler: resample_agreement,
			task_handler: NoOp,
		};
		parabyzantine.update_agreement(Agreement);
	}

	#[test]
	fn test_resample_agreement_with_test_util() {
		let mut resample_agreement = ResampleAgreement::<
			TestResampleAgreementData<u32, u32, TestSubcommittee<u32>>,
		>(TestResampleAgreementData::new());

		// Insert genesis agreement
		let genesis: Index<u32> = Index::new(0);
		let genesis_subcommittee: Sub<TestSubcommittee<u32>> =
			Sub::new(TestSubcommittee::new().with_members(vec![1, 2, 3, 4, 5, 6, 7].into_iter()));
		let mut agreement_data =
			TestResampleParabyzantineData::<u32, u32, TestSubcommittee<u32>>::new();

		let genesis_agreement_container = TestResampleAgreementContainer {
			agreement: Component::Present(Agreement),
			index: Component::Present(genesis),
			subcommittee: Component::Present(genesis_subcommittee.clone()),
			..Default::default()
		};
		agreement_data
			.parabyzantine_agreement_agreement_buffer_mut()
			.insert_container(genesis_agreement_container.clone());

		// Insert a certificate from the genesis subcommittee
		agreement_data
			.parabyzantine_agreement_certificate_buffer_mut()
			.insert_container(TestResampleCertificateContainer {
				index: Component::Present(Index::new(0)),
				value: Component::Present(Value::new(1)),
				subcommittee: Component::Present(genesis_subcommittee.clone()),
				..Default::default()
			});

		// Run the resample agreement
		resample_agreement.resample_agreement(&mut agreement_data);

		// We should now be able to query for a new agreement on a value for that index
		let agreement_containers = agreement_data
			.parabyzantine_agreement_agreement_buffer()
			.iter()
			.map(|(_entity, container)| container.clone())
			.collect::<Vec<_>>();
		assert_eq!(agreement_containers.len(), 3);

		// The 0th index should still be the genesis index agreement
		assert_eq!(agreement_containers[0], genesis_agreement_container);
	}
}
