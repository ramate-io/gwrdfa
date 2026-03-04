//! Resample task protocol.
//!
//! Task processing consumes agreement facts and decides whether this local sender
//! is responsible for executing follow-up work for a given index.

pub mod data;
pub mod execution;
pub mod task_subcommittee;

pub use data::ResampleTaskData;
use execution::ResampleTasker;
use core::marker::PhantomData;
use parabyzantine::task::{ParabyzantineTask, ParabyzantineTaskData, TaskWorld};
pub use task_subcommittee::{IndexTaskSubcommitteeAgreement, TaskSubcommittee};

/// [ResampleTask] wraps around resample task data for a given parabyzantine task data type.
pub struct ResampleTask<Data: ParabyzantineTaskData, ResampleData: ResampleTaskData<Data>>(
	pub ResampleData,
	PhantomData<Data>,
);

impl<Data: ParabyzantineTaskData, ResampleData: ResampleTaskData<Data>>
	ResampleTask<Data, ResampleData>
{
	/// Creates a task wrapper over concrete resample task data.
	pub fn new(data: ResampleData) -> Self {
		Self(data, PhantomData)
	}

	/// Immutable access to task-specific data and strategy.
	pub fn data(&self) -> &ResampleData {
		&self.0
	}

	/// Mutable access to task-specific data and strategy.
	pub fn data_mut(&mut self) -> &mut ResampleData {
		&mut self.0
	}
}

impl<Data: ParabyzantineTaskData, ResampleData: ResampleTaskData<Data>> ParabyzantineTask<Data>
	for ResampleTask<Data, ResampleData>
{
	fn update_parabyzantine_task(&mut self, data: &mut TaskWorld<Data>) {
		let index_task_subcommittee_agreement_query_plan =
			self.data().index_task_subcommittee_agreement_query_plan();
		for index_data in data.agreement_facts.query(index_task_subcommittee_agreement_query_plan) {
			let index: ResampleData::IndexTaskSubcommitteeAgreement = (index_data).into();

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

impl<Data: ParabyzantineTaskData, ResampleData: ResampleTaskData<Data>> ResampleTask<Data, ResampleData> {
	/// Runs a single resample task update pass over the provided task data.
	pub fn resample_task(&mut self, task_data: &Data) {
		let mut task_world = task_data.parabyzantine_task_world();
		self.update_parabyzantine_task(&mut task_world);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::agreement::ResampleAgreement;
	use parabyzantine::{task::Task, NoOpData, Parabyzantine};

	#[test]
	fn test_noop_resample_task_noops() {
		let resample_task = ResampleTask::new(NoOpData::new());
		let mut parabyzantine = Parabyzantine::new(
			NoOpData::new(),
			ResampleAgreement::new(NoOpData::new()),
			resample_task,
		);
		parabyzantine.update_task(Task);
	}
}
