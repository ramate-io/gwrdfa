pub mod data;
pub mod execution;
pub mod task_subcommittee;

pub use data::ResampleTaskData;
use execution::ResampleTasker;
use parabyzantine::task::{ParabyzantineTask, ParabyzantineTaskData, TaskWorld};
use parabyzantine::NoOp;
use parabyzantine::NoOpData;
pub use task_subcommittee::{IndexTaskSubcommitteeAgreement, TaskSubcommittee};

pub trait ResampleTaskBinding: Sized {
	type ParabyzantineTaskData: ParabyzantineTaskData;
	type ResampleTaskData: ResampleTaskData<Self::ParabyzantineTaskData>;
}

/// [ResampleTask] wraps around the ResampleTask data indicated by the binding.
pub struct ResampleTask<Binding: ResampleTaskBinding>(pub Binding::ResampleTaskData);

impl<Binding: ResampleTaskBinding> ResampleTask<Binding> {
	pub fn data(&self) -> &Binding::ResampleTaskData {
		&self.0
	}

	pub fn data_mut(&mut self) -> &mut Binding::ResampleTaskData {
		&mut self.0
	}
}

impl<Binding: ResampleTaskBinding> ParabyzantineTask<Binding::ParabyzantineTaskData>
	for ResampleTask<Binding>
{
	fn update_parabyzantine_task(&mut self, data: &mut TaskWorld<Binding::ParabyzantineTaskData>) {
		let index_task_subcommittee_agreement_query_plan =
			self.data().index_task_subcommittee_agreement_query_plan();
		for index_data in data.agreement_facts.query(index_task_subcommittee_agreement_query_plan) {
			let index: <Binding::ResampleTaskData as ResampleTaskData<
				Binding::ParabyzantineTaskData,
			>>::IndexTaskSubcommitteeAgreement = (index_data).into();

			// If the task is assigned to this replica, compute the resample task.
			if self.data().is_task_assigned_to_me(&index.subcommittee()) {
				let resample_tasker = self.data_mut().resample_tasker_mut();
				resample_tasker.compute_resample_task(
					&index,
					&data.agreement_facts,
					&data.transaction_facts,
					&mut data.transaction_inferences,
					&data.task_facts,
					&mut data.task_inferences,
				);
			}
		}
	}
}

impl<Binding: ResampleTaskBinding> ResampleTask<Binding> {
	pub fn resample_task(&mut self, task_data: &Binding::ParabyzantineTaskData) {
		let mut task_world = task_data.parabyzantine_task_world();
		self.update_parabyzantine_task(&mut task_world);
	}
}

impl ResampleTaskBinding for NoOp {
	type ParabyzantineTaskData = NoOpData;
	type ResampleTaskData = NoOpData;
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::agreement::ResampleAgreement;
	use parabyzantine::{task::Task, NoOp, NoOpData, Parabyzantine};

	#[test]
	fn test_noop_resample_task_noops() {
		let resample_task = ResampleTask::<NoOp>(NoOpData::new());
		let mut parabyzantine = Parabyzantine::new(
			NoOpData::new(),
			ResampleAgreement::<NoOp>(NoOpData::new()),
			resample_task,
		);
		parabyzantine.update_task(Task);
	}
}
