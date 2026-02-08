use crate::buffer::{facts::Facts, inferences::Inferences, Bufferlike, DraftBufferlike};
use crate::{Container, Factory, Member, Product, View};

/// Specifies the entities and buffers for a parabyzantine task system.
///
/// A Parabyzantine task system is concerned with deriving tasks from agreements and transactions.
pub trait ParabyzantineTaskSpec<System: ParabyzantineTaskSystem<Self>>: Sized {
	/// The entity type for the agreement.
	type AgreementEntity: Sized;
	/// The buffer type for the agreement.
	type AgreementBuffer: Bufferlike<Self::AgreementEntity> + Member<System>;
	/// The draft buffer type for the agreement.
	type AgreementDraftBuffer: DraftBufferlike<Self::AgreementEntity, Self::AgreementBuffer>
		+ Product<System>;

	/// The entity type for the transaction.
	type TransactionEntity: Sized;
	/// The buffer type for the transaction.
	type TransactionBuffer: Bufferlike<Self::TransactionEntity> + Member<System>;
	/// The draft buffer type for the transaction.
	type TransactionDraftBuffer: DraftBufferlike<Self::TransactionEntity, Self::TransactionBuffer>
		+ Product<System>;

	/// The entity type for the task.
	type TaskEntity: Sized;
	/// The buffer type for the task.
	type TaskBuffer: Bufferlike<Self::TaskEntity> + Member<System>;
	/// The draft buffer type for the task.
	type TaskDraftBuffer: DraftBufferlike<Self::TaskEntity, Self::TaskBuffer> + Product<System>;
}

pub trait ParabyzantineTaskSystem<Spec: ParabyzantineTaskSpec<Self>>: Sized {
	fn parabyzantine_task_world(&self) -> TaskWorld<Spec, Self> {
		TaskWorld {
			agreement_facts: self.member::<Spec::AgreementBuffer>().into(),
			agreement_inferences: self.produce::<Spec::AgreementDraftBuffer>().into(),
			transaction_facts: self.member::<Spec::TransactionBuffer>().into(),
			transaction_inferences: self.produce::<Spec::TransactionDraftBuffer>().into(),
			task_facts: self.member::<Spec::TaskBuffer>().into(),
			task_inferences: self.produce::<Spec::TaskDraftBuffer>().into(),
		}
	}
}

/// The world of the task step of a parabyzantine task system.
pub struct TaskWorld<'a, Spec: ParabyzantineTaskSpec<System>, System: ParabyzantineTaskSystem<Spec>>
{
	pub agreement_facts: Facts<'a, Spec::AgreementEntity, Spec::AgreementBuffer>,
	pub agreement_inferences:
		Inferences<Spec::AgreementEntity, Spec::AgreementBuffer, Spec::AgreementDraftBuffer>,
	pub transaction_facts: Facts<'a, Spec::TransactionEntity, Spec::TransactionBuffer>,
	pub transaction_inferences:
		Inferences<Spec::TransactionEntity, Spec::TransactionBuffer, Spec::TransactionDraftBuffer>,
	pub task_facts: Facts<'a, Spec::TaskEntity, Spec::TaskBuffer>,
	pub task_inferences: Inferences<Spec::TaskEntity, Spec::TaskBuffer, Spec::TaskDraftBuffer>,
}

/// View the world of a parabyzantine task system.
///
/// This is implemented for ergonomics so that the user can write in the same style if they so choose.
impl<'a, Spec: ParabyzantineTaskSpec<System>, System: ParabyzantineTaskSystem<Spec>>
	View<'a, System> for TaskWorld<'a, Spec, System>
{
	fn view(from: &'a System) -> Self {
		from.parabyzantine_task_world()
	}
}
