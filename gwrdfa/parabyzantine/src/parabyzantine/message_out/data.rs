use crate::buffer::{facts::Facts, inferences::Inferences, Bufferlike, DraftBufferlike};

#[derive(Debug, Clone, Copy)]
pub struct MessageOut;

/// Specifies the entities and buffers for a parabyzantine message out Data.
///
/// A Parabyzantine message out Data is concerned with deriving messages from tasks.
pub trait ParabyzantineMessageOutDataSpec: Sized {
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
}

pub trait ParabyzantineMessageOutData<Spec: ParabyzantineMessageOutDataSpec>: Sized {
	/// The buffer for the task.
	fn parabyzantine_message_out_task_buffer(&self) -> &Spec::TaskBuffer;
	/// The draft buffer for the task.
	fn parabyzantine_message_out_task_draft_buffer(&self) -> Spec::TaskDraftBuffer;
	/// The buffer for the message.
	fn parabyzantine_message_out_message_buffer(&self) -> &Spec::MessageBuffer;
	/// The draft buffer for the message.
	fn parabyzantine_message_out_message_draft_buffer(&self) -> Spec::MessageDraftBuffer;

	/// The world of the message out.
	fn parabyzantine_message_out_world(&self) -> MessageOutWorld<Spec> {
		MessageOutWorld {
			task_facts: self.parabyzantine_message_out_task_buffer().into(),
			task_inferences: self.parabyzantine_message_out_task_draft_buffer().into(),
			message_facts: self.parabyzantine_message_out_message_buffer().into(),
			message_inferences: self.parabyzantine_message_out_message_draft_buffer().into(),
		}
	}
}
/// The world of the message out step of a parabyzantine message out Data.
pub struct MessageOutWorld<'a, Spec: ParabyzantineMessageOutDataSpec> {
	pub task_facts: Facts<'a, Spec::TaskEntity, Spec::TaskBuffer>,
	pub task_inferences: Inferences<Spec::TaskEntity, Spec::TaskBuffer, Spec::TaskDraftBuffer>,
	pub message_facts: Facts<'a, Spec::MessageEntity, Spec::MessageBuffer>,
	pub message_inferences:
		Inferences<Spec::MessageEntity, Spec::MessageBuffer, Spec::MessageDraftBuffer>,
}

pub trait ParabyzantineMessageOut: Sized {
	type Spec: ParabyzantineMessageOutDataSpec;

	/// Compute the parabyzantine message out.
	fn compute_parabyzantine_message_out(&mut self, data: &mut MessageOutWorld<Self::Spec>);
}

/// A [ParabyzantineMessageOutDataBinding] is a binding for the [ParabyzantineMessageOut] protocol.
///
/// It binds between the [ParabyzantineMessageOutDataSpec] and the [ParabyzantineMessageOutData].
pub trait ParabyzantineMessageOutDataBinding {
	type Spec: ParabyzantineMessageOutDataSpec;
	type Data: ParabyzantineMessageOutData<Self::Spec>;
}
