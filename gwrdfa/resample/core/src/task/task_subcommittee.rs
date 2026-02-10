use crate::agreement::Subcommittee;

/// A [TaskSubcommittee] is a subcommittee that can determine whether a task has been assigned to a given [Sender].
pub trait TaskSubcommittee<Sender: Eq>: Subcommittee<Sender> {
	/// Whether the subcommittee has assigned a task to a given [Sender]
	fn is_task_assigned_to(&self, sender: &Sender) -> bool;
}
