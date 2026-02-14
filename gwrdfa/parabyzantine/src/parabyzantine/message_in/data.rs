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
pub trait ParabyzantineMessageInDataSpec: Sized {
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
}

pub trait ParabyzantineMessageInData<Spec: ParabyzantineMessageInDataSpec>: Sized {
	/// The buffer for the message.
	fn parabyzantine_message_in_message_buffer(&self) -> &Spec::MessageBuffer;
	/// The draft buffer for the message.
	fn parabyzantine_message_in_message_draft_buffer(&self) -> Spec::MessageDraftBuffer;
	/// The buffer for the transaction.
	fn parabyzantine_message_in_transaction_buffer(&self) -> &Spec::TransactionBuffer;
	/// The draft buffer for the transaction.
	fn parabyzantine_message_in_transaction_draft_buffer(&self) -> Spec::TransactionDraftBuffer;
	/// The buffer for the certificate.
	fn parabyzantine_message_in_certificate_buffer(&self) -> &Spec::CertificateBuffer;
	/// The draft buffer for the certificate.
	fn parabyzantine_message_in_certificate_draft_buffer(&self) -> Spec::CertificateDraftBuffer;

	fn parabyzantine_message_in_world(&self) -> MessageInWorld<Spec> {
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
pub struct MessageInWorld<'a, Spec: ParabyzantineMessageInDataSpec> {
	pub message_facts: Facts<'a, Spec::MessageEntity, Spec::MessageBuffer>,
	pub message_inferences:
		Inferences<Spec::MessageEntity, Spec::MessageBuffer, Spec::MessageDraftBuffer>,
	pub transaction_facts: Facts<'a, Spec::TransactionEntity, Spec::TransactionBuffer>,
	pub transaction_inferences:
		Inferences<Spec::TransactionEntity, Spec::TransactionBuffer, Spec::TransactionDraftBuffer>,
	pub certificate_facts: Facts<'a, Spec::CertificateEntity, Spec::CertificateBuffer>,
	pub certificate_inferences:
		Inferences<Spec::CertificateEntity, Spec::CertificateBuffer, Spec::CertificateDraftBuffer>,
}

pub trait ParabyzantineMessageIn: Sized {
	type Spec: ParabyzantineMessageInDataSpec;

	/// Compute the parabyzantine message in.
	fn compute_parabyzantine_message_in(&mut self, data: &mut MessageInWorld<Self::Spec>);
}

/// A [ParabyzantineMessageInDataBinding] is a binding for the [ParabyzantineMessageIn] protocol.
///
/// It binds between the [ParabyzantineMessageInDataSpec] and the [ParabyzantineMessageInData].
pub trait ParabyzantineMessageInDataBinding {
	type Spec: ParabyzantineMessageInDataSpec;
	type Data: ParabyzantineMessageInData<Self::Spec>;
}
