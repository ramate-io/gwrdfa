use super::container::TaskContainer;
use aegeri_message::Proposal;
use gwrdfa_container::{ContainerStores, Delta, DeltasContainer};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TaskDeltasContainer {
	/// Delta for message payload.
	pub proposal: Delta<Proposal>,
}

impl DeltasContainer<TaskContainer> for TaskDeltasContainer {
	/// Apply all component deltas to an existing container instance.
	fn apply_deltas(self, container: &mut TaskContainer) {
		self.proposal.apply(&mut container.proposal);
	}

	fn into_container(self) -> TaskContainer {
		TaskContainer { proposal: self.proposal.into_component() }
	}
}

impl ContainerStores<Proposal> for TaskDeltasContainer {
	fn from_data(data: Proposal) -> Self {
		Self { proposal: Delta::Modified(data) }
	}

	fn from_removed_data() -> Self {
		Self { proposal: Delta::Removed }
	}

	fn update_with_data(&mut self, data: Proposal) {
		self.proposal = Delta::Modified(data);
	}

	fn remove_from_container(&mut self) {
		self.proposal = Delta::Removed;
	}
}
