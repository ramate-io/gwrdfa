use crate::parabyzantine::broadcast_out::{
	ParabyzantineBroadcastOutData, ParabyzantineBroadcastOutSpec,
};
use crate::parabyzantine::data::{ParabyzantineData, ParabyzantineSpec};

/// Blanket implementation for the broadcast out spec.
///
/// Downcasting the world to a broadcast out world.
impl<Spec: ParabyzantineSpec> ParabyzantineBroadcastOutSpec for Spec {
	type TaskEntity = Spec::TaskEntity;
	type TaskBuffer = Spec::TaskBuffer;
	type TaskDraftBuffer = Spec::TaskDraftBuffer;
	type BroadcastEntity = Spec::BroadcastEntity;
	type BroadcastBuffer = Spec::BroadcastBuffer;
	type BroadcastDraftBuffer = Spec::BroadcastDraftBuffer;
}

/// Blanket implementation for the broadcast out data.
impl<Spec: ParabyzantineSpec, Data: ParabyzantineData<Spec>> ParabyzantineBroadcastOutData<Spec>
	for Data
{
	fn parabyzantine_broadcast_out_task_buffer(&self) -> &Spec::TaskBuffer {
		self.parabyzantine_task_buffer()
	}
	fn parabyzantine_broadcast_out_task_draft_buffer(&self) -> Spec::TaskDraftBuffer {
		self.parabyzantine_task_draft_buffer()
	}
	fn parabyzantine_broadcast_out_broadcast_buffer(&self) -> &Spec::BroadcastBuffer {
		self.parabyzantine_broadcast_buffer()
	}
	fn parabyzantine_broadcast_out_broadcast_draft_buffer(&self) -> Spec::BroadcastDraftBuffer {
		self.parabyzantine_broadcast_draft_buffer()
	}
}
