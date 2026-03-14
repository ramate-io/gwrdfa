pub mod parabyzantine_data;
use gwrdfa_container::query::matching_components::MatchingComponents;
use parabyzantine::agreement::ParabyzantineAgreementData;
pub use parabyzantine_data::AegeriParabyzantineData;

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
	ForResample, Resample,
};
use ml_dsa::{MlDsa44, SigningKey};
use parabyzantine::{act::Act, agreement::Agreement, hart::Hart, task::Task};
use parabyzantine::{message_in::MessageIn, message_out::MessageOut};

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
	pub fn mock() -> Result<(Self, GossamerChannels<ContainerEntity>), AegeriHartError> {
		let (gossamer, gossamer_channels) = Gossamer::<ContainerEntity>::mock();

		Ok((
			Self {
				data: AegeriParabyzantineData::new(),
				message: GossamerHart::new(gossamer, AegeriGossamerMessages),
				agreement: ResampleAgreement::new(MemoryAgreementData::new()),
				task: AegeriTask::new(100)?,
				message_in: AegeriMessageIn,
				message_out: AegeriMessageOut::default(),
			},
			gossamer_channels,
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

	/// With signer.
	pub fn with_signer(mut self, signer: SigningKey<MlDsa44>) -> Self {
		self.message_out = self.message_out.with_signer(signer);
		self
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

	pub fn tick(self) -> Self {
		let Self {
			mut data,
			mut message,
			mut agreement,
			mut task,
			mut message_in,
			mut message_out,
		} = self;

		message.act(Hart, &mut data);
		message_in.act(MessageIn, &mut data);
		agreement.act(Agreement, &mut data);
		task.act(Task, &mut data);
		message_out.act(MessageOut, &mut data);
		message.act(Hart, &mut data);

		Self { data, message, agreement, task, message_in, message_out }
	}

	pub fn run(mut self) {
		loop {
			self = self.tick();
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
	) -> impl Iterator<
		Item = (
			ContainerEntity,
			(&Agreement, &Resample, &Index<AegeriIndex>, &Value<AegeriProposal>),
		),
	> {
		self.data
			.parabyzantine_agreement_world()
			.agreement_facts
			.query(MatchingTuple::<(
			Agreement,
			Resample,
			Index<AegeriIndex>,
			Value<AegeriProposal>,
		)>::new())
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
mod tests {
	use super::*;
	use aegeri_message::{
		BlockHeader, Confirmation, IndexValue, Message, Nonce, Transaction, Transition,
		UnifiedMessage,
	};
	use gossamer::GossamerMessage;
	use ml_dsa::{SigningKey, B32};
	use std::collections::BTreeSet;

	#[test]
	fn test_aegeri_hart_with_genesis_subcommittee() -> Result<(), AegeriHartError> {
		let (hart, _gossamer_channels) = AegeriHart::mock()?;
		let subcommittee = AegeriSubcommittee::genesis();
		let availability = Availability::genesis();
		let hart = hart.with_genesis(subcommittee, availability);
		assert_eq!(hart.index_subcommittee_agreements().count(), 1);
		Ok(())
	}

	#[tokio::test]
	async fn test_aegeri_hart_trivial_consensus() -> Result<(), anyhow::Error> {
		let signer = SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![1; 32]));
		let signer_public_key = PublicKey::new(&signer);
		let genesis_subcommittee =
			AegeriSubcommittee::genesis().with_members(vec![signer_public_key].into_iter());
		let availability = Availability::genesis();

		let (hart, mut gossamer_channels) = AegeriHart::mock()?;
		let hart = hart.with_genesis(genesis_subcommittee, availability);

		// Send in a bunch of transactions
		for i in 0..10 {
			let transaction_signer =
				SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![i as u8; 32]));
			let transaction = Transaction::Leave;
			let message: UnifiedMessage = Message::<Transaction>::try_new(
				&transaction_signer,
				transaction,
				Nonce::new([i as u8; 32]),
			)?
			.into();
			gossamer_channels
				.message_into_gossamer_sender
				.send(message.to_gossamer_bytes()?)?;
		}

		let hart = hart.tick();
		// Simulate broadcasting
		// loop back the message
		let (entity, single_hart_cert_bytes) =
			gossamer_channels.entity_message_from_gossamer_receiver.try_recv()?;
		// Confirm the sending
		gossamer_channels.entity_into_gossamer_sender.send(Ok(entity))?;
		// Loop back the certificate
		gossamer_channels.message_into_gossamer_sender.send(single_hart_cert_bytes)?;

		// See if it generates a confirmation certificate
		let hart = hart.tick();
		let certificates = hart
			.certificates()
			.map(|(_, (_, index, value))| (index.0.clone(), value.0.clone()))
			.collect::<BTreeSet<_>>();
		let expected_certificates = BTreeSet::from_iter(vec![
			(
				AegeriIndex::Availability(IndexValue::genesis()),
				AegeriProposal::Availability(Availability::genesis()),
			),
			(
				AegeriIndex::Confirmation(IndexValue::genesis()),
				AegeriProposal::Confirmation(Confirmation::genesis()),
			),
		]);
		assert_eq!(certificates, expected_certificates);

		// Simulate broadcasting
		// loop back the message
		let (entity, single_hart_cert_bytes) =
			gossamer_channels.entity_message_from_gossamer_receiver.try_recv()?;
		// Confirm the sending
		gossamer_channels.entity_into_gossamer_sender.send(Ok(entity))?;
		// Loop back the certificate
		gossamer_channels.message_into_gossamer_sender.send(single_hart_cert_bytes)?;

		// See if it generates a block header certificate
		let hart = hart.tick();
		let certificates = hart
			.certificates()
			.map(|(_, (_, index, value))| (index.0.clone(), value.0.clone()))
			.collect::<BTreeSet<_>>();

		// Currently, there isn't any certificate eviction.
		// So we expect the confirmation certificate to still be present.
		let expected_certificates = BTreeSet::from_iter(vec![
			(
				AegeriIndex::Availability(IndexValue::genesis()),
				AegeriProposal::Availability(Availability::genesis()),
			),
			(
				AegeriIndex::Confirmation(IndexValue::genesis()),
				AegeriProposal::Confirmation(Confirmation::genesis()),
			),
			(
				AegeriIndex::Block(IndexValue::genesis()),
				AegeriProposal::BlockHeader(BlockHeader::genesis()),
			),
		]);
		assert_eq!(certificates, expected_certificates);

		// TOOD: we should not be rebroadcasting the same certificates over and over again
		// Simulate broadcasting
		// loop back the message
		/*while let Ok((entity, single_hart_cert_bytes)) =
			gossamer_channels.entity_message_from_gossamer_receiver.try_recv()
		{
			// Confirm the sending
			gossamer_channels.entity_into_gossamer_sender.send(Ok(entity))?;
			// Loop back the certificate
			gossamer_channels
				.message_into_gossamer_sender
				.send(single_hart_cert_bytes.clone())?;
		}*/
		let (entity, single_hart_cert_bytes) =
			gossamer_channels.entity_message_from_gossamer_receiver.try_recv()?;
		// Confirm the sending
		gossamer_channels.entity_into_gossamer_sender.send(Ok(entity))?;
		// Loop back the certificate
		gossamer_channels.message_into_gossamer_sender.send(single_hart_cert_bytes)?;

		// See if it generates a transition certificate
		let hart = hart.tick();
		let certificates = hart
			.certificates()
			.map(|(_, (_, index, value))| (index.0.clone(), value.0.clone()))
			.collect::<BTreeSet<_>>();
		let expected_certificates = BTreeSet::from_iter(vec![
			(
				AegeriIndex::Availability(IndexValue::genesis()),
				AegeriProposal::Availability(Availability::genesis()),
			),
			(
				AegeriIndex::Confirmation(IndexValue::genesis()),
				AegeriProposal::Confirmation(Confirmation::genesis()),
			),
			(
				AegeriIndex::Block(IndexValue::genesis()),
				AegeriProposal::BlockHeader(BlockHeader::genesis()),
			),
			(
				AegeriIndex::Transition(IndexValue::genesis()),
				AegeriProposal::Transition(Transition::genesis()),
			),
		]);
		assert_eq!(certificates, expected_certificates);

		Ok(())
	}
}
