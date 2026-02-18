pub mod data;
pub mod execution;
pub mod spec;
pub mod task_subcommittee;

pub use data::ResampleTaskData;
use execution::ResampleTasker;
use parabyzantine::task::{ParabyzantineTask, ParabyzantineTaskBinding, TaskWorld};
pub use spec::ResampleTaskSpec;
pub use task_subcommittee::{IndexTaskSubcommitteeAgreement, TaskSubcommittee};

pub trait ResampleTaskBinding: Sized {
	type ParabyzantineTaskDataBinding: ParabyzantineTaskBinding;
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
	type Spec = <Binding::ParabyzantineTaskDataBinding as ParabyzantineTaskBinding>::Spec;

	fn compute_parabyzantine_task(&mut self, data: &mut TaskWorld<Self::Spec>) {
		let index_task_subcommittee_agreement_query =
			self.index_task_subcommittee_agreement_query_plan();
		for index_bundle in data.agreement_facts.query(index_task_subcommittee_agreement_query) {
			let index: <Binding::ResampleTaskSpec as ResampleTaskSpec<
				Binding::ParabyzantineTaskDataBinding,
			>>::IndexTaskSubcommitteeAgreement = (&index_bundle).into();

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
