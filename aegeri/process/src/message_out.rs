use crate::aegeri::AegeriParabyzantineData;
use aegeri_message::{Index, IndexValue, Message, Nonce, Proposal, UnifiedMessage};
use gossamer::{GossamerMessageError, Out};
use gwrdfa_container::query::matching_components::MatchingComponents;
use ml_dsa::{MlDsa44, SigningKey, B32};
use parabyzantine::message_out::{MessageOutWorld, ParabyzantineMessageOut};

/// Aegeri outbound signer/wrapper for task certificates.
pub struct AegeriMessageOut {
	signer: SigningKey<MlDsa44>,
	nonce_counter: u64,
}

impl AegeriMessageOut {
	pub fn from_seed(seed: [u8; 32]) -> Self {
		let signer = SigningKey::<MlDsa44>::from_seed(&B32::from_iter(seed));
		Self { signer, nonce_counter: 0 }
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
		for (entity, proposal) in data.task_facts.query(MatchingComponents::<Proposal>::new()) {
			// Task buffer stores proposals; until index is carried with tasks, we wrap
			// with a placeholder transition index for signing/broadcast.
			let certificate = aegeri_message::Certificate::new(
				Index::Transition(IndexValue(0)),
				proposal.clone(),
			);

			let nonce = self.next_nonce();
			match Message::<aegeri_message::Certificate>::try_new(&self.signer, certificate, nonce)
			{
				Ok(message) => {
					data.message_inferences
						.insert(None, (Out, UnifiedMessage::Certificate(message)));
					// Consume task once emitted.
					data.task_inferences.remove_entity(entity);
				}
				Err(e) => {
					data.message_inferences
						.insert(None, GossamerMessageError::InternalError(e.to_string()));
				}
			}
		}
	}
}
