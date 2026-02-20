pub mod gossamer_messages;
pub mod spec;

use gossamer_messages::GossamerMessages;
pub use spec::GossamerSpec;

use crate::{Broadcast, In, InFlight, Out};
use crate::{Gossamer, GossamerMessageError};
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
	(In, Spec::Message): Bundle<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
	>,
	Out: Bundle<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
	>,
	InFlight: Bundle<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
	>,
	Broadcast: Bundle<
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
	(In, Spec::Message): Bundle<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
	>,
	Out: Bundle<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
	>,
	InFlight: Bundle<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
	>,
	Broadcast: Bundle<
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
	Out: Bundle<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
	>,
	(In, Spec::Message): Bundle<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
	>,
	InFlight: Bundle<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
	>,
	Broadcast: Bundle<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
	>,
	<Binding::Spec as ParabyzantineDataSpec>::MessageEntity: Copy + Send + Sync + 'static,
{
	type Binding = Binding;

	fn update_parabyzantine_hart(&mut self, data: &mut ParabyzantineWorld<Binding::Spec>) {
		// Check confirmations on any messages that were in flight
		for _ in 0..self.max_batch_size {
			match self.gossamer.try_recv_confirmation() {
				Ok(Some(entity)) => {
					// This message is no longer in flight...
					data.message_inferences.remove::<InFlight>(entity);
					// ...for the purpose of Gossamer, it has been broadcast.
					data.message_inferences.insert(Some(entity), Broadcast);
					// NOTE: we do not remove the entity or any other data besides these markers.
					// We allow a consumeing service to take care of garbage collection.
				}
				Ok(None) => {
					break;
				}
				Err(e) => {
					// Insert the error into the inferences
					data.message_inferences.insert(None, e);
				}
			}
		}

		// Try to send messages to the swarm via gossamer
		let gossamer_query_plan = self.messages.gossamer_messages_out_plan();
		for (entity, (Out, message)) in data.message_facts.query(gossamer_query_plan) {
			match self.gossamer.send_message(entity, message) {
				Ok(_) => {
					data.message_inferences.remove::<Out>(entity);
					data.message_inferences.insert(Some(entity), InFlight);
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
					data.message_inferences.insert(None, (In, message))
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

#[cfg(test)]
pub mod tests {
	use super::*;
	use gwrdfa_container::{ContainerEntityBuffer, ContainerGiving, ContainerHolding};

	#[derive(Debug, Clone, PartialEq, Eq, Hash)]
	pub struct TestMessage(String);

	#[derive(Debug, Clone, PartialEq, Eq, Hash)]
	pub struct GossamerContainer {
		message: Option<TestMessage>,
		message_in: Option<In>,
		message_out: Option<Out>,
		message_in_flight: Option<InFlight>,
		message_broadcast: Option<Broadcast>,
	}

	impl ContainerHolding<TestMessage> for GossamerContainer {
		fn from_data(data: TestMessage) -> Self {
			Self {
				message: Some(data),
				message_in: None,
				message_out: None,
				message_in_flight: None,
				message_broadcast: None,
			}
		}

		fn update_with_data(&mut self, data: TestMessage) {
			self.message = Some(data);
		}

		fn remove_from_container(&mut self) {
			self.message = None;
		}
	}

	impl<'a> ContainerGiving<'a, Option<&'a TestMessage>> for GossamerContainer {
		fn as_item(&'a self) -> Option<&'a TestMessage> {
			self.message.as_ref()
		}
	}

	impl ContainerHolding<In> for GossamerContainer {
		fn from_data(data: In) -> Self {
			Self {
				message: None,
				message_in: Some(data),
				message_out: None,
				message_in_flight: None,
				message_broadcast: None,
			}
		}

		fn update_with_data(&mut self, data: In) {
			self.message_in = Some(data);
		}

		fn remove_from_container(&mut self) {
			self.message_in = None;
		}
	}

	impl<'a> ContainerGiving<'a, Option<&'a In>> for GossamerContainer {
		fn as_item(&'a self) -> Option<&'a In> {
			self.message_in.as_ref()
		}
	}

	impl ContainerHolding<Out> for GossamerContainer {
		fn from_data(data: Out) -> Self {
			Self {
				message: None,
				message_in: None,
				message_out: Some(data),
				message_in_flight: None,
				message_broadcast: None,
			}
		}

		fn update_with_data(&mut self, data: Out) {
			self.message_out = Some(data);
		}

		fn remove_from_container(&mut self) {
			self.message_out = None;
		}
	}

	impl<'a> ContainerGiving<'a, Option<&'a Out>> for GossamerContainer {
		fn as_item(&'a self) -> Option<&'a Out> {
			self.message_out.as_ref()
		}
	}

	impl ContainerHolding<InFlight> for GossamerContainer {
		fn from_data(data: InFlight) -> Self {
			Self {
				message: None,
				message_in: None,
				message_out: None,
				message_in_flight: Some(data),
				message_broadcast: None,
			}
		}

		fn update_with_data(&mut self, data: InFlight) {
			self.message_in_flight = Some(data);
		}

		fn remove_from_container(&mut self) {
			self.message_in_flight = None;
		}
	}

	impl<'a> ContainerGiving<'a, Option<&'a InFlight>> for GossamerContainer {
		fn as_item(&'a self) -> Option<&'a InFlight> {
			self.message_in_flight.as_ref()
		}
	}

	impl ContainerHolding<Broadcast> for GossamerContainer {
		fn from_data(data: Broadcast) -> Self {
			Self {
				message: None,
				message_in: None,
				message_out: None,
				message_in_flight: None,
				message_broadcast: Some(data),
			}
		}

		fn update_with_data(&mut self, data: Broadcast) {
			self.message_broadcast = Some(data);
		}

		fn remove_from_container(&mut self) {
			self.message_broadcast = None;
		}
	}

	impl<'a> ContainerGiving<'a, Option<&'a Broadcast>> for GossamerContainer {
		fn as_item(&'a self) -> Option<&'a Broadcast> {
			self.message_broadcast.as_ref()
		}
	}
}
