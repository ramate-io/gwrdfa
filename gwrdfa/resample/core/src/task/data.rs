use parabyzantine::task::ParabyzantineTaskBinding;

use super::ResampleTaskSpec;

pub trait ResampleTaskData<Binding: ParabyzantineTaskBinding, Spec: ResampleTaskSpec<Binding>>:
	Sized
{
}
