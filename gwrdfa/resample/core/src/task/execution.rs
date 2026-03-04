//! Execution hooks for resample task processing.

use super::{IndexTaskSubcommitteeAgreement, TaskSubcommittee};
use parabyzantine::{
	buffer::{Facts, Inferences},
	task::ParabyzantineTaskData,
	NoOp,
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
	Value: Eq + 'static,
	Sub: TaskSubcommittee<Value, Sender>,
	SubAg: IndexTaskSubcommitteeAgreement<Index, Value, Sender, Sub>,
	Data: ParabyzantineTaskData,
>: Sized
{
	/// Computes task-side inferences for one agreed `(index, subcommittee)` unit.
	fn compute_resample_task(
		&mut self,
		index: &SubAg,
		agreement_facts: &Facts<
			Data::AgreementEntity,
			Data::AgreementBuffer,
		>,
		transaction_facts: &Facts<
			Data::TransactionEntity,
			Data::TransactionBuffer,
		>,
		transaction_inferences: &mut Inferences<
			Data::TransactionEntity,
			Data::TransactionBuffer,
			Data::TransactionDraftBuffer,
		>,
		task_facts: &Facts<
			Data::TaskEntity,
			Data::TaskBuffer,
		>,
		task_inferences: &mut Inferences<
			Data::TaskEntity,
			Data::TaskBuffer,
			Data::TaskDraftBuffer,
		>,
	);
}

impl<
		Index: Eq,
		Sender: Eq,
		Value: Eq + 'static,
		Sub: TaskSubcommittee<Value, Sender>,
		SubAg: IndexTaskSubcommitteeAgreement<Index, Value, Sender, Sub>,
		Data: ParabyzantineTaskData,
	> ResampleTasker<Index, Sender, Value, Sub, SubAg, Data> for NoOp
{
	fn compute_resample_task(
		&mut self,
		_index: &SubAg,
		_agreement_facts: &Facts<
			Data::AgreementEntity,
			Data::AgreementBuffer,
		>,
		_transaction_facts: &Facts<
			Data::TransactionEntity,
			Data::TransactionBuffer,
		>,
		_transaction_inferences: &mut Inferences<
			Data::TransactionEntity,
			Data::TransactionBuffer,
			Data::TransactionDraftBuffer,
		>,
		_task_facts: &Facts<
			Data::TaskEntity,
			Data::TaskBuffer,
		>,
		_task_inferences: &mut Inferences<
			Data::TaskEntity,
			Data::TaskBuffer,
			Data::TaskDraftBuffer,
		>,
	) {
	}
}
