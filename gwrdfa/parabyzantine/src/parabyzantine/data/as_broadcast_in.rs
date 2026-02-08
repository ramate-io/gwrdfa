use crate::parabyzantine::broadcast_in::ParabyzantineBroadcastInSpec;
use crate::parabyzantine::data::{ParabyzantineData, ParabyzantineSpec};

/// Blanket implementation for the broadcast in spec.
///
/// Downcasting the world to a broadcast in world.
impl<Spec: ParabyzantineSpec<Data>, Data: ParabyzantineData<Spec>>
	ParabyzantineBroadcastInSpec<Data> for Spec
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
