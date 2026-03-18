use crate::aegeri::AegeriParabyzantineData;
use aegeri_message::{Index, Message, Nonce, Proposal, Transaction, UnifiedMessage};
use gossamer::{Broadcast, GossamerMessageError, In, Out};
use gwrdfa_container::query::{
	matching_components::MatchingComponents, matching_tuple::MatchingTuple,
};
use ml_dsa::{MlDsa44, SigningKey, B32};
use parabyzantine::message_out::{MessageOutWorld, ParabyzantineMessageOut};

/// Aegeri outbound signer/wrapper for task certificates.
pub struct AegeriMessageOut {
	pub(crate) signer: SigningKey<MlDsa44>,
	nonce_counter: u64,
	loopback: bool,
}

impl AegeriMessageOut {
	pub fn new(signer: SigningKey<MlDsa44>) -> Self {
		Self { signer, nonce_counter: 0, loopback: true }
	}

	pub fn with_signer(mut self, signer: SigningKey<MlDsa44>) -> Self {
		self.signer = signer;
		self
	}

	pub fn with_nonce_counter(mut self, nonce_counter: u64) -> Self {
		self.nonce_counter = nonce_counter;
		self
	}

	pub fn with_loopback(mut self, loopback: bool) -> Self {
		self.loopback = loopback;
		self
	}

	pub fn from_seed(seed: [u8; 32]) -> Self {
		let signer = SigningKey::<MlDsa44>::from_seed(&B32::from_iter(seed));
		Self { signer, nonce_counter: 0, loopback: true }
	}

	fn next_nonce(&mut self) -> Nonce {
		let nonce = self.nonce_counter.to_le_bytes().to_vec();
		self.nonce_counter = self.nonce_counter.saturating_add(1);
		Nonce::new(nonce)
	}
}

impl Default for AegeriMessageOut {
	fn default() -> Self {
		Self::from_seed([7; 32])
	}
}

impl ParabyzantineMessageOut<AegeriParabyzantineData> for AegeriMessageOut {
	fn compute_parabyzantine_message_out(
		&mut self,
		data: &mut MessageOutWorld<AegeriParabyzantineData>,
	) {
		// Remove all of the messages that were broadcasted.
		for (entity, (Broadcast, message)) in
			data.message_facts.query(MatchingTuple::<(Broadcast, UnifiedMessage)>::new())
		{
			match message {
				UnifiedMessage::Transaction(tx) => {
					log::debug!("client: message broadcast: transaction: {:?}", tx.id());
				}
				UnifiedMessage::Certificate(cert) => {
					log::debug!("participant: message broadcast: certificate: {:?}", cert.id());
				}
			}
			data.message_inferences.remove_entity(entity);
		}

		// Emit all the certificates from the task buffer.
		for (entity, (index, proposal)) in
			data.task_facts.query(MatchingTuple::<(Index, Proposal)>::new())
		{
			// Task buffer stores proposals; until index is carried with tasks, we wrap
			// with a placeholder transition index for signing/broadcast.
			let certificate = aegeri_message::Certificate::new(index.clone(), proposal.clone());

			let nonce = self.next_nonce();
			match Message::<aegeri_message::Certificate>::try_new(&self.signer, certificate, nonce)
			{
				Ok(message) => {
					// Emit the certificate.
					data.message_inferences
						.insert(None, (Out, UnifiedMessage::Certificate(message.clone())));

					// Also automatically loop-back via Gossamer In
					if self.loopback {
						data.message_inferences
							.insert(None, (In, UnifiedMessage::Certificate(message)));
					}

					// Consume task once emitted.
					data.task_inferences.remove_entity(entity);
				}
				Err(e) => {
					data.message_inferences
						.insert(None, GossamerMessageError::InternalError(e.to_string()));
				}
			}
		}

		// Emit all the transactions from the task buffer.
		for (entity, transaction) in
			data.task_facts.query(MatchingComponents::<Message<Transaction>>::new())
		{
			data.message_inferences
				.insert(None, (Out, UnifiedMessage::Transaction(transaction.clone())));

			// Also automatically loop-back via Gossamer In
			if self.loopback {
				data.message_inferences
					.insert(None, (In, UnifiedMessage::Transaction(transaction.clone())));
			}

			// Consume transaction task once emitted.
			data.task_inferences.remove_entity(entity);
		}
	}
}
