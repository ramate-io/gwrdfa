use super::super::AegeriHart;
use aegeri_message::{
	AegeriSubcommittee, Availability, Certificate, Proposal as AegeriProposal, PublicKey,
	UnifiedMessage,
};
use gossamer::GossamerMessage;
use ml_dsa::{MlDsa44, SigningKey, B32};

#[test]
fn test_hart_pings_broadcast_subcommittee_certificate() -> Result<(), anyhow::Error> {
	let signer = SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![1; 32]));
	let signer_public_key = PublicKey::new(&signer);
	let genesis_subcommittee =
		AegeriSubcommittee::genesis().with_members(vec![signer_public_key].into_iter());

	let (mut hart, mut gossamer_channels) = AegeriHart::mock()?;
	hart = hart
		.with_signer(signer)
		.with_loopback(false)
		.with_pings(true)
		.with_genesis(genesis_subcommittee.clone(), Availability::genesis());

	hart.tick();

	let mut saw_subcommittee_broadcast = false;
	while let Ok((_entity, bytes)) = gossamer_channels.entity_message_from_gossamer_receiver.try_recv() {
		let unified = UnifiedMessage::from_gossamer_bytes(bytes)?;
		let UnifiedMessage::Certificate(message) = unified else {
			continue;
		};
		let certificate: &Certificate = message.payload();
		if let AegeriProposal::SubcommitteeBroadcast(subcommittee) = certificate.value() {
			assert_eq!(
				certificate.index(),
				subcommittee.index(),
				"ping certificate index should match the broadcast subcommittee index"
			);
			assert_eq!(
				subcommittee.senders().collect::<std::collections::BTreeSet<_>>(),
				genesis_subcommittee.senders().collect::<std::collections::BTreeSet<_>>(),
				"ping should broadcast the observed subcommittee membership"
			);
			saw_subcommittee_broadcast = true;
		}
	}

	assert!(
		saw_subcommittee_broadcast,
		"expected at least one outbound SubcommitteeBroadcast ping certificate"
	);
	Ok(())
}
