use super::{AegeriHart, AegeriHartError};
use aegeri_message::{
	AegeriSubcommittee, Availability, Id, Index as AegeriIndex, Message, Nonce,
	Proposal as AegeriProposal, PublicKey, Transaction, TransactionSet, UnifiedMessage,
};
use gossamer::{GossamerChannels, GossamerMessage};
use gwrdfa_container::ContainerEntity;
use ml_dsa::{MlDsa44, SigningKey, B32};
use std::collections::BTreeSet;

pub(crate) struct TrivialConsensusHarness {
	pub hart: AegeriHart,
	pub gossamer_channels: GossamerChannels<ContainerEntity>,
	pub genesis_subcommittee: AegeriSubcommittee,
}

pub(crate) fn setup_trivial_consensus_harness(seed: u8) -> Result<TrivialConsensusHarness, AegeriHartError> {
	let signer = SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![seed; 32]));
	let signer_public_key = PublicKey::new(&signer);
	let genesis_subcommittee =
		AegeriSubcommittee::genesis().with_members(vec![signer_public_key].into_iter());
	let availability = Availability::genesis();

	let (hart, gossamer_channels) = AegeriHart::mock()?;
	let hart = hart.with_genesis(genesis_subcommittee.clone(), availability);

	Ok(TrivialConsensusHarness { hart, gossamer_channels, genesis_subcommittee })
}

pub(crate) fn loopback_single_outbound_message(
	gossamer_channels: &mut GossamerChannels<ContainerEntity>,
) -> Result<(), anyhow::Error> {
	let (entity, single_hart_cert_bytes) =
		gossamer_channels.entity_message_from_gossamer_receiver.try_recv()?;
	gossamer_channels.entity_into_gossamer_sender.send(Ok(entity))?;
	gossamer_channels.message_into_gossamer_sender.send(single_hart_cert_bytes)?;
	Ok(())
}

pub(crate) fn advance_consensus_step(
	hart: &mut AegeriHart,
	gossamer_channels: &mut GossamerChannels<ContainerEntity>,
) -> Result<(), anyhow::Error> {
	hart.tick();
	loopback_single_outbound_message(gossamer_channels)?;
	hart.tick();
	Ok(())
}

pub(crate) fn send_transaction(
	gossamer_channels: &mut GossamerChannels<ContainerEntity>,
	seed: u8,
	payload: Transaction,
	nonce: [u8; 32],
) -> Result<Id, anyhow::Error> {
	let signer = SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![seed; 32]));
	let message = Message::<Transaction>::try_new(&signer, payload, Nonce::new(nonce))?;
	let transaction_id = *message.id();
	let unified_message: UnifiedMessage = message.into();
	gossamer_channels
		.message_into_gossamer_sender
		.send(unified_message.to_gossamer_bytes()?)?;
	Ok(transaction_id)
}

pub(crate) fn availability_from_ids(ids: impl IntoIterator<Item = Id>) -> Availability {
	let mut transactions = TransactionSet::new();
	for id in ids {
		transactions.add_id(id);
	}
	Availability::from_transactions(transactions)
}

pub(crate) fn assert_certificate_set(
	hart: &AegeriHart,
	expected: impl IntoIterator<Item = (AegeriIndex, AegeriProposal)>,
) {
	assert_eq!(
		hart.certificate_set(),
		BTreeSet::from_iter(expected),
		"unexpected certificate set"
	);
}

pub(crate) fn assert_index_subcommittee_agreement_set(
	hart: &AegeriHart,
	expected: impl IntoIterator<Item = (AegeriIndex, AegeriSubcommittee)>,
) {
	assert_eq!(
		hart.index_subcommittee_agreement_set(),
		BTreeSet::from_iter(expected),
		"unexpected index/subcommittee agreement set"
	);
}

pub(crate) fn assert_index_value_agreement_set(
	hart: &AegeriHart,
	expected: impl IntoIterator<Item = (AegeriIndex, AegeriProposal)>,
) {
	assert_eq!(
		hart.index_value_agreement_set(),
		BTreeSet::from_iter(expected),
		"unexpected index/value agreement set"
	);
}
