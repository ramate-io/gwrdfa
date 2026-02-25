use crate::buffer::{facts::Facts, Bufferlike};

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

	/// The entity type for the message.
	type MessageEntity: Sized;
	/// The buffer type for the message.
	type MessageBuffer: Bufferlike<Self::MessageEntity>;
}

pub trait ParabyzantineMessageOutData<Spec: ParabyzantineMessageOutDataSpec>: Sized {
	/// The world of the message out.
	fn parabyzantine_message_out_world<'a>(&'a mut self) -> MessageOutWorld<'a, Spec>;
}
/// The world of the message out step of a parabyzantine message out Data.
pub struct MessageOutWorld<'a, Spec: ParabyzantineMessageOutDataSpec> {
	pub task_facts: Facts<'a, Spec::TaskEntity, Spec::TaskBuffer>,
	pub message_facts: Facts<'a, Spec::MessageEntity, Spec::MessageBuffer>,
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
