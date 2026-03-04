use crate::buffer::{facts::Facts, inferences::Inferences, Bufferlike, DraftBufferlike};

#[derive(Debug, Clone, Copy)]
pub struct MessageIn;

/// Specifies the entities and buffers for a parabyzantine message in Data.
///
/// A Parabyzantine message in Data is concerned with deriving transactions and certificates from messages.
///
/// Mainly, what a system implemented on this kind of data will do is
/// look at all the messages and determine which ones come from...
/// 1. Outside the system, in which case they are transactions, or
/// 2. Inside the system, in which case they are certificates
pub trait ParabyzantineMessageInData: Sized {
	/// The entity type for the message.
	type MessageEntity: Sized;
	/// The buffer type for the message.
	type MessageBuffer: Bufferlike<Self::MessageEntity>;
	/// The draft buffer type for the message.
	type MessageDraftBuffer: DraftBufferlike<Self::MessageEntity, Self::MessageBuffer>;

	/// The entity type for the transaction.
	type TransactionEntity: Sized;
	/// The buffer type for the transaction.
	type TransactionBuffer: Bufferlike<Self::TransactionEntity>;
	/// The draft buffer type for the transaction.
	type TransactionDraftBuffer: DraftBufferlike<Self::TransactionEntity, Self::TransactionBuffer>;

	/// The entity type for the certificate.
	type CertificateEntity: Sized;
	/// The buffer type for the certificate.
	type CertificateBuffer: Bufferlike<Self::CertificateEntity>;
	/// The draft buffer type for the certificate.
	type CertificateDraftBuffer: DraftBufferlike<Self::CertificateEntity, Self::CertificateBuffer>;
	/// The buffer for the message.
	fn parabyzantine_message_in_message_buffer(&self) -> &Self::MessageBuffer;
	/// The draft buffer for the message.
	fn parabyzantine_message_in_message_draft_buffer(&self) -> Self::MessageDraftBuffer;
	/// The buffer for the transaction.
	fn parabyzantine_message_in_transaction_buffer(&self) -> &Self::TransactionBuffer;
	/// The draft buffer for the transaction.
	fn parabyzantine_message_in_transaction_draft_buffer(&self) -> Self::TransactionDraftBuffer;
	/// The buffer for the certificate.
	fn parabyzantine_message_in_certificate_buffer(&self) -> &Self::CertificateBuffer;
	/// The draft buffer for the certificate.
	fn parabyzantine_message_in_certificate_draft_buffer(&self) -> Self::CertificateDraftBuffer;

	fn parabyzantine_message_in_world<'a>(&'a self) -> MessageInWorld<'a, Self> {
		MessageInWorld {
			message_facts: self.parabyzantine_message_in_message_buffer().into(),
			message_inferences: self.parabyzantine_message_in_message_draft_buffer().into(),
			transaction_facts: self.parabyzantine_message_in_transaction_buffer().into(),
			transaction_inferences: self.parabyzantine_message_in_transaction_draft_buffer().into(),
			certificate_facts: self.parabyzantine_message_in_certificate_buffer().into(),
			certificate_inferences: self.parabyzantine_message_in_certificate_draft_buffer().into(),
		}
	}
}

/// The world of the message in step of a parabyzantine message in Data.
pub struct MessageInWorld<'a, Data: ParabyzantineMessageInData> {
	pub message_facts: Facts<'a, Data::MessageEntity, Data::MessageBuffer>,
	pub message_inferences:
		Inferences<Data::MessageEntity, Data::MessageBuffer, Data::MessageDraftBuffer>,
	pub transaction_facts: Facts<'a, Data::TransactionEntity, Data::TransactionBuffer>,
	pub transaction_inferences:
		Inferences<Data::TransactionEntity, Data::TransactionBuffer, Data::TransactionDraftBuffer>,
	pub certificate_facts: Facts<'a, Data::CertificateEntity, Data::CertificateBuffer>,
	pub certificate_inferences:
		Inferences<Data::CertificateEntity, Data::CertificateBuffer, Data::CertificateDraftBuffer>,
}

pub trait ParabyzantineMessageIn<Data: ParabyzantineMessageInData>: Sized {
	/// Compute the parabyzantine message in.
	fn compute_parabyzantine_message_in(&mut self, data: &mut MessageInWorld<Data>);
}

/// A [ParabyzantineMessageInDataBinding] is a binding for the [ParabyzantineMessageIn] protocol.
///
/// It binds to [ParabyzantineMessageInData].
pub trait ParabyzantineMessageInDataBinding {
	type Data: ParabyzantineMessageInData;
}
