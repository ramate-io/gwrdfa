use crate::parabyzantine::broadcast_in::{
	ParabyzantineBroadcastInData, ParabyzantineBroadcastInSpec,
};
use crate::parabyzantine::data::{ParabyzantineData, ParabyzantineSpec};

/// Blanket implementation for the broadcast in spec.
///
/// Downcasting the world to a broadcast in world.
impl<Spec: ParabyzantineSpec> ParabyzantineBroadcastInSpec for Spec {
	type BroadcastEntity = Spec::BroadcastEntity;
	type BroadcastBuffer = Spec::BroadcastBuffer;
	type BroadcastDraftBuffer = Spec::BroadcastDraftBuffer;
	type TransactionEntity = Spec::TransactionEntity;
	type TransactionBuffer = Spec::TransactionBuffer;
	type TransactionDraftBuffer = Spec::TransactionDraftBuffer;
	type CertificateEntity = Spec::CertificateEntity;
	type CertificateBuffer = Spec::CertificateBuffer;
	type CertificateDraftBuffer = Spec::CertificateDraftBuffer;
}

/// Blanket implementation for the broadcast in data.
impl<Spec: ParabyzantineSpec, Data: ParabyzantineData<Spec>> ParabyzantineBroadcastInData<Spec>
	for Data
{
	fn parabyzantine_broadcast_in_broadcast_buffer(&self) -> &Spec::BroadcastBuffer {
		self.parabyzantine_broadcast_buffer()
	}
	fn parabyzantine_broadcast_in_broadcast_draft_buffer(&self) -> Spec::BroadcastDraftBuffer {
		self.parabyzantine_broadcast_draft_buffer()
	}
	fn parabyzantine_broadcast_in_transaction_buffer(&self) -> &Spec::TransactionBuffer {
		self.parabyzantine_transaction_buffer()
	}
	fn parabyzantine_broadcast_in_transaction_draft_buffer(&self) -> Spec::TransactionDraftBuffer {
		self.parabyzantine_transaction_draft_buffer()
	}
	fn parabyzantine_broadcast_in_certificate_buffer(&self) -> &Spec::CertificateBuffer {
		self.parabyzantine_certificate_buffer()
	}
	fn parabyzantine_broadcast_in_certificate_draft_buffer(&self) -> Spec::CertificateDraftBuffer {
		self.parabyzantine_certificate_draft_buffer()
	}
}
