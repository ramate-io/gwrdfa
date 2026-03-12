use crate::act::Act;
use crate::buffer::{facts::Facts, inferences::Inferences, Bufferlike, DraftBufferlike};

#[derive(Debug, Clone, Copy)]
pub struct MessageOut;

/// Specifies the entities and buffers for a parabyzantine message out Data.
///
/// A Parabyzantine message out Data is concerned with deriving messages from tasks.
pub trait ParabyzantineMessageOutData: Sized {
	/// The entity type for the task.
	type TaskEntity: Sized;
	/// The buffer type for the task.
	type TaskBuffer: Bufferlike<Self::TaskEntity>;
	/// The draft buffer type for the task.
	type TaskDraftBuffer: DraftBufferlike<Self::TaskEntity, Self::TaskBuffer>;

	/// The entity type for the message.
	type MessageEntity: Sized;
	/// The buffer type for the message.
	type MessageBuffer: Bufferlike<Self::MessageEntity>;
	/// The draft buffer type for the message.
	type MessageDraftBuffer: DraftBufferlike<Self::MessageEntity, Self::MessageBuffer>;
	/// The buffer for the task.
	fn parabyzantine_message_out_task_buffer(&self) -> &Self::TaskBuffer;
	/// The buffer for the task.
	fn parabyzantine_message_out_task_buffer_mut(&mut self) -> &mut Self::TaskBuffer;
	/// The draft buffer for the task.
	fn parabyzantine_message_out_task_draft_buffer(&self) -> Self::TaskDraftBuffer;
	/// The buffer for the message.
	fn parabyzantine_message_out_message_buffer(&self) -> &Self::MessageBuffer;
	/// The buffer for the message.
	fn parabyzantine_message_out_message_buffer_mut(&mut self) -> &mut Self::MessageBuffer;
	/// The draft buffer for the message.
	fn parabyzantine_message_out_message_draft_buffer(&self) -> Self::MessageDraftBuffer;

	/// The world of the message out.
	fn parabyzantine_message_out_world<'a>(&'a self) -> MessageOutWorld<'a, Self> {
		MessageOutWorld {
			task_facts: self.parabyzantine_message_out_task_buffer().into(),
			task_inferences: self.parabyzantine_message_out_task_draft_buffer().into(),
			message_facts: self.parabyzantine_message_out_message_buffer().into(),
			message_inferences: self.parabyzantine_message_out_message_draft_buffer().into(),
		}
	}

	fn commit_parabyzantine_message_out(
		&mut self,
		message_out_inferences: MessageOutInferences<Self>,
	) {
		self.parabyzantine_message_out_task_buffer_mut()
			.commit_inferences(message_out_inferences.task_inferences);
		self.parabyzantine_message_out_message_buffer_mut()
			.commit_inferences(message_out_inferences.message_inferences);
	}
}

impl<'a, Data: ParabyzantineMessageOutData> From<MessageOutWorld<'a, Data>>
	for MessageOutInferences<Data>
{
	fn from(world: MessageOutWorld<'a, Data>) -> Self {
		MessageOutInferences {
			task_inferences: world.task_inferences,
			message_inferences: world.message_inferences,
		}
	}
}

/// The world of the message out step of a parabyzantine message out Data.
pub struct MessageOutWorld<'a, Data: ParabyzantineMessageOutData> {
	pub task_facts: Facts<'a, Data::TaskEntity, Data::TaskBuffer>,
	pub task_inferences: Inferences<Data::TaskEntity, Data::TaskBuffer, Data::TaskDraftBuffer>,
	pub message_facts: Facts<'a, Data::MessageEntity, Data::MessageBuffer>,
	pub message_inferences:
		Inferences<Data::MessageEntity, Data::MessageBuffer, Data::MessageDraftBuffer>,
}

/// The inferences for the message out step of a parabyzantine message out Data.
pub struct MessageOutInferences<Data: ParabyzantineMessageOutData> {
	pub task_inferences: Inferences<Data::TaskEntity, Data::TaskBuffer, Data::TaskDraftBuffer>,
	pub message_inferences:
		Inferences<Data::MessageEntity, Data::MessageBuffer, Data::MessageDraftBuffer>,
}

pub trait ParabyzantineMessageOut<Data: ParabyzantineMessageOutData>: Sized {
	/// Gets the [MessageOutWorld] for the parabyzantine message out.
	fn parabyzantine_message_out_world<'a>(
		&mut self,
		data: &'a mut Data,
	) -> MessageOutWorld<'a, Data> {
		data.parabyzantine_message_out_world()
	}

	/// Commits the inferences for the parabyzantine message out.
	fn commit_parabyzantine_message_out(
		&mut self,
		message_out_inferences: MessageOutInferences<Data>,
		data: &mut Data,
	) {
		data.commit_parabyzantine_message_out(message_out_inferences);
	}

	/// Compute the parabyzantine message out.
	fn compute_parabyzantine_message_out(&mut self, data: &mut MessageOutWorld<Data>);
}

impl<Data: ParabyzantineMessageOutData, MessageOutHandler: ParabyzantineMessageOut<Data>>
	Act<MessageOut, Data> for MessageOutHandler
{
	fn act(&mut self, _action: MessageOut, data: &mut Data) {
		let mut world = self.parabyzantine_message_out_world(data);
		self.compute_parabyzantine_message_out(&mut world);
		self.commit_parabyzantine_message_out(world.into(), data);
	}
}
