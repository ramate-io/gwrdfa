use crate::buffer::{facts::Facts, Bufferlike};

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

	/// The entity type for the transaction.
	type TransactionEntity: Sized;
	/// The buffer type for the transaction.
	type TransactionBuffer: Bufferlike<Self::TransactionEntity>;

	/// The entity type for the certificate.
	type CertificateEntity: Sized;
	/// The buffer type for the certificate.
	type CertificateBuffer: Bufferlike<Self::CertificateEntity>;
}

pub trait ParabyzantineMessageInData<Spec: ParabyzantineMessageInDataSpec>: Sized {
	fn parabyzantine_message_in_world<'a>(&'a mut self) -> MessageInWorld<'a, Spec>;
}

/// The world of the message in step of a parabyzantine message in Data.
pub struct MessageInWorld<'a, Spec: ParabyzantineMessageInDataSpec> {
	pub message_facts: Facts<'a, Spec::MessageEntity, Spec::MessageBuffer>,
	pub transaction_facts: Facts<'a, Spec::TransactionEntity, Spec::TransactionBuffer>,
	pub certificate_facts: Facts<'a, Spec::CertificateEntity, Spec::CertificateBuffer>,
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
