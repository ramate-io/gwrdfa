use crate::act::Act;
use crate::buffer::{facts::Facts, inferences::Inferences, Bufferlike, DraftBufferlike};
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
	/// The draft buffer type for the agreement.
	type AgreementDraftBuffer: DraftBufferlike<Self::AgreementEntity, Self::AgreementBuffer>;

	/// The entity type for the transaction.
	type TransactionEntity: Sized;
	/// The buffer type for the transaction.
	type TransactionBuffer: Bufferlike<Self::TransactionEntity>;
	/// The draft buffer type for the transaction.
	type TransactionDraftBuffer: DraftBufferlike<Self::TransactionEntity, Self::TransactionBuffer>;

	/// The entity type for the task.
	type TaskEntity: Sized;
	/// The buffer type for the task.
	type TaskBuffer: Bufferlike<Self::TaskEntity>;
	/// The draft buffer type for the task.
	type TaskDraftBuffer: DraftBufferlike<Self::TaskEntity, Self::TaskBuffer>;
}

pub trait ParabyzantineTaskData<Spec: ParabyzantineTaskDataSpec>: Sized {
	/// The buffer for the agreement.
	fn parabyzantine_task_agreement_buffer(&self) -> &Spec::AgreementBuffer;

	/// The draft buffer for the agreement.
	fn parabyzantine_task_agreement_buffer_mut(&mut self) -> &mut Spec::AgreementBuffer;

	/// The draft buffer for the agreement.
	fn parabyzantine_task_agreement_draft_buffer(&self) -> Spec::AgreementDraftBuffer;
	/// The buffer for the transaction.
	fn parabyzantine_task_transaction_buffer(&self) -> &Spec::TransactionBuffer;

	/// The draft buffer for the transaction.
	fn parabyzantine_task_transaction_buffer_mut(&mut self) -> &mut Spec::TransactionBuffer;

	/// The draft buffer for the transaction.
	fn parabyzantine_task_transaction_draft_buffer(&self) -> Spec::TransactionDraftBuffer;

	/// The buffer for the task.
	fn parabyzantine_task_task_buffer(&self) -> &Spec::TaskBuffer;

	/// The draft buffer for the task.
	fn parabyzantine_task_task_buffer_mut(&mut self) -> &mut Spec::TaskBuffer;

	/// The draft buffer for the task.
	fn parabyzantine_task_task_draft_buffer(&self) -> Spec::TaskDraftBuffer;

	/// The world of the task.
	fn parabyzantine_task_world(&self) -> TaskWorld<Spec> {
		TaskWorld {
			agreement_facts: self.parabyzantine_task_agreement_buffer().into(),
			agreement_inferences: self.parabyzantine_task_agreement_draft_buffer().into(),
			transaction_facts: self.parabyzantine_task_transaction_buffer().into(),
			transaction_inferences: self.parabyzantine_task_transaction_draft_buffer().into(),
			task_facts: self.parabyzantine_task_task_buffer().into(),
			task_inferences: self.parabyzantine_task_task_draft_buffer().into(),
		}
	}

	fn commit_parabyzantine_task(&mut self, task_inferences: TaskInferences<Spec>) {
		self.parabyzantine_task_agreement_buffer_mut()
			.commit_inferences(task_inferences.agreement_inferences);
		self.parabyzantine_task_transaction_buffer_mut()
			.commit_inferences(task_inferences.transaction_inferences);
		self.parabyzantine_task_task_buffer_mut()
			.commit_inferences(task_inferences.task_inferences);
	}
}

/// The world of the task step of a parabyzantine task Data.
pub struct TaskWorld<'a, Spec: ParabyzantineTaskDataSpec> {
	pub agreement_facts: Facts<'a, Spec::AgreementEntity, Spec::AgreementBuffer>,
	pub agreement_inferences:
		Inferences<Spec::AgreementEntity, Spec::AgreementBuffer, Spec::AgreementDraftBuffer>,
	pub transaction_facts: Facts<'a, Spec::TransactionEntity, Spec::TransactionBuffer>,
	pub transaction_inferences:
		Inferences<Spec::TransactionEntity, Spec::TransactionBuffer, Spec::TransactionDraftBuffer>,
	pub task_facts: Facts<'a, Spec::TaskEntity, Spec::TaskBuffer>,
	pub task_inferences: Inferences<Spec::TaskEntity, Spec::TaskBuffer, Spec::TaskDraftBuffer>,
}

/// The inferences for the task step of a parabyzantine task Data.
pub struct TaskInferences<Spec: ParabyzantineTaskDataSpec> {
	pub agreement_inferences:
		Inferences<Spec::AgreementEntity, Spec::AgreementBuffer, Spec::AgreementDraftBuffer>,
	pub transaction_inferences:
		Inferences<Spec::TransactionEntity, Spec::TransactionBuffer, Spec::TransactionDraftBuffer>,
	pub task_inferences: Inferences<Spec::TaskEntity, Spec::TaskBuffer, Spec::TaskDraftBuffer>,
}

impl<'a, Spec: ParabyzantineTaskDataSpec> From<TaskWorld<'a, Spec>> for TaskInferences<Spec> {
	fn from(world: TaskWorld<'a, Spec>) -> Self {
		TaskInferences {
			agreement_inferences: world.agreement_inferences,
			transaction_inferences: world.transaction_inferences,
			task_inferences: world.task_inferences,
		}
	}
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

	/// Commits the inferences for the parabyzantine task.
	fn commit_parabyzantine_task(
		&mut self,
		task_inferences: TaskInferences<<Self::Binding as ParabyzantineTaskDataBinding>::Spec>,
		data: &mut <Self::Binding as ParabyzantineTaskDataBinding>::Data,
	) {
		data.commit_parabyzantine_task(task_inferences);
	}
}

impl<Binding: ParabyzantineTaskDataBinding, TaskHandler: ParabyzantineTask<Binding = Binding>>
	Act<Task, Binding::Data> for TaskHandler
{
	fn act(&mut self, _action: Task, data: &mut Binding::Data) {
		let mut world = self.parabyzantine_task_world(data);
		self.update_parabyzantine_task(&mut world);
		self.commit_parabyzantine_task(world.into(), data);
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
