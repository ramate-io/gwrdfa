use crate::parabyzantine::broadcast_out::ParabyzantineBroadcastOutSpec;
use crate::parabyzantine::data::{ParabyzantineData, ParabyzantineSpec};

/// Blanket implementation for the broadcast out spec.
///
/// Downcasting the world to a broadcast out world.
impl<Spec: ParabyzantineSpec<Data>, Data: ParabyzantineData<Spec>>
	ParabyzantineBroadcastOutSpec<Data> for Spec
{
	type TaskEntity = Spec::TaskEntity;
	type TaskBuffer = Spec::TaskBuffer;
	type TaskDraftBuffer = Spec::TaskDraftBuffer;
	type BroadcastEntity = Spec::BroadcastEntity;
	type BroadcastBuffer = Spec::BroadcastBuffer;
	type BroadcastDraftBuffer = Spec::BroadcastDraftBuffer;
}
