use crate::task::AegeriTaskError;
use aegeri_message::{Index, Message, Proposal, Transaction};
use gwrdfa_container::{Component, ContainerGiving};

/// Container for tasks.
/// All tasks are currently are just proposals which have been processed.
///
/// We insert a proposal when we want it to be signed and broadcasted.
#[derive(Debug, Default)]
pub struct TaskContainer {
	pub index: Component<Index>,
	pub proposal: Component<Proposal>,
	pub transaction: Component<Message<Transaction>>,
	pub task_error: Component<AegeriTaskError>,
}

impl ContainerGiving<Proposal> for TaskContainer {
	fn as_component(&self) -> Component<&Proposal> {
		self.proposal.as_ref()
	}
}

impl ContainerGiving<Index> for TaskContainer {
	fn as_component(&self) -> Component<&Index> {
		self.index.as_ref()
	}
}

impl ContainerGiving<Message<Transaction>> for TaskContainer {
	fn as_component(&self) -> Component<&Message<Transaction>> {
		self.transaction.as_ref()
	}
}
