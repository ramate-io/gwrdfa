use super::{execution::ResampleTasker, IndexTaskSubcommitteeAgreement, TaskSubcommittee};
use parabyzantine::buffer::query::{QueryPlanlike, Querylike};
use parabyzantine::task::ParabyzantineTaskData;
use parabyzantine::{NoOp, NoOpData};

pub trait ResampleTaskData<Data: ParabyzantineTaskData>: Sized {
	/// The type of the index.
	type Index: Eq;
	/// The type of the sender of a task.
	type Sender: Eq;
	/// The type of the value of a task.
	type Value: Eq + 'static;
	/// The type of the task subcommittee.
	type TaskSubcommittee: TaskSubcommittee<Self::Value, Self::Sender>;
	/// The type queried for indicating agreement on an index.
	type IndexTaskSubcommitteeAgreementQueryData<'a>;
	/// The query for the index subcommittee agreement.
	type IndexTaskSubcommitteeAgreementQuery<'a>: Querylike<
		Data::AgreementEntity,
		Self::IndexTaskSubcommitteeAgreementQueryData<'a>,
	>;
	/// The query plan for the index subcommittee agreement.
	type IndexTaskSubcommitteeAgreementQueryPlan: for<'a> QueryPlanlike<
		Data::AgreementEntity,
		&'a Data::AgreementBuffer,
		Self::IndexTaskSubcommitteeAgreementQueryData<'a>,
		Self::IndexTaskSubcommitteeAgreementQuery<'a>,
	>;
	/// The type of the index subcommittee agreement.
	type IndexTaskSubcommitteeAgreement: IndexTaskSubcommitteeAgreement<
			Self::Index,
			Self::Value,
			Self::Sender,
			Self::TaskSubcommittee,
		> + for<'a> From<(
			Data::AgreementEntity,
			Self::IndexTaskSubcommitteeAgreementQueryData<'a>,
		)>;
	/// The tasker.
	type ResampleTasker: ResampleTasker<
		Self::Index,
		Self::Sender,
		Self::Value,
		Self::TaskSubcommittee,
		Self::IndexTaskSubcommitteeAgreement,
		Data,
	>;

	/// Gets the sender identifier for the Hart.
	fn me(&self) -> &Self::Sender;

	/// Computes whether the task is assigned to the sender.
	fn is_task_assigned_to_me(&self, task_subcommittee: &Self::TaskSubcommittee) -> bool {
		task_subcommittee.is_task_assigned_to(self.me())
	}

	/// Gets the query for the index subcommittee agreement.
	fn index_task_subcommittee_agreement_query_plan(
		&self,
	) -> Self::IndexTaskSubcommitteeAgreementQueryPlan;

	/// Gets the tasker
	fn resample_tasker_mut(&mut self) -> &mut Self::ResampleTasker;
}

impl ResampleTaskData<NoOpData> for NoOpData {
	type Index = NoOp;
	type Sender = NoOp;
	type Value = NoOp;
	type TaskSubcommittee = NoOp;
	type IndexTaskSubcommitteeAgreementQueryData<'a> = NoOp;
	type IndexTaskSubcommitteeAgreementQuery<'a> = NoOp;
	type IndexTaskSubcommitteeAgreementQueryPlan = NoOp;
	type IndexTaskSubcommitteeAgreement = NoOp;
	type ResampleTasker = NoOp;

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
