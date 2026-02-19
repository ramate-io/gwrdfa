pub mod gossamer_messages;
pub mod spec;

use gossamer_messages::GossamerMessages;
pub use spec::GossamerSpec;

use crate::{Broadcast, In, InFlight, Out};
use crate::{Gossamer, GossamerMessage, GossamerMessageError};
use parabyzantine::{
	buffer::Bundle,
	hart::{
		ParabyzantineDataBinding, ParabyzantineDataSpec, ParabyzantineHart, ParabyzantineWorld,
	},
};

pub struct GossamerHart<Binding: ParabyzantineDataBinding, Spec: GossamerSpec<Binding>>
where
	GossamerMessageError: Bundle<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
	>,
	<Binding::Spec as ParabyzantineDataSpec>::MessageEntity: Send + Sync + 'static,
{
	messages: Spec::Messages,
	gossamer: Gossamer<<Binding::Spec as ParabyzantineDataSpec>::MessageEntity>,
	max_batch_size: usize,
}

impl<Binding: ParabyzantineDataBinding, Spec: GossamerSpec<Binding>> GossamerHart<Binding, Spec>
where
	GossamerMessageError: Bundle<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
	>,
	<Binding::Spec as ParabyzantineDataSpec>::MessageEntity: Send + Sync + 'static,
{
	pub fn new(
		gossamer: Gossamer<<Binding::Spec as ParabyzantineDataSpec>::MessageEntity>,
		messages: Spec::Messages,
	) -> Self {
		Self { messages, gossamer, max_batch_size: 256 }
	}

	pub fn with_max_batch_size(mut self, max_batch_size: usize) -> Self {
		self.max_batch_size = max_batch_size;
		self
	}
}

impl<Binding: ParabyzantineDataBinding, Spec: GossamerSpec<Binding>> ParabyzantineHart
	for GossamerHart<Binding, Spec>
where
	GossamerMessageError: Bundle<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
	>,
	<Binding::Spec as ParabyzantineDataSpec>::MessageEntity: Copy + Send + Sync + 'static,
{
	type Binding = Binding;

	fn update_parabyzantine_hart(&mut self, data: &mut ParabyzantineWorld<Binding::Spec>) {
		// Try to send messages to the swarm via gossamer
		let gossamer_query_plan = self.messages.gossamer_messages();
		for (entity, message) in data.message_facts.query(gossamer_query_plan) {
			match self.gossamer.send_message(entity, message) {
				Ok(_) => {
					data.message_inferences.remove_entity(entity);
				}
				Err(e) => {
					// Insert the error into the inferences
					data.message_inferences.insert(Some(entity), e);
				}
			}
		}

		// Try to receive up to max_batch_size messages
		for _ in 0..self.max_batch_size {
			match self.gossamer.try_recv_message::<Spec::Message>() {
				Ok(Some(message)) => {
					// Insert the message into the inferences
					data.message_inferences.insert(None, message)
				}
				Ok(None) => {
					// No message received
					break;
				}
				Err(e) => {
					// Insert the error into the inferences
					data.message_inferences.insert(None, e);
				}
			}
		}
	}
}
