use super::super::AegeriHart;
use aegeri_message::{
	AegeriSubcommittee, Certificate, Index as AegeriIndex, IndexValue, Message, Nonce,
	Proposal as AegeriProposal, PublicKey, UnifiedMessage,
};
use gossamer::GossamerMessage;
use ml_dsa::{MlDsa44, SigningKey, B32};

fn signer(seed: u8) -> SigningKey<MlDsa44> {
	SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![seed; 32]))
}

fn send_subcommittee_broadcast_from_signer(
	gossamer_channels: &mut gossamer::GossamerChannels<gwrdfa_container::ContainerEntity>,
	signer: &SigningKey<MlDsa44>,
	index: AegeriIndex,
	subcommittee: AegeriSubcommittee,
	nonce_byte: u8,
) -> Result<(), anyhow::Error> {
	let certificate = Certificate::new(index, AegeriProposal::SubcommitteeBroadcast(subcommittee));
	let message = Message::<Certificate>::try_new(signer, certificate, Nonce::new([nonce_byte; 32]))?;
	let unified_message: UnifiedMessage = message.into();
	gossamer_channels
		.message_into_gossamer_sender
		.send(unified_message.to_gossamer_bytes()?)?;
	Ok(())
}

#[test]
fn test_bootstrap_waits_until_required_peers_seen() -> Result<(), anyhow::Error> {
	let peer_a = signer(11);
	let peer_b = signer(12);
	let peer_a_pk = PublicKey::new(&peer_a);
	let peer_b_pk = PublicKey::new(&peer_b);

	let (mut hart, mut gossamer_channels) = AegeriHart::mock()?;
	hart = hart
		.with_bootstrapped(false)
		.with_bootstrap_peer_count_required(2)
		.with_bootstrap_peers(vec![peer_a_pk.clone(), peer_b_pk.clone()]);

	let bootstrap_index = AegeriIndex::Availability(IndexValue::new(42));
	let bootstrap_subcommittee = AegeriSubcommittee::new(bootstrap_index)
		.with_members(vec![peer_a_pk.clone(), peer_b_pk.clone()].into_iter());

	send_subcommittee_broadcast_from_signer(
		&mut gossamer_channels,
		&peer_a,
		bootstrap_index,
		bootstrap_subcommittee.clone(),
		1,
	)?;
	hart.tick();

	assert!(!hart.has_bootstrapped(), "bootstrap should still be waiting for second peer");
	assert!(
		!hart
			.index_subcommittee_agreement_set()
			.contains(&(bootstrap_index, bootstrap_subcommittee.clone())),
		"agreement should not be inserted before threshold"
	);

	send_subcommittee_broadcast_from_signer(
		&mut gossamer_channels,
		&peer_b,
		bootstrap_index,
		bootstrap_subcommittee.clone(),
		2,
	)?;
	hart.tick();

	assert!(hart.has_bootstrapped(), "bootstrap should complete after required peer count");
	assert!(
		hart.index_subcommittee_agreement_set()
			.contains(&(bootstrap_index, bootstrap_subcommittee)),
		"expected bootstrap index agreement to be inserted"
	);

	Ok(())
}

#[test]
fn test_bootstrap_helper_builder_preserves_allowlist_state() -> Result<(), anyhow::Error> {
	let peer_a = signer(21);
	let peer_b = signer(22);
	let peer_a_pk = PublicKey::new(&peer_a);
	let peer_b_pk = PublicKey::new(&peer_b);

	let (hart, _gossamer_channels) = AegeriHart::mock()?;
	let hart = hart
		.with_bootstrapped(false)
		.with_bootstrap_peer_count_required(2)
		.with_bootstrap_peers(vec![peer_a_pk, peer_b_pk]);

	assert!(!hart.has_bootstrapped());
	Ok(())
}
