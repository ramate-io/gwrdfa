use crate::agreement::Subcommittee;
use parabyzantine::NoOp;

/// A [TaskSubcommittee] is a subcommittee that can determine whether a task has been assigned to a given [Sender].
pub trait TaskSubcommittee<Sender: Eq>: Subcommittee<Sender> {
	/// Whether the subcommittee has assigned a task to a given [Sender]
	fn is_task_assigned_to(&self, sender: &Sender) -> bool;
}

/// A [TaskSubcommittee] for the [NoOp] struct.
impl TaskSubcommittee<NoOp> for NoOp {
	fn is_task_assigned_to(&self, _sender: &NoOp) -> bool {
		false
	}
}

pub trait IndexTaskSubcommitteeAgreement<Index: Eq, Sender: Eq, Sub: TaskSubcommittee<Sender>>:
	Eq
{
	/// The index of the agreement.
	fn index(&self) -> Index;

	/// The subcommittee of the agreement.
	fn subcommittee(&self) -> Sub;
}

/// A [IndexSubcommitteeAgreement] for the [NoOp] struct.
impl IndexTaskSubcommitteeAgreement<NoOp, NoOp, NoOp> for NoOp {
	fn index(&self) -> NoOp {
		NoOp
	}
	fn subcommittee(&self) -> NoOp {
		NoOp
	}
}
