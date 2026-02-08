use crate::parabyzantine::system::{ParabyzantineSpec, ParabyzantineSystem};
use crate::parabyzantine::task::{ParabyzantineTaskSpec, ParabyzantineTaskSystem};

/// Blanket implementation for the task spec.
///
/// Downcasting the world to a task world.
impl<Spec: ParabyzantineSpec<System>, System: ParabyzantineSystem<Spec>>
	ParabyzantineTaskSpec<System> for Spec
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

/// Blanket implementation for the task system.
impl<Spec: ParabyzantineSpec<System>, System: ParabyzantineSystem<Spec>>
	ParabyzantineTaskSystem<Spec> for System
{
}
