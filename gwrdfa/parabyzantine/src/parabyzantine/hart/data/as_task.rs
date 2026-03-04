use crate::hart::{ParabyzantineData, ParabyzantineDataSpec};
use crate::task::ParabyzantineTaskData;

/// Blanket implementation for the task spec.
///
/// Downcasting the world to a task world.
impl<Spec: ParabyzantineDataSpec> ParabyzantineTaskDataSpec for Spec {
	type AgreementEntity = Spec::AgreementEntity;
	type AgreementBuffer = Spec::AgreementBuffer;
	type AgreementDraftBuffer = Spec::AgreementDraftBuffer;
	type TransactionEntity = Spec::TransactionEntity;
	type TransactionBuffer = Spec::TransactionBuffer;
	type TransactionDraftBuffer = Spec::TransactionDraftBuffer;
	type TaskEntity = Spec::TaskEntity;
	type TaskBuffer = Spec::TaskBuffer;
	type TaskDraftBuffer = Spec::TaskDraftBuffer;
}

/// Blanket implementation for the task data.
impl<Spec: ParabyzantineDataSpec, Data: ParabyzantineData<Spec>> ParabyzantineTaskData<Spec>
	for Data
{
	/// The buffer for the agreement.
	fn parabyzantine_task_agreement_buffer(&self) -> &Spec::AgreementBuffer {
		self.parabyzantine_agreement_buffer()
	}

	/// The draft buffer for the agreement.
	fn parabyzantine_task_agreement_draft_buffer(&self) -> Spec::AgreementDraftBuffer {
		self.parabyzantine_agreement_draft_buffer()
	}

	/// The draft buffer for the agreement.
	fn parabyzantine_task_agreement_buffer_mut(&mut self) -> &mut Spec::AgreementBuffer {
		self.parabyzantine_agreement_buffer_mut()
	}

	/// The buffer for the transaction.
	fn parabyzantine_task_transaction_buffer(&self) -> &Spec::TransactionBuffer {
		self.parabyzantine_transaction_buffer()
	}

	/// The draft buffer for the transaction.
	fn parabyzantine_task_transaction_draft_buffer(&self) -> Spec::TransactionDraftBuffer {
		self.parabyzantine_transaction_draft_buffer()
	}

	/// The draft buffer for the transaction.
	fn parabyzantine_task_transaction_buffer_mut(&mut self) -> &mut Spec::TransactionBuffer {
		self.parabyzantine_transaction_buffer_mut()
	}

	/// The buffer for the task.
	fn parabyzantine_task_task_buffer(&self) -> &Spec::TaskBuffer {
		self.parabyzantine_task_buffer()
	}

	/// The draft buffer for the task.
	fn parabyzantine_task_task_buffer_mut(&mut self) -> &mut Spec::TaskBuffer {
		self.parabyzantine_task_buffer_mut()
	}

	/// The draft buffer for the task.
	fn parabyzantine_task_task_draft_buffer(&self) -> Spec::TaskDraftBuffer {
		self.parabyzantine_task_draft_buffer()
	}
}
