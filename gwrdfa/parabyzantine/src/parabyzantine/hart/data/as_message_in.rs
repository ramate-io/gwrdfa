use crate::hart::{ParabyzantineData, ParabyzantineDataSpec, ParabyzantineWorld};
use crate::message_in::{
	MessageInWorld, ParabyzantineMessageInData, ParabyzantineMessageInDataSpec,
};

/// Blanket implementation for the message in spec.
///
/// Downcasting the world to a message in world.
impl<Spec: ParabyzantineDataSpec> ParabyzantineMessageInDataSpec for Spec {
	type MessageEntity = Spec::MessageEntity;
	type MessageBuffer = Spec::MessageBuffer;
	type TransactionEntity = Spec::TransactionEntity;
	type TransactionBuffer = Spec::TransactionBuffer;
	type CertificateEntity = Spec::CertificateEntity;
	type CertificateBuffer = Spec::CertificateBuffer;
}

/// Blanket implementation for the message in data.
impl<Spec: ParabyzantineDataSpec, Data: ParabyzantineData<Spec>> ParabyzantineMessageInData<Spec>
	for Data
where
	Spec: 'static,
{
	fn parabyzantine_message_in_world<'a>(&'a mut self) -> MessageInWorld<'a, Spec> {
		let ParabyzantineWorld { message_facts, transaction_facts, certificate_facts, .. } =
			self.parabyzantine_world();

		MessageInWorld { message_facts, transaction_facts, certificate_facts }
	}
}
