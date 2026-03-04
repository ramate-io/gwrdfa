use crate::message_in::ParabyzantineMessageInData;
use crate::hart::ParabyzantineData;

/// Blanket implementation for the message in data.
impl<Data: ParabyzantineData> ParabyzantineMessageInData for Data {
	type MessageEntity = Data::MessageEntity;
	type MessageBuffer = Data::MessageBuffer;
	type MessageDraftBuffer = Data::MessageDraftBuffer;
	type TransactionEntity = Data::TransactionEntity;
	type TransactionBuffer = Data::TransactionBuffer;
	type TransactionDraftBuffer = Data::TransactionDraftBuffer;
	type CertificateEntity = Data::CertificateEntity;
	type CertificateBuffer = Data::CertificateBuffer;
	type CertificateDraftBuffer = Data::CertificateDraftBuffer;

	fn parabyzantine_message_in_message_buffer(&self) -> &Data::MessageBuffer {
		self.parabyzantine_message_buffer()
	}
	fn parabyzantine_message_in_message_draft_buffer(&self) -> Data::MessageDraftBuffer {
		self.parabyzantine_message_draft_buffer()
	}
	fn parabyzantine_message_in_transaction_buffer(&self) -> &Data::TransactionBuffer {
		self.parabyzantine_transaction_buffer()
	}
	fn parabyzantine_message_in_transaction_draft_buffer(&self) -> Data::TransactionDraftBuffer {
		self.parabyzantine_transaction_draft_buffer()
	}
	fn parabyzantine_message_in_certificate_buffer(&self) -> &Data::CertificateBuffer {
		self.parabyzantine_certificate_buffer()
	}
	fn parabyzantine_message_in_certificate_draft_buffer(&self) -> Data::CertificateDraftBuffer {
		self.parabyzantine_certificate_draft_buffer()
	}
}
