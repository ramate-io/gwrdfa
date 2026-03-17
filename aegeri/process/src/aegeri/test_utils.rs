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

pub(crate) struct MultiHartHarness {
	pub harts: Vec<AegeriHart>,
	pub gossamer_channels: Vec<GossamerChannels<ContainerEntity>>,
	pub genesis_subcommittee: AegeriSubcommittee,
}

pub(crate) struct SentTransaction {
	pub id: Id,
	pub public_key: PublicKey,
}

pub(crate) fn setup_trivial_consensus_harness(
	seed: u8,
) -> Result<TrivialConsensusHarness, AegeriHartError> {
	let signer = SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![seed; 32]));
	let signer_public_key = PublicKey::new(&signer);
	let genesis_subcommittee =
		AegeriSubcommittee::genesis().with_members(vec![signer_public_key].into_iter());
	let availability = Availability::genesis();

	let (hart, gossamer_channels) = AegeriHart::mock()?;
	let hart = hart
		.with_genesis(genesis_subcommittee.clone(), availability)
		.with_loopback(false)
		.with_pings(false);
	Ok(TrivialConsensusHarness { hart, gossamer_channels, genesis_subcommittee })
}

pub(crate) fn setup_multi_hart_harness(
	seeds: impl IntoIterator<Item = u8>,
) -> Result<MultiHartHarness, AegeriHartError> {
	let seeds = seeds.into_iter().collect::<Vec<_>>();
	let mut members = BTreeSet::new();
	let mut signers = Vec::with_capacity(seeds.len());
	for seed in seeds {
		let signer = SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![seed; 32]));
		members.insert(PublicKey::new(&signer));
		signers.push(signer);
	}

	let genesis_subcommittee = AegeriSubcommittee::genesis().with_members(members.into_iter());
	let availability = Availability::genesis();
	let mut harts = Vec::with_capacity(signers.len());
	let mut gossamer_channels = Vec::with_capacity(signers.len());

	for signer in signers {
		let (hart, channels) = AegeriHart::mock()?;
		harts.push(
			hart.with_signer(signer)
				.with_genesis(genesis_subcommittee.clone(), availability.clone()),
		);
		gossamer_channels.push(channels);
	}

	Ok(MultiHartHarness { harts, gossamer_channels, genesis_subcommittee })
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

pub(crate) fn advance_multi_hart_consensus_step(
	harts: &mut [AegeriHart],
	gossamer_channels: &mut [GossamerChannels<ContainerEntity>],
	active: &[usize],
) -> Result<(), anyhow::Error> {
	for &i in active {
		harts[i].tick();
	}

	let mut outbound_messages = Vec::new();
	for &i in active {
		while let Ok((entity, bytes)) =
			gossamer_channels[i].entity_message_from_gossamer_receiver.try_recv()
		{
			gossamer_channels[i].entity_into_gossamer_sender.send(Ok(entity))?;
			outbound_messages.push(bytes);
		}
	}

	for &receiver in active {
		for bytes in outbound_messages.iter() {
			gossamer_channels[receiver].message_into_gossamer_sender.send(bytes.clone())?;
		}
	}

	for &i in active {
		harts[i].tick();
	}

	Ok(())
}

pub(crate) fn hart_has_certificate_index(hart: &AegeriHart, index: AegeriIndex) -> bool {
	hart.certificate_set().into_iter().any(|(candidate, _)| candidate == index)
}

pub(crate) fn assert_active_harts_have_certificate_index(
	harts: &[AegeriHart],
	active: &[usize],
	index: AegeriIndex,
) {
	for &i in active {
		let known_indexes = harts[i]
			.certificate_set()
			.into_iter()
			.map(|(candidate, _)| candidate)
			.collect::<BTreeSet<_>>();
		assert!(
			hart_has_certificate_index(&harts[i], index),
			"hart {i} does not have expected certificate index; known indexes: {known_indexes:?}"
		);
	}
}

pub(crate) fn assert_active_harts_lack_certificate_index(
	harts: &[AegeriHart],
	active: &[usize],
	index: AegeriIndex,
) {
	for &i in active {
		assert!(
			!hart_has_certificate_index(&harts[i], index),
			"hart {i} unexpectedly has certificate index"
		);
	}
}

pub(crate) fn active_harts_all_have_certificate_index(
	harts: &[AegeriHart],
	active: &[usize],
	index: AegeriIndex,
) -> bool {
	active.iter().all(|&i| hart_has_certificate_index(&harts[i], index))
}

pub(crate) fn send_transaction(
	gossamer_channels: &mut GossamerChannels<ContainerEntity>,
	seed: u8,
	payload: Transaction,
	nonce: [u8; 32],
) -> Result<Id, anyhow::Error> {
	Ok(send_transaction_with_public_key(gossamer_channels, seed, payload, nonce)?.id)
}

pub(crate) fn send_transaction_with_public_key(
	gossamer_channels: &mut GossamerChannels<ContainerEntity>,
	seed: u8,
	payload: Transaction,
	nonce: [u8; 32],
) -> Result<SentTransaction, anyhow::Error> {
	let signer = SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![seed; 32]));
	let public_key = PublicKey::new(&signer);
	let message = Message::<Transaction>::try_new(&signer, payload, Nonce::new(nonce))?;
	let transaction_id = *message.id();
	let unified_message: UnifiedMessage = message.into();
	gossamer_channels
		.message_into_gossamer_sender
		.send(unified_message.to_gossamer_bytes()?)?;
	Ok(SentTransaction { id: transaction_id, public_key })
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
	assert_eq!(hart.certificate_set(), BTreeSet::from_iter(expected), "unexpected certificate set");
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
