use crate::parabyzantine::data::{ParabyzantineData, ParabyzantineSpec};
use crate::parabyzantine::task::ParabyzantineTaskSpec;

/// Blanket implementation for the task spec.
///
/// Downcasting the world to a task world.
impl<Spec: ParabyzantineSpec<Data>, Data: ParabyzantineData<Spec>> ParabyzantineTaskSpec<Data>
	for Spec
{
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
