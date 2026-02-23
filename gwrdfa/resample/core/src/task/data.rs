use super::TaskSubcommittee;
use parabyzantine::buffer::query::IntoQuery;
use parabyzantine::task::{ParabyzantineTaskDataBinding, ParabyzantineTaskDataSpec};
use parabyzantine::{NoOp, NoOpData};

use super::ResampleTaskSpec;

pub trait ResampleTaskData<Binding: ParabyzantineTaskDataBinding, Spec: ResampleTaskSpec<Binding>>:
	Sized
where
	for<'a> &'a <Binding::Spec as ParabyzantineTaskDataSpec>::AgreementBuffer: IntoQuery<
		<Binding::Spec as ParabyzantineTaskDataSpec>::AgreementEntity,
		Spec::IndexTaskSubcommitteeAgreementQueryPlan,
		Query = Spec::IndexTaskSubcommitteeAgreementQuery<'a>,
	>,
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
