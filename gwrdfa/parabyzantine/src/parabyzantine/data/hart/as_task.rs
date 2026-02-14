use crate::data::hart::{ParabyzantineData, ParabyzantineSpec};
use crate::data::task::{ParabyzantineTaskData, ParabyzantineTaskSpec};

/// Blanket implementation for the task spec.
///
/// Downcasting the world to a task world.
impl<Spec: ParabyzantineSpec> ParabyzantineTaskSpec for Spec {
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
impl<Spec: ParabyzantineSpec, Data: ParabyzantineData<Spec>> ParabyzantineTaskData<Spec> for Data {
	fn parabyzantine_task_agreement_buffer(&self) -> &Spec::AgreementBuffer {
		self.parabyzantine_agreement_buffer()
	}
	fn parabyzantine_task_agreement_draft_buffer(&self) -> Spec::AgreementDraftBuffer {
		self.parabyzantine_agreement_draft_buffer()
	}
	fn parabyzantine_task_transaction_buffer(&self) -> &Spec::TransactionBuffer {
		self.parabyzantine_transaction_buffer()
	}
	fn parabyzantine_task_transaction_draft_buffer(&self) -> Spec::TransactionDraftBuffer {
		self.parabyzantine_transaction_draft_buffer()
	}
	fn parabyzantine_task_task_buffer(&self) -> &Spec::TaskBuffer {
		self.parabyzantine_task_buffer()
	}
	fn parabyzantine_task_task_draft_buffer(&self) -> Spec::TaskDraftBuffer {
		self.parabyzantine_task_draft_buffer()
	}
}
