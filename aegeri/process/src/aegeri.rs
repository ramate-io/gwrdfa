pub mod parabyzantine_data;
pub use parabyzantine_data::AegeriParabyzantineData;

use crate::task::{AegeriTask, AegeriTaskError};
use aegeri_message::{
	AegeriSubcommittee, Index as AegeriIndex, Proposal as AegeriProposal, UnifiedMessage,
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
	std::{ConstantCommittee, MemoryAgreementData},
	ResampleAgreement,
};
use parabyzantine::{act::Act, agreement::Agreement, hart::Hart, task::Task};
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
		MemoryAgreementData<AegeriIndex, AegeriProposal, AegeriSubcommittee, ConstantCommittee>,
	>,

	/// Task protocol is aegeri task.
	task: AegeriTask,
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
			},
			listen_addr,
		))
	}

	pub fn tick(self) -> Self {
		let Self { mut data, mut message, mut agreement, mut task } = self;
		message.act(Hart, &mut data);
		agreement.act(Agreement, &mut data);
		task.act(Task, &mut data);
		Self { data, message, agreement, task }
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
