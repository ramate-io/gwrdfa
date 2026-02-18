use super::TaskSubcommittee;
use parabyzantine::task::ParabyzantineTaskBinding;

use super::ResampleTaskSpec;

pub trait ResampleTaskData<Binding: ParabyzantineTaskBinding, Spec: ResampleTaskSpec<Binding>>:
	Sized
{
	/// Gets the sender identifier for the Hart.
	fn me(&self) -> &Spec::Sender;

	/// Computes whether the task is assigned to the sender.
	fn is_task_assigned_to_me(&self, task_subcommittee: &Spec::TaskSubcommittee) -> bool {
		task_subcommittee.is_task_assigned_to(self.me())
	}
}
