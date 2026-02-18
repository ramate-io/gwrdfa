use super::{IndexTaskSubcommitteeAgreement, ResampleTasker, TaskSubcommittee};
use parabyzantine::{
	buffer::{QueryPlanlike, Querylike},
	task::ParabyzantineTaskDataBinding,
	task::ParabyzantineTaskDataSpec,
	NoOp,
};

pub trait ResampleTaskSpec<Binding: ParabyzantineTaskDataBinding>: Sized {
	/// The type of the index.
	type Index: Eq;

	/// The type of the sender of a task.
	type Sender: Eq;

	/// The type of the task subcommittee.
	type TaskSubcommittee: TaskSubcommittee<Self::Sender>;

	/// The type queried for indicating agreement on an index.
	type IndexTaskSubcommitteeAgreementQueryData;

	/// The query for the index subcommittee agreement.
	type IndexTaskSubcommitteeAgreementQuery: Querylike<
		<Binding::Spec as ParabyzantineTaskDataSpec>::AgreementEntity,
		<Binding::Spec as ParabyzantineTaskDataSpec>::AgreementBuffer,
		Self::IndexTaskSubcommitteeAgreementQueryData,
	>;

	/// The query plan for the index subcommittee agreement.
	type IndexTaskSubcommitteeAgreementQueryPlan: for<'a> QueryPlanlike<
		'a,
		<Binding::Spec as ParabyzantineTaskDataSpec>::AgreementEntity,
		<Binding::Spec as ParabyzantineTaskDataSpec>::AgreementBuffer,
		Self::IndexTaskSubcommitteeAgreementQueryData,
		Self::IndexTaskSubcommitteeAgreementQuery,
	>;

	/// The type of the index subcommittee agreement.
	type IndexTaskSubcommitteeAgreement: IndexTaskSubcommitteeAgreement<Self::Index, Self::Sender, Self::TaskSubcommittee>
		+ for<'a> From<&'a (
			<Binding::Spec as ParabyzantineTaskDataSpec>::AgreementEntity,
			Self::IndexTaskSubcommitteeAgreementQueryData,
		)>;

	/// The tasker
	type ResampleTasker: ResampleTasker<
		Self::Index,
		Self::Sender,
		Self::TaskSubcommittee,
		Self::IndexTaskSubcommitteeAgreement,
		Binding,
	>;
}

impl ResampleTaskSpec<NoOp> for NoOp {
	type Index = NoOp;
	type Sender = NoOp;
	type TaskSubcommittee = NoOp;
	type IndexTaskSubcommitteeAgreementQueryData = NoOp;
	type IndexTaskSubcommitteeAgreementQuery = NoOp;
	type IndexTaskSubcommitteeAgreementQueryPlan = NoOp;
	type IndexTaskSubcommitteeAgreement = NoOp;
	type ResampleTasker = NoOp;
}
