use crate::hart::{ParabyzantineData, ParabyzantineDataSpec, ParabyzantineWorld};
use crate::task::{ParabyzantineTaskData, ParabyzantineTaskDataSpec, TaskWorld};

/// Blanket implementation for the task spec.
///
/// Downcasting the world to a task world.
impl<Spec: ParabyzantineDataSpec> ParabyzantineTaskDataSpec for Spec {
	type AgreementEntity = Spec::AgreementEntity;
	type AgreementBuffer = Spec::AgreementBuffer;

	type TransactionEntity = Spec::TransactionEntity;
	type TransactionBuffer = Spec::TransactionBuffer;

	type TaskEntity = Spec::TaskEntity;
	type TaskBuffer = Spec::TaskBuffer;
}

/// Blanket implementation for the task data.
impl<Spec: ParabyzantineDataSpec, Data: ParabyzantineData<Spec>> ParabyzantineTaskData<Spec>
	for Data
where
	Spec: 'static,
{
	fn parabyzantine_task_world<'a>(&'a mut self) -> TaskWorld<'a, Spec> {
		let ParabyzantineWorld { agreement_facts, transaction_facts, task_facts, .. } =
			self.parabyzantine_world();

		TaskWorld { agreement_facts, transaction_facts, task_facts }
	}
}
