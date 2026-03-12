use aegeri_message::Proposal;
use gwrdfa_container::{Component, ContainerGiving};

/// Container for tasks.
/// All tasks are currently are just proposals which have been processed.
///
/// We insert a proposal when we want it to be signed and broadcasted.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TaskContainer {
	pub proposal: Component<Proposal>,
}

impl ContainerGiving<Proposal> for TaskContainer {
	fn as_component(&self) -> Component<&Proposal> {
		self.proposal.as_ref()
	}
}
