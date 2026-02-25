use crate::act::Act;
use crate::buffer::{facts::Facts, Bufferlike};
use crate::NoOp;
use crate::NoOpData;

#[derive(Debug, Clone, Copy)]
pub struct Task;

/// Specifies the entities and buffers for a parabyzantine task Data.
///
/// A Parabyzantine task Data is concerned with deriving tasks from agreements and transactions.
pub trait ParabyzantineTaskDataSpec: Sized {
	/// The entity type for the agreement.
	type AgreementEntity: Sized;
	/// The buffer type for the agreement.
	type AgreementBuffer: Bufferlike<Self::AgreementEntity>;

	/// The entity type for the transaction.
	type TransactionEntity: Sized;
	/// The buffer type for the transaction.
	type TransactionBuffer: Bufferlike<Self::TransactionEntity>;

	/// The entity type for the task.
	type TaskEntity: Sized;
	/// The buffer type for the task.
	type TaskBuffer: Bufferlike<Self::TaskEntity>;
}

pub trait ParabyzantineTaskData<Spec: ParabyzantineTaskDataSpec>: Sized {
	/// The world of the task.
	fn parabyzantine_task_world<'a>(&'a mut self) -> TaskWorld<'a, Spec>;
}

/// The world of the task step of a parabyzantine task Data.
pub struct TaskWorld<'a, Spec: ParabyzantineTaskDataSpec> {
	pub agreement_facts: Facts<'a, Spec::AgreementEntity, Spec::AgreementBuffer>,
	pub transaction_facts: Facts<'a, Spec::TransactionEntity, Spec::TransactionBuffer>,
	pub task_facts: Facts<'a, Spec::TaskEntity, Spec::TaskBuffer>,
}

pub trait ParabyzantineTask: Sized {
	type Binding: ParabyzantineTaskDataBinding;

	/// Gets the [TaskWorld] for the parabyzantine task.
	fn parabyzantine_task_world<'a>(
		&mut self,
		data: &'a mut <Self::Binding as ParabyzantineTaskDataBinding>::Data,
	) -> TaskWorld<'a, <Self::Binding as ParabyzantineTaskDataBinding>::Spec> {
		data.parabyzantine_task_world()
	}

	/// Compute the parabyzantine task.
	fn update_parabyzantine_task(
		&mut self,
		data: &mut TaskWorld<<Self::Binding as ParabyzantineTaskDataBinding>::Spec>,
	);
}

impl<Binding: ParabyzantineTaskDataBinding, TaskHandler: ParabyzantineTask<Binding = Binding>>
	Act<Task, Binding::Data> for TaskHandler
{
	fn act(&mut self, _action: Task, data: &mut Binding::Data) {
		let mut world = self.parabyzantine_task_world(data);
		self.update_parabyzantine_task(&mut world);
	}
}

/// A [ParabyzantineTaskDataBinding] is a binding for the [ParabyzantineTask] protocol.
///
/// It binds between the [ParabyzantineTaskDataSpec] and the [ParabyzantineTaskData].
pub trait ParabyzantineTaskDataBinding {
	type Spec: ParabyzantineTaskDataSpec;
	type Data: ParabyzantineTaskData<Self::Spec>;
}

impl ParabyzantineTaskDataBinding for NoOp {
	type Spec = NoOp;
	type Data = NoOpData;
}
