use crate::agreement::Subcommittee;
use parabyzantine::NoOp;

/// A [TaskSubcommittee] is a subcommittee that can determine whether a task has been assigned to a given [Sender].
pub trait TaskSubcommittee<Value: Eq + 'static, Sender: Eq>: Subcommittee<Value> {
	/// Whether the subcommittee has assigned a task to a given [Sender]
	fn is_task_assigned_to(&self, sender: &Sender) -> bool;
}

/// A [TaskSubcommittee] for the [NoOp] struct.
impl<T: Eq + 'static> TaskSubcommittee<T, NoOp> for NoOp {
	fn is_task_assigned_to(&self, _sender: &NoOp) -> bool {
		false
	}
}

pub trait IndexTaskSubcommitteeAgreement<
	Index: Eq,
	Value: Eq + 'static,
	Sender: Eq,
	Sub: TaskSubcommittee<Value, Sender>,
>: Eq
{
	/// The index of the agreement.
	fn index(&self) -> Index;

	/// The subcommittee of the agreement.
	fn subcommittee(&self) -> Sub;
}

/// A [IndexSubcommitteeAgreement] for the [NoOp] struct.
impl<T: Eq + 'static> IndexTaskSubcommitteeAgreement<NoOp, T, NoOp, NoOp> for NoOp {
	fn index(&self) -> NoOp {
		NoOp
	}
	fn subcommittee(&self) -> NoOp {
		NoOp
	}
}
