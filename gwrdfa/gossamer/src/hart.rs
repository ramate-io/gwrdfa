pub mod gossamer_messages;
pub mod gossamer_storage;
pub mod spec;

use gossamer_messages::GossamerMessages;
use gossamer_storage::GossamerMessageStorage;
pub use spec::GossamerSpec;

use crate::Gossamer;
use crate::{Broadcast, In, InFlight, Out};
use parabyzantine::hart::{
	ParabyzantineDataBinding, ParabyzantineDataSpec, ParabyzantineHart, ParabyzantineWorld,
};

pub struct GossamerHart<Binding: ParabyzantineDataBinding, Spec: GossamerSpec<Binding>>
where
	<Binding::Spec as ParabyzantineDataSpec>::MessageDraftBuffer: GossamerMessageStorage<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		Spec::Message,
	>,
	<Binding::Spec as ParabyzantineDataSpec>::MessageEntity: Send + Sync,
{
	messages: Spec::Messages,
	gossamer: Gossamer<<Binding::Spec as ParabyzantineDataSpec>::MessageEntity>,
	max_batch_size: usize,
}

impl<Binding: ParabyzantineDataBinding, Spec: GossamerSpec<Binding>> GossamerHart<Binding, Spec>
where
	<Binding::Spec as ParabyzantineDataSpec>::MessageDraftBuffer: GossamerMessageStorage<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		Spec::Message,
	>,
	<Binding::Spec as ParabyzantineDataSpec>::MessageEntity: Send + Sync,
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
	<Binding::Spec as ParabyzantineDataSpec>::MessageDraftBuffer: GossamerMessageStorage<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		Spec::Message,
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
	use crate::GossamerMessageError;
	use crate::{container::GossamerContainer, delta_container::GossamerDeltaContainer};
	use gwrdfa_container::Component;
	use gwrdfa_container::{
		draft_buffer::ContainerEntityDraftBuffer,
		query::matching_tuple::{MatchingTuple, MatchingTupleQuery},
		ContainerEntity, ContainerEntityBuffer,
	};
	use parabyzantine::{NoOp, NoOpData, ParabyzantineData};
	use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

	#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
	pub struct TestMessage(String);

	impl GossamerMessage for TestMessage {
		fn to_gossamer_bytes(&self) -> Result<Vec<u8>, GossamerMessageError> {
			Ok(self.0.as_bytes().to_vec())
		}

		fn from_gossamer_bytes(bytes: Vec<u8>) -> Result<Self, GossamerMessageError> {
			Ok(TestMessage(String::from_utf8(bytes).unwrap()))
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
		type MessageBuffer = ContainerEntityBuffer<GossamerContainer<TestMessage>>;
		type MessageDraftBuffer = ContainerEntityDraftBuffer<GossamerDeltaContainer<TestMessage>>;
		type TaskEntity = NoOp;
		type TaskBuffer = NoOp;
		type TaskDraftBuffer = NoOp;
	}

	#[derive(Debug, Default)]
	pub struct TestParabyzantineData {
		pub gossamer_buffer: ContainerEntityBuffer<GossamerContainer<TestMessage>>,
		pub noop_data: NoOpData,
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

		fn parabyzantine_message_buffer(
			&self,
		) -> &ContainerEntityBuffer<GossamerContainer<TestMessage>> {
			&self.gossamer_buffer
		}

		fn parabyzantine_message_buffer_mut(
			&mut self,
		) -> &mut ContainerEntityBuffer<GossamerContainer<TestMessage>> {
			&mut self.gossamer_buffer
		}

		fn parabyzantine_message_draft_buffer(
			&self,
		) -> ContainerEntityDraftBuffer<GossamerDeltaContainer<TestMessage>> {
			ContainerEntityDraftBuffer::default()
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

	impl GossamerMessages<TestParabyzantineDataBinding> for TestGossamerMessages {
		type Message = TestMessage;
		type OutQuery<'a> =
			MatchingTupleQuery<'a, GossamerContainer<TestMessage>, (Out, TestMessage)>;
		type OutQueryPlan = MatchingTuple<(Out, TestMessage)>;

		fn gossamer_messages_out_plan(&mut self) -> MatchingTuple<(Out, TestMessage)> {
			MatchingTuple::new()
		}
	}

	pub struct TestGossamerSpec;

	impl GossamerSpec<TestParabyzantineDataBinding> for TestGossamerSpec {
		type Message = TestMessage;
		type OutQuery<'a> =
			MatchingTupleQuery<'a, GossamerContainer<TestMessage>, (Out, TestMessage)>;
		type OutQueryPlan = MatchingTuple<(Out, TestMessage)>;
		type Messages = TestGossamerMessages;
	}

	fn hart_in(
		hart: &mut GossamerHart<TestParabyzantineDataBinding, TestGossamerSpec>,
		data: &mut TestParabyzantineData,
		message_into_gossamer_sender: UnboundedSender<Vec<u8>>,
		mut messages: Vec<TestMessage>,
	) -> Result<(), anyhow::Error> {
		for message in messages.iter() {
			message_into_gossamer_sender.send(message.to_gossamer_bytes()?)?;
		}

		hart.act_on_parabyzantine_hart(data);

		{
			let mut buffer_messages = Vec::new();
			let mut containers = Vec::new();
			for (entity, container) in data.gossamer_buffer.iter() {
				containers.push((entity, container));
				match &container.message {
					Component::Present(message) => {
						buffer_messages.push(message.clone());
					}
					_ => {
						return Err(anyhow::anyhow!("Message not found in buffer"));
					}
				}
			}

			messages.sort();
			buffer_messages.sort();
			assert_eq!(messages, buffer_messages);
		}

		Ok(())
	}

	fn hart_out(
		hart: &mut GossamerHart<TestParabyzantineDataBinding, TestGossamerSpec>,
		data: &mut TestParabyzantineData,
		entity_message_from_gossamer_receiver: &mut UnboundedReceiver<(ContainerEntity, Vec<u8>)>,
		mut messages: Vec<TestMessage>,
	) -> Result<Vec<(ContainerEntity, TestMessage)>, anyhow::Error> {
		for message in messages.iter() {
			data.gossamer_buffer.insert_container(GossamerContainer {
				message: Component::Present(message.clone()),
				message_in: Component::Absent,
				message_out: Component::Present(Out),
				message_in_flight: Component::Absent,
				message_broadcast: Component::Absent,
				message_error: Component::Absent,
			});
		}

		hart.act_on_parabyzantine_hart(data);

		let mut out_messages = Vec::new();
		let mut out_messages_with_entities = Vec::new();

		for _ in 0..messages.len() {
			let (entity, gossamer_bytes) = entity_message_from_gossamer_receiver.try_recv()?;
			out_messages.push(TestMessage::from_gossamer_bytes(gossamer_bytes.clone())?);
			out_messages_with_entities
				.push((entity, TestMessage::from_gossamer_bytes(gossamer_bytes)?));
		}

		messages.sort();
		out_messages.sort();
		assert_eq!(messages, out_messages);

		Ok(out_messages_with_entities)
	}

	fn hart_confirm(
		hart: &mut GossamerHart<TestParabyzantineDataBinding, TestGossamerSpec>,
		data: &mut TestParabyzantineData,
		entity_into_gossamer_sender: UnboundedSender<ContainerEntity>,
		messages: Vec<(ContainerEntity, TestMessage)>,
	) -> Result<(), anyhow::Error> {
		for (entity, _message) in messages.iter() {
			entity_into_gossamer_sender.send(*entity)?;
		}

		hart.act_on_parabyzantine_hart(data);

		for (entity, message) in messages.into_iter() {
			let broadcast_container =
				data.gossamer_buffer.get(entity).ok_or(anyhow::anyhow!("Entity not found"))?;
			assert_eq!(
				broadcast_container,
				&GossamerContainer {
					message: Component::Present(message),
					message_in: Component::Absent,
					message_out: Component::Absent,
					message_in_flight: Component::Absent,
					message_broadcast: Component::Present(Broadcast),
					message_error: Component::Absent,
				}
			);
		}

		Ok(())
	}

	fn hart_out_and_confirm(
		hart: &mut GossamerHart<TestParabyzantineDataBinding, TestGossamerSpec>,
		data: &mut TestParabyzantineData,
		entity_message_from_gossamer_receiver: &mut UnboundedReceiver<(ContainerEntity, Vec<u8>)>,
		entity_into_gossamer_sender: UnboundedSender<ContainerEntity>,
		messages: Vec<TestMessage>,
	) -> Result<(), anyhow::Error> {
		let out_messages = hart_out(hart, data, entity_message_from_gossamer_receiver, messages)?;
		hart_confirm(hart, data, entity_into_gossamer_sender, out_messages)?;
		Ok(())
	}

	#[test]
	fn test_gossamer_single_hart_in() -> Result<(), anyhow::Error> {
		let (
			gossamer,
			message_into_gossamer_sender,
			mut _entity_message_from_gossamer_receiver,
			_entity_into_gossamer_sender,
		) = Gossamer::<ContainerEntity>::mock();

		let messages = TestGossamerMessages;
		let mut hart =
			GossamerHart::<TestParabyzantineDataBinding, TestGossamerSpec>::new(gossamer, messages);

		let mut data = TestParabyzantineData::default();

		hart_in(
			&mut hart,
			&mut data,
			message_into_gossamer_sender,
			vec![TestMessage("Hello, world!".to_string())],
		)?;

		Ok(())
	}

	/// Tests just hart out.
	#[test]
	fn test_gossamer_single_hart_out() -> Result<(), anyhow::Error> {
		let (
			gossamer,
			_message_into_gossamer_sender,
			mut entity_message_from_gossamer_receiver,
			_entity_into_gossamer_sender,
		) = Gossamer::<ContainerEntity>::mock();

		let messages = TestGossamerMessages;
		let mut hart =
			GossamerHart::<TestParabyzantineDataBinding, TestGossamerSpec>::new(gossamer, messages);

		let mut data = TestParabyzantineData::default();

		hart_out(
			&mut hart,
			&mut data,
			&mut entity_message_from_gossamer_receiver,
			vec![TestMessage("Hello, world out!".to_string())],
		)?;

		Ok(())
	}

	/// Tests hart out and confirm.
	#[tokio::test]
	async fn test_gossamer_single_hart_out_confirm() -> Result<(), anyhow::Error> {
		let (
			gossamer,
			_message_into_gossamer_sender,
			mut entity_message_from_gossamer_receiver,
			entity_into_gossamer_sender,
		) = Gossamer::<ContainerEntity>::mock();

		let messages = TestGossamerMessages;
		let mut hart =
			GossamerHart::<TestParabyzantineDataBinding, TestGossamerSpec>::new(gossamer, messages);

		let mut data = TestParabyzantineData::default();

		hart_out_and_confirm(
			&mut hart,
			&mut data,
			&mut entity_message_from_gossamer_receiver,
			entity_into_gossamer_sender,
			vec![TestMessage("Hello, world out!".to_string())],
		)?;

		Ok(())
	}

	/// Tests the complete lifecycle of a message through the Gossamer Hart.
	#[test]
	fn test_gossamer_single_hart() -> Result<(), anyhow::Error> {
		let (
			gossamer,
			message_into_gossamer_sender,
			mut entity_message_from_gossamer_receiver,
			entity_into_gossamer_sender,
		) = Gossamer::<ContainerEntity>::mock();
		let messages = TestGossamerMessages;
		let mut hart =
			GossamerHart::<TestParabyzantineDataBinding, TestGossamerSpec>::new(gossamer, messages);

		let mut data = TestParabyzantineData::default();

		hart_in(
			&mut hart,
			&mut data,
			message_into_gossamer_sender,
			vec![TestMessage("Hello, world!".to_string())],
		)?;

		hart_out_and_confirm(
			&mut hart,
			&mut data,
			&mut entity_message_from_gossamer_receiver,
			entity_into_gossamer_sender,
			vec![TestMessage("Hello, world out!".to_string())],
		)?;

		Ok(())
	}

	#[test]
	fn test_gossamer_multiple_hart() -> Result<(), anyhow::Error> {
		let (
			gossamer,
			message_into_gossamer_sender,
			mut entity_message_from_gossamer_receiver,
			entity_into_gossamer_sender,
		) = Gossamer::<ContainerEntity>::mock();

		let messages = TestGossamerMessages;
		let mut hart =
			GossamerHart::<TestParabyzantineDataBinding, TestGossamerSpec>::new(gossamer, messages);

		let mut data = TestParabyzantineData::default();

		let in_messages: Vec<TestMessage> =
			(0..32).map(|i| TestMessage(format!("Hello, world! {}", i))).collect();
		hart_in(&mut hart, &mut data, message_into_gossamer_sender, in_messages)?;

		let out_messages: Vec<TestMessage> =
			(0..32).map(|i| TestMessage(format!("Hello, world out! {}", i))).collect();
		hart_out_and_confirm(
			&mut hart,
			&mut data,
			&mut entity_message_from_gossamer_receiver,
			entity_into_gossamer_sender,
			out_messages,
		)?;

		Ok(())
	}
}
