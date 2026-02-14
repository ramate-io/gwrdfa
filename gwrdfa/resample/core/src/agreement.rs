pub mod certificate;
pub mod consensus;
pub mod countable;
pub mod data;
pub mod sampler;
pub mod spec;
pub mod subcommittee;

pub use certificate::{Certificate, CertificateSet};
pub use consensus::{Condition, ResampleAgreementConsensusUpdate};
pub use data::ResampleAgreementData;
use parabyzantine::agreement::{
	AgreementWorld, ParabyzantineAgreement, ParabyzantineAgreementBinding,
	ParabyzantineAgreementData, ParabyzantineAgreementDataSpec,
};
use parabyzantine::{NoOp, NoOpData};
pub use sampler::Sampler;
pub use spec::ResampleAgreementSpec;
pub use subcommittee::{IndexSubcommitteeAgreement, Subcommittee};

/// A [ResampleAgreementBinding] is a binding for the [ResampleAgreement] protocol.
///
/// It binds between the [ParabyzantineAgreementBinding] and the [ResampleAgreementSpec] and the [ResampleAgreementData].
pub trait ResampleAgreementBinding: Sized {
	type ParabyzantineAgreementBinding: ParabyzantineAgreementBinding;
	type ResampleAgreementSpec: ResampleAgreementSpec<Self::ParabyzantineAgreementBinding>;
	type ResampleAgreementData: ResampleAgreementData<
		Self::ParabyzantineAgreementBinding,
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

impl<Binding: ResampleAgreementBinding> ResampleAgreement<Binding> {
	pub fn data(&self) -> &Binding::ResampleAgreementData {
		&self.0
	}

	pub fn data_mut(&mut self) -> &mut Binding::ResampleAgreementData {
		&mut self.0
	}
}

impl<Binding: ResampleAgreementBinding>
	ResampleAgreementData<Binding::ParabyzantineAgreementBinding, Binding::ResampleAgreementSpec>
	for ResampleAgreement<Binding>
{
	/// A [ResampleAgreement] data must be able to provide a [CertificateSet]
	fn certificate_set(
		&self,
	) -> &<Binding::ResampleAgreementSpec as ResampleAgreementSpec<
		Binding::ParabyzantineAgreementBinding,
	>>::CertificateSet {
		self.data().certificate_set()
	}

	/// A [ResampleAgreement] data must be able to provide a mutable [CertificateSet]
	fn certificate_set_mut(
		&mut self,
	) -> &mut <Binding::ResampleAgreementSpec as ResampleAgreementSpec<
		Binding::ParabyzantineAgreementBinding,
	>>::CertificateSet {
		self.data_mut().certificate_set_mut()
	}

	/// A [ResampleAgreement] data must be able to provide a [Sampler]
	fn sampler(
		&self,
	) -> &<Binding::ResampleAgreementSpec as ResampleAgreementSpec<
		Binding::ParabyzantineAgreementBinding,
	>>::Sampler {
		self.data().sampler()
	}

	/// A [ResampleAgreement] data must be able to provide a mutable [Sampler]
	fn sampler_mut(
		&mut self,
	) -> &mut <Binding::ResampleAgreementSpec as ResampleAgreementSpec<
		Binding::ParabyzantineAgreementBinding,
	>>::Sampler {
		self.data_mut().sampler_mut()
	}

	/// A [ResampleAgreement] data must be able to provide a [ResampleAgreementConsensusUpdate]
	fn resample_agreement_consensus_update(
		&self,
	) -> &<Binding::ResampleAgreementSpec as ResampleAgreementSpec<
		Binding::ParabyzantineAgreementBinding,
	>>::ResampleAgreementConsensusUpdate {
		self.data().resample_agreement_consensus_update()
	}

	/// A [ResampleAgreement] data must be able to provide a mutable [ResampleAgreementConsensusUpdate]
	fn resample_agreement_consensus_update_mut(
		&mut self,
	) -> &mut <Binding::ResampleAgreementSpec as ResampleAgreementSpec<
		Binding::ParabyzantineAgreementBinding,
	>>::ResampleAgreementConsensusUpdate {
		self.data_mut().resample_agreement_consensus_update_mut()
	}

	/// A [ResampleAgreement] data must be able to provide a [IndexSubcommitteeAgreementQuery]
	fn index_subcommittee_agreement_query(
		&mut self,
	) -> <Binding::ResampleAgreementSpec as ResampleAgreementSpec<
		Binding::ParabyzantineAgreementBinding,
	>>::IndexSubcommitteeAgreementQuery {
		self.data_mut().index_subcommittee_agreement_query()
	}

	/// A [ResampleAgreement] data must be able to provide a [CertificateQuery]
	fn certificate_query(
		&mut self,
		index: &(
			<<Binding::ParabyzantineAgreementBinding as ParabyzantineAgreementBinding>::Spec as ParabyzantineAgreementDataSpec>::AgreementEntity,
			<Binding::ResampleAgreementSpec as ResampleAgreementSpec<Binding::ParabyzantineAgreementBinding>>::IndexSubcommitteeAgreementBundle,
		),
	) -> <Binding::ResampleAgreementSpec as ResampleAgreementSpec<
		Binding::ParabyzantineAgreementBinding,
	>>::CertificateQuery {
		self.0.certificate_query(index)
	}
}

impl<Binding: ResampleAgreementBinding> ParabyzantineAgreement for ResampleAgreement<Binding> {
	type Binding = Binding::ParabyzantineAgreementBinding;

	fn update_parabyzantine_agreement(
		&mut self,
		agreement_world: &mut AgreementWorld<
			<Binding::ParabyzantineAgreementBinding as ParabyzantineAgreementBinding>::Spec,
		>,
	) {
		// over all the index subcommittee agreements
		let index_query = self.data_mut().index_subcommittee_agreement_query();
		for index_bundle in agreement_world.agreement_facts.query(index_query) {
			let index: <Binding::ResampleAgreementSpec as ResampleAgreementSpec<
				Binding::ParabyzantineAgreementBinding,
			>>::IndexSubcommitteeAgreement = (&index_bundle).into();

			// insert all of the certificates for this index into the certificate set
			let certificate_query = self.data_mut().certificate_query(&index_bundle);
			for certificate_bundle in agreement_world.certificate_facts.query(certificate_query) {
				let certificate: <Binding::ResampleAgreementSpec as ResampleAgreementSpec<
					Binding::ParabyzantineAgreementBinding,
				>>::Certificate = (&certificate_bundle).into();

				self.data_mut().certificate_set_mut().insert(certificate);
			}

			// check the subcommittee condition
			let subcommittee_condition = index.subcommittee().condition(
				self.data_mut()
					.certificate_set()
					.partial_subcommittees_for_index(&index.index()),
			);
			match subcommittee_condition {
				Condition::Consensus(value) => {
					// Elect the subcommittees from the consensus value
					self.data_mut().sampler_mut().elect_subcommittees_from_consensus_value(
						&value,
						&index,
						&mut agreement_world.agreement_inferences,
					);

					// Insert the ResampleAgreement consensus agreement
					self.data_mut()
						.resample_agreement_consensus_update_mut()
						.insert_resample_agreement_consensus_agreement(
							&index.index(),
							&value,
							&mut agreement_world.agreement_inferences,
						);
				}
				Condition::Hung => {
					// Elect the subcommittees from the hung value
					self.data_mut().sampler_mut().elect_subcommittees_from_hung_value(
						&index,
						&mut agreement_world.agreement_inferences,
					);
				}
				Condition::InProgress => {
					// In progress does not need to do anything
					// In the future, we may want to add a hook for in progress.
					// But for now, we keep the semantics stricter.
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
		agreement_data: &<Binding::ParabyzantineAgreementBinding as ParabyzantineAgreementBinding>::Data,
	) {
		let mut agreement_world = agreement_data.parabyzantine_agreement_world();

		self.update_parabyzantine_agreement(&mut agreement_world);
	}
}

/// A [ResampleAgreementBinding] for the [NoOp] struct.
impl ResampleAgreementBinding for NoOp {
	type ParabyzantineAgreementBinding = NoOp;
	type ResampleAgreementSpec = NoOp;
	type ResampleAgreementData = NoOpData;
}

#[cfg(test)]
mod tests {
	use super::*;
	use parabyzantine::parabyzantine::{
		agreement::Agreement,
		hart::{Parabyzantine, SystemSpec},
	};

	#[test]
	fn test_noop_resample_agreement_noops() {
		let resample_agreement = ResampleAgreement::<NoOp>(NoOpData::new());
		let mut parabyzantine: Parabyzantine<SystemSpec<Agreement, NoOp, ResampleAgreement<NoOp>>> =
			Parabyzantine { data: NoOpData::new(), agreement_handler: resample_agreement };
		parabyzantine.update(Agreement);
	}
}
