pub mod data;
pub mod execution;
pub mod spec;
pub mod task_subcommittee;

pub use data::ResampleTaskData;
use execution::ResampleTasker;
use parabyzantine::task::{
	ParabyzantineTask, ParabyzantineTaskData, ParabyzantineTaskDataBinding, TaskWorld,
};
use parabyzantine::NoOp;
use parabyzantine::NoOpData;
pub use spec::ResampleTaskSpec;
pub use task_subcommittee::{IndexTaskSubcommitteeAgreement, TaskSubcommittee};

pub trait ResampleTaskBinding: Sized {
	type ParabyzantineTaskDataBinding: ParabyzantineTaskDataBinding;
	type ResampleTaskSpec: ResampleTaskSpec<Self::ParabyzantineTaskDataBinding>;
	type ResampleTaskData: ResampleTaskData<
		Self::ParabyzantineTaskDataBinding,
		Self::ResampleTaskSpec,
	>;
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

impl<Binding: ResampleTaskBinding>
	ResampleTaskData<Binding::ParabyzantineTaskDataBinding, Binding::ResampleTaskSpec>
	for ResampleTask<Binding>
{
	fn me(
		&self,
	) -> &<Binding::ResampleTaskSpec as ResampleTaskSpec<Binding::ParabyzantineTaskDataBinding>>::Sender{
		self.data().me()
	}

	fn index_task_subcommittee_agreement_query_plan(
		&self,
	) -> <Binding::ResampleTaskSpec as ResampleTaskSpec<Binding::ParabyzantineTaskDataBinding>>::IndexTaskSubcommitteeAgreementQueryPlan{
		self.data().index_task_subcommittee_agreement_query_plan()
	}

	fn resample_tasker_mut(&mut self) -> &mut <Binding::ResampleTaskSpec as ResampleTaskSpec<Binding::ParabyzantineTaskDataBinding>>::ResampleTasker{
		self.data_mut().resample_tasker_mut()
	}
}

impl<Binding: ResampleTaskBinding> ParabyzantineTask for ResampleTask<Binding> {
	type Binding = Binding::ParabyzantineTaskDataBinding;

	fn update_parabyzantine_task(
		&mut self,
		data: &mut TaskWorld<
			<Binding::ParabyzantineTaskDataBinding as ParabyzantineTaskDataBinding>::Spec,
		>,
	) {
		let index_task_subcommittee_agreement_query_plan =
			self.index_task_subcommittee_agreement_query_plan();
		for index_data in data.agreement_facts.query(index_task_subcommittee_agreement_query_plan) {
			let index: <Binding::ResampleTaskSpec as ResampleTaskSpec<
				Binding::ParabyzantineTaskDataBinding,
			>>::IndexTaskSubcommitteeAgreement = (index_data).into();

			// If the task is assigned to this replica, compute the resample task.
			if self.is_task_assigned_to_me(&index.subcommittee()) {
				let resample_tasker = self.resample_tasker_mut();
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
	pub fn resample_task(
		&mut self,
		task_data: &<Binding::ParabyzantineTaskDataBinding as ParabyzantineTaskDataBinding>::Data,
	) {
		let mut task_world = task_data.parabyzantine_task_world();
		self.update_parabyzantine_task(&mut task_world);
	}
}

impl ResampleTaskBinding for NoOp {
	type ParabyzantineTaskDataBinding = NoOp;
	type ResampleTaskSpec = NoOp;
	type ResampleTaskData = NoOpData;
}

#[cfg(test)]
mod tests {
	use super::*;
	use parabyzantine::{
		agreement::Agreement, task::Task, AgreementAction, AgreementHandler, DataBinding, NoOp,
		NoOpData, Parabyzantine, Spec, TaskAction, TaskHandler,
	};

	#[test]
	fn test_noop_resample_task_noops() {
		let resample_task = ResampleTask::<NoOp>(NoOpData::new());
		let mut parabyzantine: Parabyzantine<
			Spec<(
				DataBinding<NoOp>,
				AgreementAction<Agreement>,
				AgreementHandler<NoOp>,
				TaskAction<Task>,
				TaskHandler<ResampleTask<NoOp>>,
			)>,
		> = Parabyzantine {
			data: NoOpData::new(),
			agreement_handler: NoOp,
			task_handler: resample_task,
		};
		parabyzantine.update_task(Task);
	}
}
