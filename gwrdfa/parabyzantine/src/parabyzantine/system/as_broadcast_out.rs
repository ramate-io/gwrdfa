use crate::parabyzantine::broadcast_out::{
	ParabyzantineBroadcastOutSpec, ParabyzantineBroadcastOutSystem,
};
use crate::parabyzantine::system::{ParabyzantineSpec, ParabyzantineSystem};

/// Blanket implementation for the broadcast out spec.
///
/// Downcasting the world to a broadcast out world.
impl<Spec: ParabyzantineSpec<System>, System: ParabyzantineSystem<Spec>>
	ParabyzantineBroadcastOutSpec<System> for Spec
{
	type TaskEntity = Spec::TaskEntity;
	type TaskBuffer = Spec::TaskBuffer;
	type TaskDraftBuffer = Spec::TaskDraftBuffer;
	type BroadcastEntity = Spec::BroadcastEntity;
	type BroadcastBuffer = Spec::BroadcastBuffer;
	type BroadcastDraftBuffer = Spec::BroadcastDraftBuffer;
}

/// Blanket implementation for the broadcast out system.
impl<Spec: ParabyzantineSpec<System>, System: ParabyzantineSystem<Spec>>
	ParabyzantineBroadcastOutSystem<Spec> for System
{
}
