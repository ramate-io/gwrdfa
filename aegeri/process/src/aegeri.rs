pub mod parabyzantine_data;
use parabyzantine::agreement::ParabyzantineAgreementData;
pub use parabyzantine_data::AegeriParabyzantineData;

use crate::message_in::AegeriMessageIn;
use crate::message_out::AegeriMessageOut;
use crate::task::{AegeriTask, AegeriTaskError};
use aegeri_message::{
	AegeriSubcommittee, Index as AegeriIndex, Proposal as AegeriProposal, PublicKey, UnifiedMessage,
};
use gossamer::{
	container::GossamerContainer, hart::gossamer_messages::GossamerMessages, hart::GossamerHart,
	Gossamer, GossamerConfig, GossamerConfigError, GossamerTaskError, Multiaddr, Out,
};
use gwrdfa_container::{
	query::matching_tuple::{MatchingTuple, MatchingTupleQuery},
	ContainerEntity,
};
use gwrdfa_resample::agreement::{
	std::{join_set_committee::JoinSetCommittee, Index, MemoryAgreementData, Subcom},
	ResampleAgreement,
};
use parabyzantine::{act::Act, agreement::Agreement, hart::Hart, task::Task};
use parabyzantine::{message_in::MessageIn, message_out::MessageOut};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

/// A [AegeriHart] is a [Hart] that implements the Aegeri protocol.
pub struct AegeriHart {
	/// This is where the parbyzantine data will go.
	data: AegeriParabyzantineData,

	/// Message protocol is gossamer messages over [UnifiedMessage].
	message: GossamerHart<AegeriParabyzantineData, AegeriGossamerMessages>,

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

impl AegeriHart {
	pub fn mock() -> Result<
		(
			Self,
			UnboundedSender<Vec<u8>>,
			UnboundedReceiver<(ContainerEntity, Vec<u8>)>,
			UnboundedSender<Result<ContainerEntity, (ContainerEntity, GossamerTaskError)>>,
		),
		AegeriHartError,
	> {
		let (
			gossamer,
			message_into_gossamer_sender,
			entity_message_from_gossamer_receiver,
			entity_into_gossamer_sender,
		) = Gossamer::<ContainerEntity>::mock();

		Ok((
			Self {
				data: AegeriParabyzantineData::new(),
				message: GossamerHart::new(gossamer, AegeriGossamerMessages),
				agreement: ResampleAgreement::new(MemoryAgreementData::new()),
				task: AegeriTask::new(100)?,
				message_in: AegeriMessageIn,
				message_out: AegeriMessageOut::default(),
			},
			message_into_gossamer_sender,
			entity_message_from_gossamer_receiver,
			entity_into_gossamer_sender,
		))
	}

	pub async fn spawn_tokio(config: GossamerConfig) -> Result<(Self, Multiaddr), AegeriHartError> {
		let (gossamer, listen_addr) = Gossamer::spawn_tokio(config).await?;

		Ok((
			Self {
				data: AegeriParabyzantineData::new(),
				message: GossamerHart::new(gossamer, AegeriGossamerMessages),
				agreement: ResampleAgreement::new(MemoryAgreementData::new()),
				task: AegeriTask::new(100)?,
				message_in: AegeriMessageIn,
				message_out: AegeriMessageOut::default(),
			},
			listen_addr,
		))
	}

	/// Registers the genesis subcommittee for the agreement.
	pub fn with_genesis_subcommittee(mut self, subcommittee: AegeriSubcommittee) -> Self {
		let mut agreement_world = self.data.parabyzantine_agreement_world();

		// Clear out any existing agreements for the genesis index.
		for (entity, (_index, _subcommittee)) in agreement_world
			.agreement_facts
			.query(MatchingTuple::<(Index<AegeriIndex>, Subcom<AegeriSubcommittee>)>::new())
		{
			agreement_world.agreement_inferences.remove_entity(entity);
		}

		// Insert the new genesis subcommittee.
		agreement_world
			.agreement_inferences
			.insert(None, (Index::new(AegeriIndex::genesis()), Subcom::new(subcommittee)));

		self.data.commit_parabyzantine_agreement(agreement_world.into());
		self
	}

	pub fn tick(self) -> Self {
		let Self {
			mut data,
			mut message,
			mut agreement,
			mut task,
			mut message_in,
			mut message_out,
		} = self;

		message_in.act(MessageIn, &mut data);
		message.act(Hart, &mut data);
		agreement.act(Agreement, &mut data);
		task.act(Task, &mut data);
		message_out.act(MessageOut, &mut data);
		Self { data, message, agreement, task, message_in, message_out }
	}

	pub fn run(mut self) {
		loop {
			self = self.tick();
		}
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
