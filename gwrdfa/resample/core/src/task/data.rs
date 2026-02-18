use super::TaskSubcommittee;
use parabyzantine::task::ParabyzantineTaskDataBinding;
use parabyzantine::{NoOp, NoOpData};

use super::ResampleTaskSpec;

pub trait ResampleTaskData<Binding: ParabyzantineTaskDataBinding, Spec: ResampleTaskSpec<Binding>>:
	Sized
{
	/// Gets the sender identifier for the Hart.
	fn me(&self) -> &Spec::Sender;

	/// Computes whether the task is assigned to the sender.
	fn is_task_assigned_to_me(&self, task_subcommittee: &Spec::TaskSubcommittee) -> bool {
		task_subcommittee.is_task_assigned_to(self.me())
	}

	/// Gets the query for the index subcommittee agreement.
	fn index_task_subcommittee_agreement_query_plan(
		&self,
	) -> Spec::IndexTaskSubcommitteeAgreementQueryPlan;

	/// Gets the tasker
	fn resample_tasker_mut(&mut self) -> &mut Spec::ResampleTasker;
}

impl ResampleTaskData<NoOp, NoOp> for NoOpData {
	fn me(&self) -> &NoOp {
		&self.no_op
	}
	fn is_task_assigned_to_me(&self, _task_subcommittee: &NoOp) -> bool {
		false
	}
	fn index_task_subcommittee_agreement_query_plan(&self) -> NoOp {
		NoOp
	}

	fn resample_tasker_mut(&mut self) -> &mut NoOp {
		&mut self.no_op
	}
}
