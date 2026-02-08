use crate::buffer::{facts::Facts, inferences::Inferences, Bufferlike, DraftBufferlike};
use crate::{Container, Factory, Member, Product, View};

/// Specifies the entities and buffers for a parabyzantine broadcast out Data.
///
/// A Parabyzantine broadcast out Data is concerned with deriving broadcasts from tasks.
pub trait ParabyzantineBroadcastOutSpec<Data: ParabyzantineBroadcastOutData<Self>>: Sized {
	/// The entity type for the task.
	type TaskEntity: Sized;
	/// The buffer type for the task.
	type TaskBuffer: Bufferlike<Self::TaskEntity> + Member<Data>;
	/// The draft buffer type for the task.
	type TaskDraftBuffer: DraftBufferlike<Self::TaskEntity, Self::TaskBuffer> + Product<Data>;

	/// The entity type for the broadcast.
	type BroadcastEntity: Sized;
	/// The buffer type for the broadcast.
	type BroadcastBuffer: Bufferlike<Self::BroadcastEntity> + Member<Data>;
	/// The draft buffer type for the broadcast.
	type BroadcastDraftBuffer: DraftBufferlike<Self::BroadcastEntity, Self::BroadcastBuffer>
		+ Product<Data>;
}

pub trait ParabyzantineBroadcastOutData<Spec: ParabyzantineBroadcastOutSpec<Self>>: Sized {
	fn parabyzantine_broadcast_out_world(&self) -> BroadcastOutWorld<Spec, Self>;
}

/// Blanket implementation for the broadcast out Data when members are available.
impl<Spec: ParabyzantineBroadcastOutSpec<Data>, Data> ParabyzantineBroadcastOutData<Spec> for Data
where
	Data: Sized,
	Spec::TaskBuffer: Member<Data>,
	Spec::TaskDraftBuffer: Product<Data>,
	Spec::BroadcastBuffer: Member<Data>,
	Spec::BroadcastDraftBuffer: Product<Data>,
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

/// The world of the broadcast out step of a parabyzantine broadcast out Data.
pub struct BroadcastOutWorld<
	'a,
	Spec: ParabyzantineBroadcastOutSpec<Data>,
	Data: ParabyzantineBroadcastOutData<Spec>,
> {
	pub task_facts: Facts<'a, Spec::TaskEntity, Spec::TaskBuffer>,
	pub task_inferences: Inferences<Spec::TaskEntity, Spec::TaskBuffer, Spec::TaskDraftBuffer>,
	pub broadcast_facts: Facts<'a, Spec::BroadcastEntity, Spec::BroadcastBuffer>,
	pub broadcast_inferences:
		Inferences<Spec::BroadcastEntity, Spec::BroadcastBuffer, Spec::BroadcastDraftBuffer>,
}

/// View the world of a parabyzantine broadcast out Data.
///
/// This is implemented for ergonomics so that the user can write in the same style if they so choose.
impl<'a, Spec: ParabyzantineBroadcastOutSpec<Data>, Data: ParabyzantineBroadcastOutData<Spec>>
	View<'a, Data> for BroadcastOutWorld<'a, Spec, Data>
{
	fn view(from: &'a Data) -> Self {
		from.parabyzantine_broadcast_out_world()
	}
}
