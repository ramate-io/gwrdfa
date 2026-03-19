pub mod parabyzantine_data;
use gwrdfa_container::query::matching_components::MatchingComponents;
use parabyzantine::agreement::ParabyzantineAgreementData;
pub use parabyzantine_data::AegeriParabyzantineData;

use crate::bootstrap::Bootstrap;
use crate::message_in::AegeriMessageIn;
use crate::message_out::AegeriMessageOut;
use crate::task::{AegeriTask, AegeriTaskError};
use aegeri_message::{
	AegeriSubcommittee, Availability, Index as AegeriIndex, Proposal as AegeriProposal, PublicKey,
	UnifiedMessage,
};
use gossamer::{
	container::GossamerContainer, hart::gossamer_messages::GossamerMessages, hart::GossamerHart,
	Gossamer, GossamerChannels, GossamerConfig, GossamerConfigError, Multiaddr, Out,
};
use gwrdfa_container::{
	query::matching_tuple::{MatchingTuple, MatchingTupleQuery},
	ContainerEntity,
};
use gwrdfa_resample::{
	agreement::{
		std::{join_set_committee::JoinSetCommittee, Index, MemoryAgreementData, Subcom, Value},
		ResampleAgreement,
	},
	ForResample,
};
use ml_dsa::{MlDsa44, SigningKey};
use parabyzantine::{act::Act, agreement::Agreement, hart::Hart, task::Task};
use parabyzantine::{message_in::MessageIn, message_out::MessageOut};
use std::collections::BTreeSet;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

/// A [AegeriHart] is a [Hart] that implements the Aegeri protocol.
pub struct AegeriHart {
	/// This is where the parbyzantine data will go.
	data: AegeriParabyzantineData,

	/// Message protocol is gossamer messages over [UnifiedMessage].
	message: GossamerHart<AegeriParabyzantineData, AegeriGossamerMessages>,

	// Bootstrap agreement protocol
	bootstrap: Bootstrap,

	/// Agreement protocol is resample agreement.
	agreement: ResampleAgreement<
		AegeriParabyzantineData,
		MemoryAgreementData<AegeriIndex, AegeriProposal, AegeriSubcommittee, JoinSetCommittee>,
	>,

	/// Task protocol is aegeri task.
	task: AegeriTask,

	/// Message input protocol is aegeri message in.
	message_in: AegeriMessageIn,

	/// Message output protocol is aegeri message out.
	message_out: AegeriMessageOut,
}

#[derive(Debug, thiserror::Error)]
pub enum AegeriHartError {
	#[error("task error: {0}")]
	Task(#[from] AegeriTaskError),
	#[error("gossamer error: {0}")]
	Gossamer(#[from] GossamerConfigError),
}

#[derive(Debug, Clone, Copy)]
pub struct AegeriBufferSizes {
	pub messages: usize,
	pub transaction: usize,
	pub certificate: usize,
	pub agreement: usize,
	pub task: usize,
}

impl AegeriHart {
	pub fn from_gossamer(gossamer: Gossamer<ContainerEntity>) -> Result<Self, AegeriHartError> {
		Ok(Self {
			data: AegeriParabyzantineData::new(),
			message: GossamerHart::new(gossamer, AegeriGossamerMessages),
			bootstrap: Bootstrap::new(),
			agreement: ResampleAgreement::new(MemoryAgreementData::new()),
			task: AegeriTask::new(100)?,
			message_in: AegeriMessageIn,
			message_out: AegeriMessageOut::default(),
		})
	}

	/// Replaces the underlying gossamer transport used by this hart.
	pub fn with_gossamer(mut self, gossamer: Gossamer<ContainerEntity>) -> Self {
		self.message = GossamerHart::new(gossamer, AegeriGossamerMessages);
		self
	}

	pub fn mock() -> Result<(Self, GossamerChannels<ContainerEntity>), AegeriHartError> {
		let (gossamer, gossamer_channels) = Gossamer::<ContainerEntity>::mock();

		Ok((Self::from_gossamer(gossamer)?, gossamer_channels))
	}

	pub async fn spawn_tokio(config: GossamerConfig) -> Result<(Self, Multiaddr), AegeriHartError> {
		let (gossamer, listen_addr) = Gossamer::spawn_tokio(config).await?;

		Ok((Self::from_gossamer(gossamer)?, listen_addr))
	}

	/// With signer.
	pub fn with_signer(mut self, signer: SigningKey<MlDsa44>) -> Self {
		self.message_out = self.message_out.with_signer(signer);
		self
	}

	/// With bootstrap configuration.
	pub fn with_bootstrap(mut self, bootstrap: Bootstrap) -> Self {
		self.bootstrap = bootstrap;
		self
	}

	/// Configure whether bootstrap has already completed.
	pub fn with_bootstrapped(mut self, bootstrapped: bool) -> Self {
		self.bootstrap = self.bootstrap.with_bootstrapped(bootstrapped);
		self
	}

	/// Configure required peers for bootstrap gate.
	pub fn with_bootstrap_peer_count_required(mut self, peer_count_required: usize) -> Self {
		self.bootstrap = self.bootstrap.with_peer_count_required(peer_count_required);
		self
	}

	/// Configure bootstrap peer allowlist.
	pub fn with_bootstrap_peers(
		mut self,
		bootstrap_peers: impl IntoIterator<Item = PublicKey>,
	) -> Self {
		self.bootstrap = self.bootstrap.with_bootstrap_peers(bootstrap_peers);
		self
	}

	/// Whether this hart has completed bootstrap.
	pub fn has_bootstrapped(&self) -> bool {
		self.bootstrap.has_bootstrapped()
	}

	/// Gets the public key of the signer.
	pub fn signer_public_key(&self) -> PublicKey {
		PublicKey::new(&self.message_out.signer)
	}

	/// With nonce counter.
	pub fn with_nonce_counter(mut self, nonce_counter: u64) -> Self {
		self.message_out = self.message_out.with_nonce_counter(nonce_counter);
		self
	}

	/// With loopback.
	pub fn with_loopback(mut self, loopback: bool) -> Self {
		self.message_out = self.message_out.with_loopback(loopback);
		self
	}

	/// Enable or disable subcommittee ping broadcasts from task.
	pub fn with_pings(mut self, pings: bool) -> Self {
		self.task = self.task.with_pings(pings);
		self
	}

	/// Enable or disable consensus participation logic.
	pub fn with_is_participant(mut self, is_participant: bool) -> Self {
		self.task = self.task.with_is_participant(is_participant);
		self
	}

	/// Wire an external transaction broadcast receiver into task.
	pub fn with_broadcast_transaction_receiver(
		mut self,
		broadcast_transaction_receiver: UnboundedReceiver<
			aegeri_message::Message<aegeri_message::Transaction>,
		>,
	) -> Self {
		self.task = self.task.with_broadcast_transaction_receiver(broadcast_transaction_receiver);
		self
	}

	/// Wire an external transaction status sender into task.
	pub fn with_transaction_status_sender(
		mut self,
		transaction_status_sender: UnboundedSender<(AegeriIndex, aegeri_message::Id)>,
	) -> Self {
		self.task = self.task.with_transaction_status_sender(transaction_status_sender);
		self
	}

	/// Control whether dropped transaction channels are reported as task errors.
	pub fn with_report_transaction_channel_errors(
		mut self,
		report_transaction_channel_errors: bool,
	) -> Self {
		self.task = self
			.task
			.with_report_transaction_channel_errors(report_transaction_channel_errors);
		self
	}

	/// Registers the genesis subcommittee for the agreement.
	pub fn with_genesis(
		mut self,
		genesis_subcommittee: AegeriSubcommittee,
		genesis_availability_agreement: Availability,
	) -> Self {
		let mut agreement_world = self.data.parabyzantine_agreement_world();

		// Clear out any existing agreements for the genesis index.
		for (entity, _index) in agreement_world
			.agreement_facts
			.query(MatchingComponents::<Index<AegeriIndex>>::new())
		{
			agreement_world.agreement_inferences.remove_entity(entity);
		}

		// Insert the new genesis subcommittee.
		agreement_world.agreement_inferences.insert(
			None,
			(Index::new(AegeriIndex::genesis()), Subcom::new(genesis_subcommittee.clone())),
		);

		// Insert the new genesis availability certificate.
		agreement_world.certificate_inferences.insert(
			None,
			(
				ForResample,
				Index::new(AegeriIndex::genesis()),
				Value::new(AegeriProposal::Availability(genesis_availability_agreement)),
				Subcom::new(genesis_subcommittee),
			),
		);

		self.data.commit_parabyzantine_agreement(agreement_world.into());
		self
	}

	pub fn mempool_slot_width_ms(&self) -> u64 {
		self.task.slot_width_ms()
	}

	pub fn tick(&mut self) {
		log::debug!("\n\n===\nTICKING AEGERI HART: {}\n====", self.signer_public_key());
		self.message.act(Hart, &mut self.data);
		self.message_in.act(MessageIn, &mut self.data);
		self.bootstrap.act(Agreement, &mut self.data);
		self.agreement.act(Agreement, &mut self.data);
		self.task.act(Task, &mut self.data);
		self.message_out.act(MessageOut, &mut self.data);
		self.message.act(Hart, &mut self.data);
	}

	pub fn run(&mut self) {
		loop {
			self.tick();
		}
	}

	pub fn buffer_sizes(&self) -> AegeriBufferSizes {
		AegeriBufferSizes {
			messages: self.data.messages.len(),
			transaction: self.data.transactions.len(),
			certificate: self.data.certificates.len(),
			agreement: self.data.agreements.len(),
			task: self.data.tasks.len(),
		}
	}

	pub fn index_subcommittee_agreements(
		&self,
	) -> impl Iterator<Item = (ContainerEntity, (&Index<AegeriIndex>, &Subcom<AegeriSubcommittee>))>
	{
		self.data
			.parabyzantine_agreement_world()
			.agreement_facts
			.query(MatchingTuple::<(Index<AegeriIndex>, Subcom<AegeriSubcommittee>)>::new())
	}

	pub fn index_value_agreements(
		&self,
	) -> impl Iterator<Item = (ContainerEntity, (&Agreement, &Index<AegeriIndex>, &Value<AegeriProposal>))>
	{
		self.data
			.parabyzantine_agreement_world()
			.agreement_facts
			.query(MatchingTuple::<(Agreement, Index<AegeriIndex>, Value<AegeriProposal>)>::new())
	}

	pub fn certificates(
		&self,
	) -> impl Iterator<
		Item = (ContainerEntity, (&ForResample, &Index<AegeriIndex>, &Value<AegeriProposal>)),
	> {
		self.data
			.parabyzantine_agreement_world()
			.certificate_facts
			.query(MatchingTuple::<(ForResample, Index<AegeriIndex>, Value<AegeriProposal>)>::new())
	}

	/// Convenience view of certificates as `(index, proposal)` pairs.
	pub fn certificate_set(&self) -> BTreeSet<(AegeriIndex, AegeriProposal)> {
		self.certificates()
			.map(|(_, (_, index, value))| (index.0.clone(), value.0.clone()))
			.collect()
	}

	/// Convenience view of index/subcommittee agreements as pairs.
	pub fn index_subcommittee_agreement_set(&self) -> BTreeSet<(AegeriIndex, AegeriSubcommittee)> {
		self.index_subcommittee_agreements()
			.map(|(_, (index, subcommittee))| (index.0.clone(), subcommittee.0.clone()))
			.collect()
	}

	/// Convenience view of index/value agreements as `(index, proposal)` pairs.
	pub fn index_value_agreement_set(&self) -> BTreeSet<(AegeriIndex, AegeriProposal)> {
		self.index_value_agreements()
			.map(|(_, (_agreement, index, proposal))| (index.0.clone(), proposal.0.clone()))
			.collect()
	}
}

pub struct AegeriGossamerMessages;

impl GossamerMessages<AegeriParabyzantineData> for AegeriGossamerMessages {
	type Message = UnifiedMessage;
	type OutQuery<'a> =
		MatchingTupleQuery<'a, GossamerContainer<UnifiedMessage>, (Out, UnifiedMessage)>;
	type OutQueryPlan = MatchingTuple<(Out, UnifiedMessage)>;

	fn gossamer_messages_out_plan(&mut self) -> MatchingTuple<(Out, UnifiedMessage)> {
		MatchingTuple::new()
	}
}

#[cfg(test)]
mod test_utils;
#[cfg(test)]
mod tests;
