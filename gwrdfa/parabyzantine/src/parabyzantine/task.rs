use crate::buffer::{facts::Facts, inferences::Inferences, Bufferlike, DraftBufferlike};

/// Specifies the entities and buffers for a parabyzantine task system.
///
/// A Parabyzantine task system is concerned with deriving tasks from agreements and transactions.
pub trait ParabyzantineTaskSpec {
	type AgreementEntity: Sized;
	type AgreementBuffer: Bufferlike<Self::AgreementEntity>;
	type AgreementDraftBuffer: DraftBufferlike<Self::AgreementEntity, Self::AgreementBuffer>;
	type TransactionEntity: Sized;
	type TransactionBuffer: Bufferlike<Self::TransactionEntity>;
	type TransactionDraftBuffer: DraftBufferlike<Self::TransactionEntity, Self::TransactionBuffer>;
	type TaskEntity: Sized;
	type TaskBuffer: Bufferlike<Self::TaskEntity>;
	type TaskDraftBuffer: DraftBufferlike<Self::TaskEntity, Self::TaskBuffer>;
}

pub trait ParabyzantineTaskWorld<Spec: ParabyzantineTaskSpec> {
	fn agreement_buffer(&self) -> &Spec::AgreementBuffer;

	fn agreement_facts(&self) -> Facts<Spec::AgreementEntity, Spec::AgreementBuffer>;

	fn agreement_draft_buffer(&self) -> Spec::AgreementDraftBuffer;

	fn agreement_inferences(
		&self,
	) -> Inferences<Spec::AgreementEntity, Spec::AgreementBuffer, Spec::AgreementDraftBuffer>;

	fn transaction_buffer(&self) -> &Spec::TransactionBuffer;

	fn transaction_facts(&self) -> Facts<Spec::TransactionEntity, Spec::TransactionBuffer>;

	fn transaction_draft_buffer(&self) -> Spec::TransactionDraftBuffer;

	fn transaction_inferences(
		&self,
	) -> Inferences<Spec::TransactionEntity, Spec::TransactionBuffer, Spec::TransactionDraftBuffer>;

	fn task_buffer(&self) -> &Spec::TaskBuffer;

	fn task_facts(&self) -> Facts<Spec::TaskEntity, Spec::TaskBuffer>;

	fn task_draft_buffer(&self) -> Spec::TaskDraftBuffer;

	fn task_inferences(
		&self,
	) -> Inferences<Spec::TaskEntity, Spec::TaskBuffer, Spec::TaskDraftBuffer>;
}

/// Parabyzantine agreement systems are concerned with deriving agreements from certificates.
pub trait ParabyzantineTask<Spec: ParabyzantineTaskSpec, TaskWorld: ParabyzantineTaskWorld<Spec>> {
	fn compute_parabyzantine_task(&mut self, task_world: &mut TaskWorld);
}
