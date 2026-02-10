use super::TaskSubcommittee;
use parabyzantine::task::ParabyzantineTaskBinding;

pub trait ResampleTaskSpec<Binding: ParabyzantineTaskBinding>: Sized {
	/// The type of the sender of a task.
	type Sender: Eq;

	/// The type of the task subcommittee.
	type TaskSubcommittee: TaskSubcommittee<Self::Sender>;
}
