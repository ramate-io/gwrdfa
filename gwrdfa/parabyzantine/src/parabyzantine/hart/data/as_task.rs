use crate::hart::ParabyzantineData;
use crate::task::ParabyzantineTaskData;

/// Blanket implementation for the task data.
impl<Data: ParabyzantineData> ParabyzantineTaskData for Data {
	type AgreementEntity = Data::AgreementEntity;
	type AgreementBuffer = Data::AgreementBuffer;
	type AgreementDraftBuffer = Data::AgreementDraftBuffer;
	type TransactionEntity = Data::TransactionEntity;
	type TransactionBuffer = Data::TransactionBuffer;
	type TransactionDraftBuffer = Data::TransactionDraftBuffer;
	type TaskEntity = Data::TaskEntity;
	type TaskBuffer = Data::TaskBuffer;
	type TaskDraftBuffer = Data::TaskDraftBuffer;

	/// The buffer for the agreement.
	fn parabyzantine_task_agreement_buffer(&self) -> &Data::AgreementBuffer {
		self.parabyzantine_agreement_buffer()
	}

	/// The draft buffer for the agreement.
	fn parabyzantine_task_agreement_draft_buffer(&self) -> Data::AgreementDraftBuffer {
		self.parabyzantine_agreement_draft_buffer()
	}

	/// The draft buffer for the agreement.
	fn parabyzantine_task_agreement_buffer_mut(&mut self) -> &mut Data::AgreementBuffer {
		self.parabyzantine_agreement_buffer_mut()
	}

	/// The buffer for the transaction.
	fn parabyzantine_task_transaction_buffer(&self) -> &Data::TransactionBuffer {
		self.parabyzantine_transaction_buffer()
	}

	/// The draft buffer for the transaction.
	fn parabyzantine_task_transaction_draft_buffer(&self) -> Data::TransactionDraftBuffer {
		self.parabyzantine_transaction_draft_buffer()
	}

	/// The draft buffer for the transaction.
	fn parabyzantine_task_transaction_buffer_mut(&mut self) -> &mut Data::TransactionBuffer {
		self.parabyzantine_transaction_buffer_mut()
	}

	/// The buffer for the task.
	fn parabyzantine_task_task_buffer(&self) -> &Data::TaskBuffer {
		self.parabyzantine_task_buffer()
	}

	/// The draft buffer for the task.
	fn parabyzantine_task_task_buffer_mut(&mut self) -> &mut Data::TaskBuffer {
		self.parabyzantine_task_buffer_mut()
	}

	/// The draft buffer for the task.
	fn parabyzantine_task_task_draft_buffer(&self) -> Data::TaskDraftBuffer {
		self.parabyzantine_task_draft_buffer()
	}
}
