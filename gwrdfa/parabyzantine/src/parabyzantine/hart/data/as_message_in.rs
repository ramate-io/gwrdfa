use crate::message_in::{ParabyzantineMessageInData, ParabyzantineMessageInDataSpec};
use crate::hart::{ParabyzantineData, ParabyzantineDataSpec};

/// Blanket implementation for the message in spec.
///
/// Downcasting the world to a message in world.
impl<Spec: ParabyzantineDataSpec> ParabyzantineMessageInDataSpec for Spec {
	type MessageEntity = Spec::MessageEntity;
	type MessageBuffer = Spec::MessageBuffer;
	type MessageDraftBuffer = Spec::MessageDraftBuffer;
	type TransactionEntity = Spec::TransactionEntity;
	type TransactionBuffer = Spec::TransactionBuffer;
	type TransactionDraftBuffer = Spec::TransactionDraftBuffer;
	type CertificateEntity = Spec::CertificateEntity;
	type CertificateBuffer = Spec::CertificateBuffer;
	type CertificateDraftBuffer = Spec::CertificateDraftBuffer;
}

/// Blanket implementation for the message in data.
impl<Spec: ParabyzantineDataSpec, Data: ParabyzantineData<Spec>> ParabyzantineMessageInData<Spec>
	for Data
{
	fn parabyzantine_message_in_message_buffer(&self) -> &Spec::MessageBuffer {
		self.parabyzantine_message_buffer()
	}
	fn parabyzantine_message_in_message_draft_buffer(&self) -> Spec::MessageDraftBuffer {
		self.parabyzantine_message_draft_buffer()
	}
	fn parabyzantine_message_in_transaction_buffer(&self) -> &Spec::TransactionBuffer {
		self.parabyzantine_transaction_buffer()
	}
	fn parabyzantine_message_in_transaction_draft_buffer(&self) -> Spec::TransactionDraftBuffer {
		self.parabyzantine_transaction_draft_buffer()
	}
	fn parabyzantine_message_in_certificate_buffer(&self) -> &Spec::CertificateBuffer {
		self.parabyzantine_certificate_buffer()
	}
	fn parabyzantine_message_in_certificate_draft_buffer(&self) -> Spec::CertificateDraftBuffer {
		self.parabyzantine_certificate_draft_buffer()
	}
}
