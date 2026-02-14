use crate::message_out::{ParabyzantineMessageOutData, ParabyzantineMessageOutDataSpec};
use crate::hart::{ParabyzantineData, ParabyzantineDataSpec};

/// Blanket implementation for the message out spec.
///
/// Downcasting the world to a message out world.
impl<Spec: ParabyzantineDataSpec> ParabyzantineMessageOutDataSpec for Spec {
	type TaskEntity = Spec::TaskEntity;
	type TaskBuffer = Spec::TaskBuffer;
	type TaskDraftBuffer = Spec::TaskDraftBuffer;
	type MessageEntity = Spec::MessageEntity;
	type MessageBuffer = Spec::MessageBuffer;
	type MessageDraftBuffer = Spec::MessageDraftBuffer;
}

/// Blanket implementation for the message out data.
impl<Spec: ParabyzantineDataSpec, Data: ParabyzantineData<Spec>> ParabyzantineMessageOutData<Spec>
	for Data
{
	fn parabyzantine_message_out_task_buffer(&self) -> &Spec::TaskBuffer {
		self.parabyzantine_task_buffer()
	}
	fn parabyzantine_message_out_task_draft_buffer(&self) -> Spec::TaskDraftBuffer {
		self.parabyzantine_task_draft_buffer()
	}
	fn parabyzantine_message_out_message_buffer(&self) -> &Spec::MessageBuffer {
		self.parabyzantine_message_buffer()
	}
	fn parabyzantine_message_out_message_draft_buffer(&self) -> Spec::MessageDraftBuffer {
		self.parabyzantine_message_draft_buffer()
	}
}
