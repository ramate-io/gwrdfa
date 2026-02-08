use crate::buffer::{facts::Facts, inferences::Inferences, Bufferlike, DraftBufferlike};

/// Specifies the entities and buffers for a parabyzantine broadcast out system.
///
/// A Parabyzantine broadcast out system is concerned with deriving transactions and certificates from broadcasts.
pub trait ParabyzantineBroadcastInSpec {
	type BroadcastEntity: Sized;
	type BroadcastBuffer: Bufferlike<Self::BroadcastEntity>;
	type BroadcastDraftBuffer: DraftBufferlike<Self::BroadcastEntity, Self::BroadcastBuffer>;
	type TransactionEntity: Sized;
	type TransactionBuffer: Bufferlike<Self::TransactionEntity>;
	type TransactionDraftBuffer: DraftBufferlike<Self::TransactionEntity, Self::TransactionBuffer>;
	type CertificateEntity: Sized;
	type CertificateBuffer: Bufferlike<Self::CertificateEntity>;
	type CertificateDraftBuffer: DraftBufferlike<Self::CertificateEntity, Self::CertificateBuffer>;
}
pub trait ParabyzantineBroadcastInWorld<Spec: ParabyzantineBroadcastInSpec> {
	fn broadcast_buffer(&self) -> &Spec::BroadcastBuffer;

	fn broadcast_facts(&self) -> Facts<Spec::BroadcastEntity, Spec::BroadcastBuffer>;

	fn broadcast_draft_buffer(&self) -> Spec::BroadcastDraftBuffer;

	fn broadcast_inferences(
		&self,
	) -> Inferences<Spec::BroadcastEntity, Spec::BroadcastBuffer, Spec::BroadcastDraftBuffer>;

	fn transaction_buffer(&self) -> &Spec::TransactionBuffer;

	fn transaction_facts(&self) -> Facts<Spec::TransactionEntity, Spec::TransactionBuffer>;

	fn transaction_draft_buffer(&self) -> Spec::TransactionDraftBuffer;

	fn transaction_inferences(
		&self,
	) -> Inferences<Spec::TransactionEntity, Spec::TransactionBuffer, Spec::TransactionDraftBuffer>;

	fn certificate_buffer(&self) -> &Spec::CertificateBuffer;

	fn certificate_facts(&self) -> Facts<Spec::CertificateEntity, Spec::CertificateBuffer>;

	fn certificate_draft_buffer(&self) -> Spec::CertificateDraftBuffer;

	fn certificate_inferences(
		&self,
	) -> Inferences<Spec::CertificateEntity, Spec::CertificateBuffer, Spec::CertificateDraftBuffer>;
}

/// Parabyzantine broadcast in systems are concerned with deriving transactions and certificates from broadcasts.
pub trait ParabyzantineBroadcastIn<
	Spec: ParabyzantineBroadcastInSpec,
	BroadcastInWorld: ParabyzantineBroadcastInWorld<Spec>,
>
{
	fn compute_parabyzantine_broadcast_in(&mut self, broadcast_in_world: &mut BroadcastInWorld);
}
