use crate::aegeri::AegeriParabyzantineData;
use aegeri_message::UnifiedMessage;
use gossamer::{GossamerMessageError, In};
use gwrdfa_container::query::matching_tuple::{MatchingTuple, MatchingTupleQuery};
use gwrdfa_resample::{
	agreement::std::{Index as ResampleIndex, Subcom, Value as ResampleValue},
	ForResample,
};
use parabyzantine::message_in::{MessageInWorld, ParabyzantineMessageIn};

/// Aegeri inbound message splitter.
///
/// Verifies incoming `UnifiedMessage`s from gossamer and routes:
/// - transaction envelopes -> transaction buffer
/// - certificate envelopes -> certificate buffer (resample format)
pub struct AegeriMessageIn;

impl ParabyzantineMessageIn<AegeriParabyzantineData> for AegeriMessageIn {
	fn compute_parabyzantine_message_in(
		&mut self,
		data: &mut MessageInWorld<AegeriParabyzantineData>,
	) {
		for (entity, (In, message)) in
			data.message_facts.query(MatchingTuple::<(In, UnifiedMessage)>::new())
		{
			match message {
				UnifiedMessage::Transaction(message) => match message.clone().into_verified() {
					Ok(verified) => {
						data.transaction_inferences.insert(None, verified);
					}
					Err(e) => {
						data.message_inferences.insert(
							Some(entity),
							GossamerMessageError::InternalError(e.to_string()),
						);
					}
				},
				UnifiedMessage::Certificate(message) => match message.clone().into_verified() {
					Ok(verified) => {
						let (index, subcommittee, proposal) = verified.into_consensus_parts();
						data.certificate_inferences.insert(
							None,
							(
								ForResample,
								ResampleIndex::new(index),
								ResampleValue::new(proposal),
								Subcom::new(subcommittee),
							),
						);
					}
					Err(e) => {
						data.message_inferences.insert(
							Some(entity),
							GossamerMessageError::InternalError(e.to_string()),
						);
					}
				},
			}

			// Avoid reprocessing the same inbound record.
			data.message_inferences.remove::<In>(entity);
		}
	}
}
