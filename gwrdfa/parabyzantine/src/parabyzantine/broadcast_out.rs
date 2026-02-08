use crate::buffer::{facts::Facts, inferences::Inferences, Bufferlike, DraftBufferlike};

/// Specifies the entities and buffers for a parabyzantine broadcast out system.
///
/// A Parabyzantine broadcast out system is concerned with deriving broadcasts from tasks.
pub trait ParabyzantineBroadcastOutSpec {
	type TaskEntity: Sized;
	type TaskBuffer: Bufferlike<Self::TaskEntity>;
	type TaskDraftBuffer: DraftBufferlike<Self::TaskEntity, Self::TaskBuffer>;
	type BroadcastEntity: Sized;
	type BroadcastBuffer: Bufferlike<Self::BroadcastEntity>;
	type BroadcastDraftBuffer: DraftBufferlike<Self::BroadcastEntity, Self::BroadcastBuffer>;
}

pub trait ParabyzantineBroadcastOutWorld<Spec: ParabyzantineBroadcastOutSpec> {
	fn task_buffer(&self) -> &Spec::TaskBuffer;

	fn task_facts(&self) -> Facts<Spec::TaskEntity, Spec::TaskBuffer> {
		Facts::new(self.task_buffer())
	}

	fn task_draft_buffer(&self) -> Spec::TaskDraftBuffer;

	fn task_inferences(
		&self,
	) -> Inferences<Spec::TaskEntity, Spec::TaskBuffer, Spec::TaskDraftBuffer> {
		Inferences::new(self.task_draft_buffer())
	}

	fn broadcast_buffer(&self) -> &Spec::BroadcastBuffer;

	fn broadcast_facts(&self) -> Facts<Spec::BroadcastEntity, Spec::BroadcastBuffer> {
		Facts::new(self.broadcast_buffer())
	}

	fn broadcast_draft_buffer(&self) -> Spec::BroadcastDraftBuffer;

	fn broadcast_inferences(
		&self,
	) -> Inferences<Spec::BroadcastEntity, Spec::BroadcastBuffer, Spec::BroadcastDraftBuffer> {
		Inferences::new(self.broadcast_draft_buffer())
	}
}

/// Parabyzantine broadcast out systems are concerned with deriving broadcasts from tasks.
pub trait ParabyzantineBroadcastOut<
	Spec: ParabyzantineBroadcastOutSpec,
	BroadcastOutWorld: ParabyzantineBroadcastOutWorld<Spec>,
>
{
	fn compute_parabyzantine_broadcast_out(&mut self, broadcast_out_world: &mut BroadcastOutWorld);
}
