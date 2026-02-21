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
	use crate::GossamerMessage;
	use core::marker::PhantomData;
	use gwrdfa_container::{
		query::matching_tuple::{MatchingTuple, MatchingTupleQuery},
		Component, ContainerAccepting, ContainerEntity, ContainerEntityBuffer, ContainerGiving,
	};
	use parabyzantine::{NoOp, NoOpData, ParabyzantineData};

	#[derive(Debug, Clone, PartialEq, Eq, Hash)]
	pub struct TestMessage(String);

	impl GossamerMessage for TestMessage {
		fn to_goassamer_bytes(&self) -> Result<Vec<u8>, GossamerMessageError> {
			Ok(self.0.as_bytes().to_vec())
		}

		fn from_gossamer_bytes(bytes: Vec<u8>) -> Result<Self, GossamerMessageError> {
			Ok(TestMessage(String::from_utf8(bytes).unwrap()))
		}
	}

	#[derive(Debug, Clone, PartialEq, Eq)]
	pub struct GossamerContainer {
		message: Component<TestMessage>,
		message_in: Component<In>,
		message_out: Component<Out>,
		message_in_flight: Component<InFlight>,
		message_broadcast: Component<Broadcast>,
	}

	impl ContainerAccepting<TestMessage> for GossamerContainer {
		fn from_data(data: TestMessage) -> Self {
			Self {
				message: Component::Present(data),
				message_in: Component::Absent,
				message_out: Component::Absent,
				message_in_flight: Component::Absent,
				message_broadcast: Component::Absent,
			}
		}

		fn update_with_data(&mut self, data: TestMessage) {
			self.message = Component::Present(data);
		}

		fn remove_from_container(&mut self) {
			self.message = Component::Absent;
		}
	}

	impl<'a> ContainerGiving<'a, TestMessage> for GossamerContainer {
		fn as_component(&'a self) -> Component<&'a TestMessage> {
			self.message.as_ref()
		}
	}

	impl ContainerAccepting<In> for GossamerContainer {
		fn from_data(data: In) -> Self {
			Self {
				message: Component::Absent,
				message_in: Component::Present(data),
				message_out: Component::Absent,
				message_in_flight: Component::Absent,
				message_broadcast: Component::Absent,
			}
		}

		fn update_with_data(&mut self, data: In) {
			self.message_in = Component::Present(data);
		}

		fn remove_from_container(&mut self) {
			self.message_in = Component::Absent;
		}
	}

	impl<'a> ContainerGiving<'a, In> for GossamerContainer {
		fn as_component(&'a self) -> Component<&'a In> {
			self.message_in.as_ref()
		}
	}

	impl ContainerAccepting<Out> for GossamerContainer {
		fn from_data(data: Out) -> Self {
			Self {
				message: Component::Absent,
				message_in: Component::Absent,
				message_out: Component::Present(data),
				message_in_flight: Component::Absent,
				message_broadcast: Component::Absent,
			}
		}

		fn update_with_data(&mut self, data: Out) {
			self.message_out = Component::Present(data);
		}

		fn remove_from_container(&mut self) {
			self.message_out = Component::Absent;
		}
	}

	impl<'a> ContainerGiving<'a, Out> for GossamerContainer {
		fn as_component(&'a self) -> Component<&'a Out> {
			self.message_out.as_ref()
		}
	}

	impl ContainerAccepting<InFlight> for GossamerContainer {
		fn from_data(data: InFlight) -> Self {
			Self {
				message: Component::Absent,
				message_in: Component::Absent,
				message_out: Component::Absent,
				message_in_flight: Component::Present(data),
				message_broadcast: Component::Absent,
			}
		}

		fn update_with_data(&mut self, data: InFlight) {
			self.message_in_flight = Component::Present(data);
		}

		fn remove_from_container(&mut self) {
			self.message_in_flight = Component::Absent;
		}
	}

	impl<'a> ContainerGiving<'a, InFlight> for GossamerContainer {
		fn as_component(&'a self) -> Component<&'a InFlight> {
			self.message_in_flight.as_ref()
		}
	}

	impl ContainerAccepting<Broadcast> for GossamerContainer {
		fn from_data(data: Broadcast) -> Self {
			Self {
				message: Component::Absent,
				message_in: Component::Absent,
				message_out: Component::Absent,
				message_in_flight: Component::Absent,
				message_broadcast: Component::Present(data),
			}
		}

		fn update_with_data(&mut self, data: Broadcast) {
			self.message_broadcast = Component::Present(data);
		}

		fn remove_from_container(&mut self) {
			self.message_broadcast = Component::Absent;
		}
	}

	impl<'a> ContainerGiving<'a, Broadcast> for GossamerContainer {
		fn as_component(&'a self) -> Component<&'a Broadcast> {
			self.message_broadcast.as_ref()
		}
	}

	pub struct TestParabyzantineSpec;

	impl ParabyzantineDataSpec for TestParabyzantineSpec {
		type CertificateEntity = NoOp;
		type CertificateBuffer = NoOp;
		type CertificateDraftBuffer = NoOp;
		type AgreementEntity = NoOp;
		type AgreementBuffer = NoOp;
		type AgreementDraftBuffer = NoOp;
		type TransactionEntity = NoOp;
		type TransactionBuffer = NoOp;
		type TransactionDraftBuffer = NoOp;
		type MessageEntity = ContainerEntity;
		type MessageBuffer = ContainerEntityBuffer<GossamerContainer>;
		type MessageDraftBuffer = NoOp;
		type TaskEntity = NoOp;
		type TaskBuffer = NoOp;
		type TaskDraftBuffer = NoOp;
	}

	pub struct TestParabyzantineData {
		gossamer_buffer: ContainerEntityBuffer<GossamerContainer>,
		noop_data: NoOpData,
	}

	impl ParabyzantineData<TestParabyzantineSpec> for TestParabyzantineData {
		fn parabyzantine_certificate_buffer(&self) -> &NoOp {
			&self.noop_data.no_op
		}

		fn parabyzantine_certificate_buffer_mut(&mut self) -> &mut NoOp {
			&mut self.noop_data.no_op
		}

		fn parabyzantine_certificate_draft_buffer(&self) -> NoOp {
			NoOp
		}

		fn parabyzantine_agreement_buffer(&self) -> &NoOp {
			&self.noop_data.no_op
		}

		fn parabyzantine_agreement_buffer_mut(&mut self) -> &mut NoOp {
			&mut self.noop_data.no_op
		}

		fn parabyzantine_agreement_draft_buffer(&self) -> NoOp {
			NoOp
		}

		fn parabyzantine_transaction_buffer(&self) -> &NoOp {
			&self.noop_data.no_op
		}

		fn parabyzantine_transaction_buffer_mut(&mut self) -> &mut NoOp {
			&mut self.noop_data.no_op
		}

		fn parabyzantine_transaction_draft_buffer(&self) -> NoOp {
			NoOp
		}

		fn parabyzantine_message_buffer(&self) -> &ContainerEntityBuffer<GossamerContainer> {
			&self.gossamer_buffer
		}

		fn parabyzantine_message_buffer_mut(
			&mut self,
		) -> &mut ContainerEntityBuffer<GossamerContainer> {
			&mut self.gossamer_buffer
		}

		fn parabyzantine_message_draft_buffer(&self) -> NoOp {
			NoOp
		}

		fn parabyzantine_task_buffer(&self) -> &NoOp {
			&self.noop_data.no_op
		}

		fn parabyzantine_task_buffer_mut(&mut self) -> &mut NoOp {
			&mut self.noop_data.no_op
		}

		fn parabyzantine_task_draft_buffer(&self) -> NoOp {
			NoOp
		}
	}

	pub struct TestParabyzantineDataBinding;

	impl ParabyzantineDataBinding for TestParabyzantineDataBinding {
		type Spec = TestParabyzantineSpec;
		type Data = TestParabyzantineData;
	}

	pub struct TestGossamerMessages;

	impl<'a>
		GossamerMessages<
			'a,
			TestMessage,
			TestParabyzantineDataBinding,
			MatchingTupleQuery<'a, GossamerContainer, (Out, TestMessage)>,
			MatchingTuple<(Out, TestMessage)>,
		> for TestGossamerMessages
	{
		fn gossamer_messages_out_plan(&mut self) -> MatchingTuple<(Out, TestMessage)> {
			MatchingTuple::new()
		}
	}

	pub struct TestGossamerSpec<'a> {
		_phantom: PhantomData<&'a ()>,
	}

	impl<'a> GossamerSpec<'a, TestParabyzantineDataBinding> for TestGossamerSpec<'a> {
		type Message = TestMessage;
		type MessageOutQuery = MatchingTupleQuery<'a, GossamerContainer, (Out, TestMessage)>;
		type MessageOutQueryPlan = MatchingTuple<(Out, TestMessage)>;
		type Messages = TestGossamerMessages;
	}

	#[tokio::test]
	async fn test_gossamer_hart() {
		let gossamer = Gossamer::<ContainerEntity>::mock();
		let messages = GossamerMessages::<
			ContainerEntity,
			TestParabyzantineDataBinding,
			MatchingTupleQuery<'_, GossamerContainer, (Out, TestMessage)>,
			MatchingTuple<(Out, TestMessage)>,
		>::new();
		let hart = GossamerHart::<Binding, Spec>::new(gossamer, messages);
	}
}
