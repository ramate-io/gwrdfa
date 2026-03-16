use super::container::TaskContainer;
use aegeri_message::{Index, Proposal};
use gwrdfa_container::{ContainerStores, Delta, DeltasContainer};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TaskDeltasContainer {
	/// Delta for proposal payload.
	pub proposal: Delta<Proposal>,
	/// Delta for index.
	pub index: Delta<Index>,
}

impl DeltasContainer<TaskContainer> for TaskDeltasContainer {
	/// Apply all component deltas to an existing container instance.
	fn apply_deltas(self, container: &mut TaskContainer) {
		self.proposal.apply(&mut container.proposal);
	}

	fn into_container(self) -> TaskContainer {
		TaskContainer {
			index: self.index.into_component(),
			proposal: self.proposal.into_component(),
		}
	}
}

impl ContainerStores<Proposal> for TaskDeltasContainer {
	fn from_data(data: Proposal) -> Self {
		Self { index: Delta::Unchanged, proposal: Delta::Modified(data) }
	}

	fn from_removed_data() -> Self {
		Self { index: Delta::Unchanged, proposal: Delta::Removed }
	}

	fn update_with_data(&mut self, data: Proposal) {
		self.proposal = Delta::Modified(data);
	}

	fn remove_from_container(&mut self) {
		self.proposal = Delta::Removed;
	}
}

impl ContainerStores<Index> for TaskDeltasContainer {
	fn from_data(data: Index) -> Self {
		Self { index: Delta::Modified(data), proposal: Delta::Unchanged }
	}

	fn from_removed_data() -> Self {
		Self { index: Delta::Removed, proposal: Delta::Unchanged }
	}

	fn update_with_data(&mut self, data: Index) {
		self.index = Delta::Modified(data);
	}

	fn remove_from_container(&mut self) {
		self.index = Delta::Removed;
	}
}
