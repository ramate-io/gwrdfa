use crate::buffer::{facts::Facts, inferences::Inferences, Bufferlike, DraftBufferlike};

#[derive(Debug, Clone, Copy)]
pub struct BroadcastIn;

/// Specifies the entities and buffers for a parabyzantine broadcast in Data.
///
/// A Parabyzantine broadcast in Data is concerned with deriving transactions and certificates from broadcasts.
///
/// Mainly, what a system implemented on this kind of data will do is
/// to look at all the broadcasts and determine which ones come from:
/// 1. Outside the system, in which case they are transactions
/// 2. Inside the system, in which case they are certificates
pub trait ParabyzantineBroadcastInDataSpec: Sized {
	/// The entity type for the broadcast.
	type BroadcastEntity: Sized;
	/// The buffer type for the broadcast.
	type BroadcastBuffer: Bufferlike<Self::BroadcastEntity>;
	/// The draft buffer type for the broadcast.
	type BroadcastDraftBuffer: DraftBufferlike<Self::BroadcastEntity, Self::BroadcastBuffer>;

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

pub trait ParabyzantineBroadcastInData<Spec: ParabyzantineBroadcastInDataSpec>: Sized {
	/// The buffer for the broadcast.
	fn parabyzantine_broadcast_in_broadcast_buffer(&self) -> &Spec::BroadcastBuffer;
	/// The draft buffer for the broadcast.
	fn parabyzantine_broadcast_in_broadcast_draft_buffer(&self) -> Spec::BroadcastDraftBuffer;
	/// The buffer for the transaction.
	fn parabyzantine_broadcast_in_transaction_buffer(&self) -> &Spec::TransactionBuffer;
	/// The draft buffer for the transaction.
	fn parabyzantine_broadcast_in_transaction_draft_buffer(&self) -> Spec::TransactionDraftBuffer;
	/// The buffer for the certificate.
	fn parabyzantine_broadcast_in_certificate_buffer(&self) -> &Spec::CertificateBuffer;
	/// The draft buffer for the certificate.
	fn parabyzantine_broadcast_in_certificate_draft_buffer(&self) -> Spec::CertificateDraftBuffer;

	fn parabyzantine_broadcast_in_world(&self) -> BroadcastInWorld<Spec> {
		BroadcastInWorld {
			broadcast_facts: self.parabyzantine_broadcast_in_broadcast_buffer().into(),
			broadcast_inferences: self.parabyzantine_broadcast_in_broadcast_draft_buffer().into(),
			transaction_facts: self.parabyzantine_broadcast_in_transaction_buffer().into(),
			transaction_inferences: self
				.parabyzantine_broadcast_in_transaction_draft_buffer()
				.into(),
			certificate_facts: self.parabyzantine_broadcast_in_certificate_buffer().into(),
			certificate_inferences: self
				.parabyzantine_broadcast_in_certificate_draft_buffer()
				.into(),
		}
	}
}

/// The world of the broadcast in step of a parabyzantine broadcast in Data.
pub struct BroadcastInWorld<'a, Spec: ParabyzantineBroadcastInDataSpec> {
	pub broadcast_facts: Facts<'a, Spec::BroadcastEntity, Spec::BroadcastBuffer>,
	pub broadcast_inferences:
		Inferences<Spec::BroadcastEntity, Spec::BroadcastBuffer, Spec::BroadcastDraftBuffer>,
	pub transaction_facts: Facts<'a, Spec::TransactionEntity, Spec::TransactionBuffer>,
	pub transaction_inferences:
		Inferences<Spec::TransactionEntity, Spec::TransactionBuffer, Spec::TransactionDraftBuffer>,
	pub certificate_facts: Facts<'a, Spec::CertificateEntity, Spec::CertificateBuffer>,
	pub certificate_inferences:
		Inferences<Spec::CertificateEntity, Spec::CertificateBuffer, Spec::CertificateDraftBuffer>,
}

pub trait ParabyzantineBroadcastIn: Sized {
	type Spec: ParabyzantineBroadcastInDataSpec;

	/// Compute the parabyzantine broadcast in.
	fn compute_parabyzantine_broadcast_in(&mut self, data: &mut BroadcastInWorld<Self::Spec>);
}

/// A [ParabyzantineBroadcastInDataBinding] is a binding for the [ParabyzantineBroadcastIn] protocol.
///
/// It binds between the [ParabyzantineBroadcastInDataSpec] and the [ParabyzantineBroadcastInData].
pub trait ParabyzantineBroadcastInDataBinding {
	type Spec: ParabyzantineBroadcastInDataSpec;
	type Data: ParabyzantineBroadcastInData<Self::Spec>;
}
