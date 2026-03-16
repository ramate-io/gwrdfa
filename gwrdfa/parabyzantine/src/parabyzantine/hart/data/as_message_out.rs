use crate::hart::ParabyzantineData;
use crate::message_out::ParabyzantineMessageOutData;

/// Blanket implementation for the message out data.
impl<Data: ParabyzantineData> ParabyzantineMessageOutData for Data {
	type TaskEntity = Data::TaskEntity;
	type TaskBuffer = Data::TaskBuffer;
	type TaskDraftBuffer = Data::TaskDraftBuffer;
	type MessageEntity = Data::MessageEntity;
	type MessageBuffer = Data::MessageBuffer;
	type MessageDraftBuffer = Data::MessageDraftBuffer;

	fn parabyzantine_message_out_task_buffer(&self) -> &Data::TaskBuffer {
		self.parabyzantine_task_buffer()
	}
	fn parabyzantine_message_out_task_buffer_mut(&mut self) -> &mut Data::TaskBuffer {
		self.parabyzantine_task_buffer_mut()
	}
	fn parabyzantine_message_out_task_draft_buffer(&self) -> Data::TaskDraftBuffer {
		self.parabyzantine_task_draft_buffer()
	}
	fn parabyzantine_message_out_message_buffer(&self) -> &Data::MessageBuffer {
		self.parabyzantine_message_buffer()
	}
	fn parabyzantine_message_out_message_buffer_mut(&mut self) -> &mut Data::MessageBuffer {
		self.parabyzantine_message_buffer_mut()
	}
	fn parabyzantine_message_out_message_draft_buffer(&self) -> Data::MessageDraftBuffer {
		self.parabyzantine_message_draft_buffer()
	}
}
