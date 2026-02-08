use crate::parabyzantine::broadcast_in::{
	ParabyzantineBroadcastInSpec, ParabyzantineBroadcastInSystem,
};
use crate::parabyzantine::system::{ParabyzantineSpec, ParabyzantineSystem};

/// Blanket implementation for the broadcast in spec.
///
/// Downcasting the world to a broadcast in world.
impl<Spec: ParabyzantineSpec<System>, System: ParabyzantineSystem<Spec>>
	ParabyzantineBroadcastInSpec<System> for Spec
{
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

/// Blanket implementation for the broadcast in system.
impl<Spec: ParabyzantineSpec<System>, System: ParabyzantineSystem<Spec>>
	ParabyzantineBroadcastInSystem<Spec> for System
{
}
