use crate::hart::{ParabyzantineData, ParabyzantineDataSpec, ParabyzantineWorld};
use crate::message_out::{
	MessageOutWorld, ParabyzantineMessageOutData, ParabyzantineMessageOutDataSpec,
};

/// Blanket implementation for the message out spec.
///
/// Downcasting the world to a message out world.
impl<Spec: ParabyzantineDataSpec> ParabyzantineMessageOutDataSpec for Spec {
	type TaskEntity = Spec::TaskEntity;
	type TaskBuffer = Spec::TaskBuffer;
	type MessageEntity = Spec::MessageEntity;
	type MessageBuffer = Spec::MessageBuffer;
}

/// Blanket implementation for the message out data.
impl<Spec: ParabyzantineDataSpec, Data: ParabyzantineData<Spec>> ParabyzantineMessageOutData<Spec>
	for Data
where
	Spec: 'static,
{
	fn parabyzantine_message_out_world<'a>(&'a mut self) -> MessageOutWorld<'a, Spec> {
		let ParabyzantineWorld { task_facts, message_facts, .. } = self.parabyzantine_world();

		MessageOutWorld { task_facts, message_facts }
	}
}
