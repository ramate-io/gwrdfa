use crate::parabyzantine::broadcast_in::{
	ParabyzantineBroadcastInSpec, ParabyzantineBroadcastInWorld,
};
use crate::parabyzantine::world::{ParabyzantineWorld, ParabyzantineWorldSpec};

/// Blanket implementation for the agreement spec.
///
/// Downcasting the world to an agreement world.
impl<Spec: ParabyzantineWorldSpec> ParabyzantineBroadcastInSpec for Spec {
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

impl<Spec: ParabyzantineWorldSpec, World: ParabyzantineWorld<Spec>>
	ParabyzantineBroadcastInWorld<Spec> for World
{
	fn broadcast_buffer(&self) -> &Spec::BroadcastBuffer {
		ParabyzantineWorld::broadcast_buffer(self)
	}

	fn broadcast_draft_buffer(&self) -> Spec::BroadcastDraftBuffer {
		ParabyzantineWorld::broadcast_draft_buffer(self)
	}

	fn certificate_buffer(&self) -> &Spec::CertificateBuffer {
		ParabyzantineWorld::certificate_buffer(self)
	}

	fn certificate_draft_buffer(&self) -> Spec::CertificateDraftBuffer {
		ParabyzantineWorld::certificate_draft_buffer(self)
	}

	fn transaction_buffer(&self) -> &Spec::TransactionBuffer {
		ParabyzantineWorld::transaction_buffer(self)
	}

	fn transaction_draft_buffer(&self) -> Spec::TransactionDraftBuffer {
		ParabyzantineWorld::transaction_draft_buffer(self)
	}
}
