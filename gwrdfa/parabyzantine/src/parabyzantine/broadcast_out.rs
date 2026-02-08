use crate::buffer::{facts::Facts, inferences::Inferences, Bufferlike, DraftBufferlike};
use crate::{Container, Factory, Member, Product, View};

/// Specifies the entities and buffers for a parabyzantine broadcast out system.
///
/// A Parabyzantine broadcast out system is concerned with deriving broadcasts from tasks.
pub trait ParabyzantineBroadcastOutSpec<System: ParabyzantineBroadcastOutSystem<Self>>:
	Sized
{
	/// The entity type for the task.
	type TaskEntity: Sized;
	/// The buffer type for the task.
	type TaskBuffer: Bufferlike<Self::TaskEntity> + Member<System>;
	/// The draft buffer type for the task.
	type TaskDraftBuffer: DraftBufferlike<Self::TaskEntity, Self::TaskBuffer> + Product<System>;

	/// The entity type for the broadcast.
	type BroadcastEntity: Sized;
	/// The buffer type for the broadcast.
	type BroadcastBuffer: Bufferlike<Self::BroadcastEntity> + Member<System>;
	/// The draft buffer type for the broadcast.
	type BroadcastDraftBuffer: DraftBufferlike<Self::BroadcastEntity, Self::BroadcastBuffer>
		+ Product<System>;
}

pub trait ParabyzantineBroadcastOutSystem<Spec: ParabyzantineBroadcastOutSpec<Self>>:
	Sized
{
	fn parabyzantine_broadcast_out_world(&self) -> BroadcastOutWorld<Spec, Self> {
		BroadcastOutWorld {
			task_facts: self.member::<Spec::TaskBuffer>().into(),
			task_inferences: self.produce::<Spec::TaskDraftBuffer>().into(),
			broadcast_facts: self.member::<Spec::BroadcastBuffer>().into(),
			broadcast_inferences: self.produce::<Spec::BroadcastDraftBuffer>().into(),
		}
	}
}

/// The world of the broadcast out step of a parabyzantine broadcast out system.
pub struct BroadcastOutWorld<
	'a,
	Spec: ParabyzantineBroadcastOutSpec<System>,
	System: ParabyzantineBroadcastOutSystem<Spec>,
> {
	pub task_facts: Facts<'a, Spec::TaskEntity, Spec::TaskBuffer>,
	pub task_inferences: Inferences<Spec::TaskEntity, Spec::TaskBuffer, Spec::TaskDraftBuffer>,
	pub broadcast_facts: Facts<'a, Spec::BroadcastEntity, Spec::BroadcastBuffer>,
	pub broadcast_inferences:
		Inferences<Spec::BroadcastEntity, Spec::BroadcastBuffer, Spec::BroadcastDraftBuffer>,
}

/// View the world of a parabyzantine broadcast out system.
///
/// This is implemented for ergonomics so that the user can write in the same style if they so choose.
impl<
		'a,
		Spec: ParabyzantineBroadcastOutSpec<System>,
		System: ParabyzantineBroadcastOutSystem<Spec>,
	> View<'a, System> for BroadcastOutWorld<'a, Spec, System>
{
	fn view(from: &'a System) -> Self {
		from.parabyzantine_broadcast_out_world()
	}
}
