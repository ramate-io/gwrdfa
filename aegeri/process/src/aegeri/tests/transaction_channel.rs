use super::super::AegeriHart;
use aegeri_message::{
	AegeriSubcommittee, Availability, Index as AegeriIndex, IndexValue, Message, Nonce,
	Transaction, TransactionSet, UnifiedMessage,
};
use gossamer::GossamerMessage;
use ml_dsa::{MlDsa44, SigningKey, B32};
use tokio::sync::mpsc::unbounded_channel;

fn tx_message(seed: u8, nonce: &[u8]) -> Result<Message<Transaction>, anyhow::Error> {
	let signer = SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![seed; 32]));
	Ok(Message::<Transaction>::try_new(&signer, Transaction::Join, Nonce::new(nonce))?)
}

#[test]
fn test_hart_broadcasts_transaction_from_task_channel() -> Result<(), anyhow::Error> {
	let (tx_sender, tx_receiver) = unbounded_channel();
	let (status_sender, _status_receiver) = unbounded_channel();
	let (mut hart, mut gossamer_channels) = AegeriHart::mock()?;
	hart = hart
		.with_loopback(false)
		.with_pings(false)
		.with_is_participant(false)
		.with_broadcast_transaction_receiver(tx_receiver)
		.with_transaction_status_sender(status_sender);

	let tx = tx_message(1, b"broadcast")?;
	let tx_id = *tx.id();
	tx_sender.send(tx)?;

	hart.tick();

	let mut saw_transaction = false;
	while let Ok((_entity, bytes)) = gossamer_channels.entity_message_from_gossamer_receiver.try_recv() {
		let unified = UnifiedMessage::from_gossamer_bytes(bytes)?;
		if let UnifiedMessage::Transaction(message) = unified {
			assert_eq!(*message.id(), tx_id);
			saw_transaction = true;
		}
	}

	assert!(saw_transaction, "expected outbound transaction message");
	Ok(())
}

#[test]
fn test_hart_sends_transaction_status_when_agreement_contains_inflight_id() -> Result<(), anyhow::Error> {
	let (tx_sender, tx_receiver) = unbounded_channel();
	let (status_sender, mut status_receiver) = unbounded_channel();
	let (mut hart, _gossamer_channels) = AegeriHart::mock()?;

	let signer = SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![9; 32]));
	let signer_public_key = aegeri_message::PublicKey::new(&signer);
	let genesis_subcommittee =
		AegeriSubcommittee::genesis().with_members(vec![signer_public_key].into_iter());

	let tx = tx_message(3, b"status")?;
	let tx_id = *tx.id();
	let mut txs = TransactionSet::new();
	txs.add_id(tx_id);
	let availability = Availability::from_transactions(txs);

	hart = hart
		.with_loopback(false)
		.with_pings(false)
		.with_is_participant(false)
		.with_genesis(genesis_subcommittee, availability)
		.with_broadcast_transaction_receiver(tx_receiver)
		.with_transaction_status_sender(status_sender);

	tx_sender.send(tx)?;
	hart.tick();

	let (status_index, status_id) = status_receiver.try_recv()?;
	assert_eq!(status_index, AegeriIndex::Availability(IndexValue::genesis()));
	assert_eq!(status_id, tx_id);
	Ok(())
}
