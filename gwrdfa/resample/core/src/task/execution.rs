use super::{IndexTaskSubcommitteeAgreement, TaskSubcommittee};
use parabyzantine::{
	buffer::{Facts, Inferences},
	task::{ParabyzantineTaskBinding, ParabyzantineTaskDataSpec},
};

/// A [ResampleTasker] is a trait that can compute a resample task.
///
/// Generally, there are two semantic patterns of tasks:
/// 1. The task is executed prior to being populated in the buffer.
/// This effectively means that tasks stand for the result of the execution.
/// 2. The task is executed after being populated in the buffer.
/// This effectively means that tasks stand for the scheduling of the execution.
pub trait ResampleTasker<
	Index: Eq,
	Sender: Eq,
	Sub: TaskSubcommittee<Sender>,
	SubAg: IndexTaskSubcommitteeAgreement<Index, Sender, Sub>,
	Binding: ParabyzantineTaskBinding,
>: Sized
{
	fn compute_resample_task(
		&mut self,
		index: &SubAg,
		agreement_facts: &Facts<
			<Binding::Spec as ParabyzantineTaskDataSpec>::AgreementEntity,
			<Binding::Spec as ParabyzantineTaskDataSpec>::AgreementBuffer,
		>,
		transaction_facts: &Facts<
			<Binding::Spec as ParabyzantineTaskDataSpec>::TransactionEntity,
			<Binding::Spec as ParabyzantineTaskDataSpec>::TransactionBuffer,
		>,
		transaction_inferences: &mut Inferences<
			<Binding::Spec as ParabyzantineTaskDataSpec>::TransactionEntity,
			<Binding::Spec as ParabyzantineTaskDataSpec>::TransactionBuffer,
			<Binding::Spec as ParabyzantineTaskDataSpec>::TransactionDraftBuffer,
		>,
		task_facts: &Facts<
			<Binding::Spec as ParabyzantineTaskDataSpec>::TaskEntity,
			<Binding::Spec as ParabyzantineTaskDataSpec>::TaskBuffer,
		>,
		task_inferences: &mut Inferences<
			<Binding::Spec as ParabyzantineTaskDataSpec>::TaskEntity,
			<Binding::Spec as ParabyzantineTaskDataSpec>::TaskBuffer,
			<Binding::Spec as ParabyzantineTaskDataSpec>::TaskDraftBuffer,
		>,
	);
}
