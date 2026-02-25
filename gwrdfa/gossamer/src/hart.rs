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
	<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer: GossamerMessageStorage<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		Spec::Message,
	>,
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
	<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer: GossamerMessageStorage<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		Spec::Message,
	>,
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
	<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer: GossamerMessageStorage<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		Spec::Message,
	>,
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
	use gwrdfa_container::{
		draft_buffer::ContainerEntityDraftBuffer,
		query::matching_tuple::{MatchingTuple, MatchingTupleQuery},
		ContainerEntity, ContainerEntityBuffer,
	};
	use parabyzantine::{NoOp, NoOpData, ParabyzantineData};

	#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
	pub struct TestMessage(String);

	impl GossamerMessage for TestMessage {
		fn to_goassamer_bytes(&self) -> Result<Vec<u8>, GossamerMessageError> {
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

	#[tokio::test]
	async fn test_gossamer_hart() -> Result<(), anyhow::Error> {
		let (
			gossamer,
			message_into_gossamer_sender,
			mut entity_message_from_gossamer_receiver,
			entity_into_gossamer_sender,
		) = Gossamer::<ContainerEntity>::mock();
		let messages = TestGossamerMessages;
		let mut hart =
			GossamerHart::<TestParabyzantineDataBinding, TestGossamerSpec>::new(gossamer, messages);

		message_into_gossamer_sender
			.send(TestMessage("Hello, world!".to_string()).to_goassamer_bytes()?)?;

		let mut data = TestParabyzantineData::default();

		hart.act_on_parabyzantine_hart(&mut data);

		// Check that the message was inserted into the buffer
		{
			let mut containers = Vec::new();
			for (entity, container) in data.gossamer_buffer.iter() {
				containers.push((entity, container));
			}

			assert_eq!(containers.len(), 1);
		}

		Ok(())
	}
}
