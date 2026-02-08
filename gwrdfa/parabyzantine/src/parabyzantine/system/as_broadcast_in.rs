use crate::parabyzantine::broadcast_in::{
	ParabyzantineBroadcastInSpec, ParabyzantineBroadcastInWorld,
};
use crate::parabyzantine::system::{ParabyzantineSystem, ParabyzantineSystemSpec};

/// Blanket implementation for the agreement spec.
///
/// Downcasting the world to an agreement world.
impl<Spec: ParabyzantineSystemSpec> ParabyzantineBroadcastInSpec for Spec {
	type BroadcastEntity = Spec::BroadcastEntity;
	type BroadcastBuffer = Spec::BroadcastBuffer;
	type BroadcastDraftBuffer = Spec::BroadcastDraftBuffer;
	type CertificateEntity = Spec::CertificateEntity;
	type CertificateBuffer = Spec::CertificateBuffer;
	type CertificateDraftBuffer = Spec::CertificateDraftBuffer;
	type TransactionEntity = Spec::TransactionEntity;
	type TransactionBuffer = Spec::TransactionBuffer;
	type TransactionDraftBuffer = Spec::TransactionDraftBuffer;
}

impl<Spec: ParabyzantineSystemSpec, World: ParabyzantineSystem<Spec>>
	ParabyzantineBroadcastInWorld<Spec> for World
{
	fn broadcast_buffer(&self) -> &Spec::BroadcastBuffer {
		ParabyzantineSystem::broadcast_buffer(self)
	}

	fn broadcast_draft_buffer(&self) -> Spec::BroadcastDraftBuffer {
		ParabyzantineSystem::broadcast_draft_buffer(self)
	}

	fn certificate_buffer(&self) -> &Spec::CertificateBuffer {
		ParabyzantineSystem::certificate_buffer(self)
	}

	fn certificate_draft_buffer(&self) -> Spec::CertificateDraftBuffer {
		ParabyzantineSystem::certificate_draft_buffer(self)
	}

	fn transaction_buffer(&self) -> &Spec::TransactionBuffer {
		ParabyzantineSystem::transaction_buffer(self)
	}

	fn transaction_draft_buffer(&self) -> Spec::TransactionDraftBuffer {
		ParabyzantineSystem::transaction_draft_buffer(self)
	}
}
