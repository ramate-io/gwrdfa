use aegeri_message::Proposal;
use gwrdfa_container::{Component, ContainerGiving};

/// Canonical message container used by the Gossamer Hart integration.
///
/// The container keeps the message payload plus lifecycle/error markers as
/// components so Parabyzantine queries can reason over transport state.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TaskContainer {
	pub proposal: Component<Proposal>,
}

impl ContainerGiving<Proposal> for TaskContainer {
	fn as_component(&self) -> Component<&Proposal> {
		self.proposal.as_ref()
	}
}
