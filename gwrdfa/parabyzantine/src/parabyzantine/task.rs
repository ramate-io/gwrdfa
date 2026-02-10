use crate::buffer::{facts::Facts, inferences::Inferences, Bufferlike, DraftBufferlike};

/// The schedule for the prepare step of the parabyzantine task.
#[derive(Debug, Clone, Copy)]
pub struct PrepareParabyzantineTask;

/// The schedule for the compute step of the parabyzantine task.
#[derive(Debug, Clone, Copy)]
pub struct ComputeParabyzantineTask;

/// The schedule for the commit step of the parabyzantine task.
#[derive(Debug, Clone, Copy)]
pub struct CommitParabyzantineTask;

/// Specifies the entities and buffers for a parabyzantine task Data.
///
/// A Parabyzantine task Data is concerned with deriving tasks from agreements and transactions.
pub trait ParabyzantineTaskSpec: Sized {
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

pub trait ParabyzantineTaskData<Spec: ParabyzantineTaskSpec>: Sized {
	/// The buffer for the agreement.
	fn parabyzantine_task_agreement_buffer(&self) -> &Spec::AgreementBuffer;
	/// The draft buffer for the agreement.
	fn parabyzantine_task_agreement_draft_buffer(&self) -> Spec::AgreementDraftBuffer;
	/// The buffer for the transaction.
	fn parabyzantine_task_transaction_buffer(&self) -> &Spec::TransactionBuffer;
	/// The draft buffer for the transaction.
	fn parabyzantine_task_transaction_draft_buffer(&self) -> Spec::TransactionDraftBuffer;
	/// The buffer for the task.
	fn parabyzantine_task_task_buffer(&self) -> &Spec::TaskBuffer;
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
}

/// The world of the task step of a parabyzantine task Data.
pub struct TaskWorld<'a, Spec: ParabyzantineTaskSpec> {
	pub agreement_facts: Facts<'a, Spec::AgreementEntity, Spec::AgreementBuffer>,
	pub agreement_inferences:
		Inferences<Spec::AgreementEntity, Spec::AgreementBuffer, Spec::AgreementDraftBuffer>,
	pub transaction_facts: Facts<'a, Spec::TransactionEntity, Spec::TransactionBuffer>,
	pub transaction_inferences:
		Inferences<Spec::TransactionEntity, Spec::TransactionBuffer, Spec::TransactionDraftBuffer>,
	pub task_facts: Facts<'a, Spec::TaskEntity, Spec::TaskBuffer>,
	pub task_inferences: Inferences<Spec::TaskEntity, Spec::TaskBuffer, Spec::TaskDraftBuffer>,
}

pub trait ParabyzantineTask<Spec: ParabyzantineTaskSpec>: Sized {
	/// Compute the parabyzantine task.
	fn compute_parabyzantine_task(&mut self, data: &mut TaskWorld<Spec>);
}
