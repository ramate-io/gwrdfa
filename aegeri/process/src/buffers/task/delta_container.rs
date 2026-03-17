use super::container::TaskContainer;
use crate::task::AegeriTaskError;
use aegeri_message::{Index, Proposal, Transaction};
use gwrdfa_container::{ContainerStores, Delta, DeltasContainer};

#[derive(Debug, Default)]
pub struct TaskDeltasContainer {
	/// Delta for proposal payload.
	pub proposal: Delta<Proposal>,
	/// Delta for index.
	pub index: Delta<Index>,
	/// Delta for transaction.
	pub transaction: Delta<Transaction>,
	/// Delta for task error.
	pub task_error: Delta<AegeriTaskError>,
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
			transaction: self.transaction.into_component(),
			task_error: self.task_error.into_component(),
		}
	}
}

impl ContainerStores<Proposal> for TaskDeltasContainer {
	fn from_data(data: Proposal) -> Self {
		Self {
			index: Delta::Unchanged,
			proposal: Delta::Modified(data),
			transaction: Delta::Unchanged,
			task_error: Delta::Unchanged,
		}
	}

	fn from_removed_data() -> Self {
		Self {
			index: Delta::Unchanged,
			proposal: Delta::Removed,
			transaction: Delta::Unchanged,
			task_error: Delta::Unchanged,
		}
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
		Self {
			index: Delta::Modified(data),
			proposal: Delta::Unchanged,
			transaction: Delta::Unchanged,
			task_error: Delta::Unchanged,
		}
	}

	fn from_removed_data() -> Self {
		Self {
			index: Delta::Removed,
			proposal: Delta::Unchanged,
			transaction: Delta::Unchanged,
			task_error: Delta::Unchanged,
		}
	}

	fn update_with_data(&mut self, data: Index) {
		self.index = Delta::Modified(data);
	}

	fn remove_from_container(&mut self) {
		self.index = Delta::Removed;
	}
}

impl ContainerStores<Transaction> for TaskDeltasContainer {
	fn from_data(data: Transaction) -> Self {
		Self {
			index: Delta::Unchanged,
			proposal: Delta::Unchanged,
			transaction: Delta::Modified(data),
			task_error: Delta::Unchanged,
		}
	}

	fn from_removed_data() -> Self {
		Self {
			index: Delta::Unchanged,
			proposal: Delta::Unchanged,
			transaction: Delta::Removed,
			task_error: Delta::Unchanged,
		}
	}

	fn update_with_data(&mut self, data: Transaction) {
		self.transaction = Delta::Modified(data);
	}

	fn remove_from_container(&mut self) {
		self.transaction = Delta::Removed;
	}
}

impl ContainerStores<AegeriTaskError> for TaskDeltasContainer {
	fn from_data(data: AegeriTaskError) -> Self {
		Self {
			index: Delta::Unchanged,
			proposal: Delta::Unchanged,
			transaction: Delta::Unchanged,
			task_error: Delta::Modified(data),
		}
	}

	fn from_removed_data() -> Self {
		Self {
			index: Delta::Unchanged,
			proposal: Delta::Unchanged,
			transaction: Delta::Unchanged,
			task_error: Delta::Removed,
		}
	}

	fn update_with_data(&mut self, data: AegeriTaskError) {
		self.task_error = Delta::Modified(data);
	}

	fn remove_from_container(&mut self) {
		self.task_error = Delta::Removed;
	}
}
