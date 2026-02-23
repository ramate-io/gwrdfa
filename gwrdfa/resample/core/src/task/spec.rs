use super::{IndexTaskSubcommitteeAgreement, ResampleTasker, TaskSubcommittee};
use parabyzantine::{
	buffer::query::{IntoQuery, Querylike},
	task::ParabyzantineTaskDataBinding,
	task::ParabyzantineTaskDataSpec,
	NoOp,
};

pub trait ResampleTaskSpec<Binding: ParabyzantineTaskDataBinding>: Sized
where
	for<'a> &'a <Binding::Spec as ParabyzantineTaskDataSpec>::AgreementBuffer: IntoQuery<
		<Binding::Spec as ParabyzantineTaskDataSpec>::AgreementEntity,
		Self::IndexTaskSubcommitteeAgreementQueryPlan,
		Query = Self::IndexTaskSubcommitteeAgreementQuery<'a>,
	>,
{
	/// The type of the index.
	type Index: Eq;

	/// The type of the sender of a task.
	type Sender: Eq;

	/// The type of the task subcommittee.
	type TaskSubcommittee: TaskSubcommittee<Self::Sender>;

	/// The type queried for indicating agreement on an index.
	type IndexTaskSubcommitteeAgreementQueryData<'a>;

	/// The query for the index subcommittee agreement.
	type IndexTaskSubcommitteeAgreementQuery<'a>: Querylike<
		<Binding::Spec as ParabyzantineTaskDataSpec>::AgreementEntity,
		Item = Self::IndexTaskSubcommitteeAgreementQueryData<'a>,
	>;

	/// The query plan for the index subcommittee agreement.
	type IndexTaskSubcommitteeAgreementQueryPlan;

	/// The type of the index subcommittee agreement.
	type IndexTaskSubcommitteeAgreement: IndexTaskSubcommitteeAgreement<Self::Index, Self::Sender, Self::TaskSubcommittee>
		+ for<'a> From<(
			<Binding::Spec as ParabyzantineTaskDataSpec>::AgreementEntity,
			Self::IndexTaskSubcommitteeAgreementQueryData<'a>,
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
	type IndexTaskSubcommitteeAgreementQueryData<'a> = NoOp;
	type IndexTaskSubcommitteeAgreementQuery<'a> = NoOp;
	type IndexTaskSubcommitteeAgreementQueryPlan = NoOp;
	type IndexTaskSubcommitteeAgreement = NoOp;
	type ResampleTasker = NoOp;
}
