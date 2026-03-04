use crate::act::Act;
use crate::buffer::{facts::Facts, inferences::Inferences, Bufferlike, DraftBufferlike};

#[derive(Debug, Clone, Copy)]
pub struct Task;

pub trait ParabyzantineTaskData: Sized {
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

	/// The buffer for the agreement.
	fn parabyzantine_task_agreement_buffer(&self) -> &Self::AgreementBuffer;

	/// The draft buffer for the agreement.
	fn parabyzantine_task_agreement_buffer_mut(&mut self) -> &mut Self::AgreementBuffer;

	/// The draft buffer for the agreement.
	fn parabyzantine_task_agreement_draft_buffer(&self) -> Self::AgreementDraftBuffer;
	/// The buffer for the transaction.
	fn parabyzantine_task_transaction_buffer(&self) -> &Self::TransactionBuffer;

	/// The draft buffer for the transaction.
	fn parabyzantine_task_transaction_buffer_mut(&mut self) -> &mut Self::TransactionBuffer;

	/// The draft buffer for the transaction.
	fn parabyzantine_task_transaction_draft_buffer(&self) -> Self::TransactionDraftBuffer;

	/// The buffer for the task.
	fn parabyzantine_task_task_buffer(&self) -> &Self::TaskBuffer;

	/// The draft buffer for the task.
	fn parabyzantine_task_task_buffer_mut(&mut self) -> &mut Self::TaskBuffer;

	/// The draft buffer for the task.
	fn parabyzantine_task_task_draft_buffer(&self) -> Self::TaskDraftBuffer;

	/// The world of the task.
	fn parabyzantine_task_world<'a>(&'a self) -> TaskWorld<'a, Self> {
		TaskWorld {
			agreement_facts: self.parabyzantine_task_agreement_buffer().into(),
			agreement_inferences: self.parabyzantine_task_agreement_draft_buffer().into(),
			transaction_facts: self.parabyzantine_task_transaction_buffer().into(),
			transaction_inferences: self.parabyzantine_task_transaction_draft_buffer().into(),
			task_facts: self.parabyzantine_task_task_buffer().into(),
			task_inferences: self.parabyzantine_task_task_draft_buffer().into(),
		}
	}

	fn commit_parabyzantine_task(&mut self, task_inferences: TaskInferences<Self>) {
		self.parabyzantine_task_agreement_buffer_mut()
			.commit_inferences(task_inferences.agreement_inferences);
		self.parabyzantine_task_transaction_buffer_mut()
			.commit_inferences(task_inferences.transaction_inferences);
		self.parabyzantine_task_task_buffer_mut()
			.commit_inferences(task_inferences.task_inferences);
	}
}

/// The world of the task step of a parabyzantine task Data.
pub struct TaskWorld<'a, Data: ParabyzantineTaskData> {
	pub agreement_facts: Facts<'a, Data::AgreementEntity, Data::AgreementBuffer>,
	pub agreement_inferences:
		Inferences<Data::AgreementEntity, Data::AgreementBuffer, Data::AgreementDraftBuffer>,
	pub transaction_facts: Facts<'a, Data::TransactionEntity, Data::TransactionBuffer>,
	pub transaction_inferences:
		Inferences<Data::TransactionEntity, Data::TransactionBuffer, Data::TransactionDraftBuffer>,
	pub task_facts: Facts<'a, Data::TaskEntity, Data::TaskBuffer>,
	pub task_inferences: Inferences<Data::TaskEntity, Data::TaskBuffer, Data::TaskDraftBuffer>,
}

/// The inferences for the task step of a parabyzantine task Data.
pub struct TaskInferences<Data: ParabyzantineTaskData> {
	pub agreement_inferences:
		Inferences<Data::AgreementEntity, Data::AgreementBuffer, Data::AgreementDraftBuffer>,
	pub transaction_inferences:
		Inferences<Data::TransactionEntity, Data::TransactionBuffer, Data::TransactionDraftBuffer>,
	pub task_inferences: Inferences<Data::TaskEntity, Data::TaskBuffer, Data::TaskDraftBuffer>,
}

impl<'a, Data: ParabyzantineTaskData> From<TaskWorld<'a, Data>> for TaskInferences<Data> {
	fn from(world: TaskWorld<'a, Data>) -> Self {
		TaskInferences {
			agreement_inferences: world.agreement_inferences,
			transaction_inferences: world.transaction_inferences,
			task_inferences: world.task_inferences,
		}
	}
}

pub trait ParabyzantineTask<Data: ParabyzantineTaskData>: Sized {
	/// Compute the parabyzantine task.
	fn update_parabyzantine_task(&mut self, data: &mut TaskWorld<Data>);

	/// Commits the inferences for the parabyzantine task.
	fn commit_parabyzantine_task(
		&mut self,
		task_inferences: TaskInferences<Data>,
		data: &mut Data,
	) {
		data.commit_parabyzantine_task(task_inferences);
	}
}

impl<Data: ParabyzantineTaskData, TaskHandler: ParabyzantineTask<Data>> Act<Task, Data>
	for TaskHandler
{
	fn act(&mut self, _action: Task, data: &mut Data) {
		let mut world = self.parabyzantine_task_world(data);
		self.update_parabyzantine_task(&mut world);
		self.commit_parabyzantine_task(world.into(), data);
	}
}
