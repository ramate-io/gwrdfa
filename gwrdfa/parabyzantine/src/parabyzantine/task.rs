use crate::buffer::{facts::Facts, inferences::Inferences, Bufferlike, DraftBufferlike};
use crate::{Container, Factory, Member, Product, View};

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
pub trait ParabyzantineTaskSpec<Data: ParabyzantineTaskData<Self>>: Sized {
	/// The entity type for the agreement.
	type AgreementEntity: Sized;
	/// The buffer type for the agreement.
	type AgreementBuffer: Bufferlike<Self::AgreementEntity> + Member<Data>;
	/// The draft buffer type for the agreement.
	type AgreementDraftBuffer: DraftBufferlike<Self::AgreementEntity, Self::AgreementBuffer>
		+ Product<Data>;

	/// The entity type for the transaction.
	type TransactionEntity: Sized;
	/// The buffer type for the transaction.
	type TransactionBuffer: Bufferlike<Self::TransactionEntity> + Member<Data>;
	/// The draft buffer type for the transaction.
	type TransactionDraftBuffer: DraftBufferlike<Self::TransactionEntity, Self::TransactionBuffer>
		+ Product<Data>;

	/// The entity type for the task.
	type TaskEntity: Sized;
	/// The buffer type for the task.
	type TaskBuffer: Bufferlike<Self::TaskEntity> + Member<Data>;
	/// The draft buffer type for the task.
	type TaskDraftBuffer: DraftBufferlike<Self::TaskEntity, Self::TaskBuffer> + Product<Data>;
}

pub trait ParabyzantineTaskData<Spec: ParabyzantineTaskSpec<Self>>: Sized {
	fn parabyzantine_task_world(&self) -> TaskWorld<Spec, Self>;
}

/// Blanket implementation for the task Data when members are available.
impl<Spec: ParabyzantineTaskSpec<Data>, Data> ParabyzantineTaskData<Spec> for Data
where
	Data: Sized,
	Spec::AgreementBuffer: Member<Data>,
	Spec::AgreementDraftBuffer: Product<Data>,
	Spec::TransactionBuffer: Member<Data>,
	Spec::TransactionDraftBuffer: Product<Data>,
	Spec::TaskBuffer: Member<Data>,
	Spec::TaskDraftBuffer: Product<Data>,
{
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

/// The world of the task step of a parabyzantine task Data.
pub struct TaskWorld<'a, Spec: ParabyzantineTaskSpec<Data>, Data: ParabyzantineTaskData<Spec>> {
	pub agreement_facts: Facts<'a, Spec::AgreementEntity, Spec::AgreementBuffer>,
	pub agreement_inferences:
		Inferences<Spec::AgreementEntity, Spec::AgreementBuffer, Spec::AgreementDraftBuffer>,
	pub transaction_facts: Facts<'a, Spec::TransactionEntity, Spec::TransactionBuffer>,
	pub transaction_inferences:
		Inferences<Spec::TransactionEntity, Spec::TransactionBuffer, Spec::TransactionDraftBuffer>,
	pub task_facts: Facts<'a, Spec::TaskEntity, Spec::TaskBuffer>,
	pub task_inferences: Inferences<Spec::TaskEntity, Spec::TaskBuffer, Spec::TaskDraftBuffer>,
}

/// View the world of a parabyzantine task Data.
///
/// This is implemented for ergonomics so that the user can write in the same style if they so choose.
impl<'a, Spec: ParabyzantineTaskSpec<Data>, Data: ParabyzantineTaskData<Spec>> View<'a, Data>
	for TaskWorld<'a, Spec, Data>
{
	fn view(from: &'a Data) -> Self {
		from.parabyzantine_task_world()
	}
}

pub trait ParabyzantineTask<Spec: ParabyzantineTaskSpec<Data>, Data: ParabyzantineTaskData<Spec>>:
	Sized
{
	/// Prepare the parabyzantine task.
	fn prepare_parabyzantine_task(&mut self, data: &mut Data);

	/// Compute the parabyzantine task.
	fn compute_parabyzantine_task(&mut self, data: &mut Data);

	/// Commit the parabyzantine task.
	fn commit_parabyzantine_task(&mut self, data: &mut Data);
}
