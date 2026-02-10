use crate::buffer::{facts::Facts, inferences::Inferences, Bufferlike, DraftBufferlike};

/// The schedule for the prepare step of the parabyzantine broadcast out.
#[derive(Debug, Clone, Copy)]
pub struct PrepareParabyzantineBroadcastOut;

/// The schedule for the compute step of the parabyzantine broadcast out.
#[derive(Debug, Clone, Copy)]
pub struct ComputeParabyzantineBroadcastOut;

/// The schedule for the commit step of the parabyzantine broadcast out.

#[derive(Debug, Clone, Copy)]
pub struct CommitParabyzantineBroadcastOut;

/// Specifies the entities and buffers for a parabyzantine broadcast out Data.
///
/// A Parabyzantine broadcast out Data is concerned with deriving broadcasts from tasks.
pub trait ParabyzantineBroadcastOutSpec: Sized {
	/// The entity type for the task.
	type TaskEntity: Sized;
	/// The buffer type for the task.
	type TaskBuffer: Bufferlike<Self::TaskEntity>;
	/// The draft buffer type for the task.
	type TaskDraftBuffer: DraftBufferlike<Self::TaskEntity, Self::TaskBuffer>;

	/// The entity type for the broadcast.
	type BroadcastEntity: Sized;
	/// The buffer type for the broadcast.
	type BroadcastBuffer: Bufferlike<Self::BroadcastEntity>;
	/// The draft buffer type for the broadcast.
	type BroadcastDraftBuffer: DraftBufferlike<Self::BroadcastEntity, Self::BroadcastBuffer>;
}

pub trait ParabyzantineBroadcastOutData<Spec: ParabyzantineBroadcastOutSpec>: Sized {
	/// The buffer for the task.
	fn parabyzantine_broadcast_out_task_buffer(&self) -> &Spec::TaskBuffer;
	/// The draft buffer for the task.
	fn parabyzantine_broadcast_out_task_draft_buffer(&self) -> Spec::TaskDraftBuffer;
	/// The buffer for the broadcast.
	fn parabyzantine_broadcast_out_broadcast_buffer(&self) -> &Spec::BroadcastBuffer;
	/// The draft buffer for the broadcast.
	fn parabyzantine_broadcast_out_broadcast_draft_buffer(&self) -> Spec::BroadcastDraftBuffer;

	/// The world of the broadcast out.
	fn parabyzantine_broadcast_out_world(&self) -> BroadcastOutWorld<Spec> {
		BroadcastOutWorld {
			task_facts: self.parabyzantine_broadcast_out_task_buffer().into(),
			task_inferences: self.parabyzantine_broadcast_out_task_draft_buffer().into(),
			broadcast_facts: self.parabyzantine_broadcast_out_broadcast_buffer().into(),
			broadcast_inferences: self.parabyzantine_broadcast_out_broadcast_draft_buffer().into(),
		}
	}
}
/// The world of the broadcast out step of a parabyzantine broadcast out Data.
pub struct BroadcastOutWorld<'a, Spec: ParabyzantineBroadcastOutSpec> {
	pub task_facts: Facts<'a, Spec::TaskEntity, Spec::TaskBuffer>,
	pub task_inferences: Inferences<Spec::TaskEntity, Spec::TaskBuffer, Spec::TaskDraftBuffer>,
	pub broadcast_facts: Facts<'a, Spec::BroadcastEntity, Spec::BroadcastBuffer>,
	pub broadcast_inferences:
		Inferences<Spec::BroadcastEntity, Spec::BroadcastBuffer, Spec::BroadcastDraftBuffer>,
}

pub trait ParabyzantineBroadcastOut<Spec: ParabyzantineBroadcastOutSpec>: Sized {
	/// Compute the parabyzantine broadcast out.
	fn compute_parabyzantine_broadcast_out(&mut self, data: &mut BroadcastOutWorld<Spec>);
}

/// A [ParabyzantineBroadcastOutBinding] is a binding for the [ParabyzantineBroadcastOut] protocol.
///
/// It binds between the [ParabyzantineBroadcastOutSpec] and the [ParabyzantineBroadcastOutData].
pub trait ParabyzantineBroadcastOutBinding {
	type Spec: ParabyzantineBroadcastOutSpec;
	type Data: ParabyzantineBroadcastOutData<Self::Spec>;
}
