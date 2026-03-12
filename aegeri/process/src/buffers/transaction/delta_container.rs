use super::container::TransactionContainer;
use crate::task::AegeriTaskError;
use aegeri_message::{Transaction, VerifiedMessage};
use gwrdfa_container::{ContainerStores, Delta, DeltasContainer};

#[derive(Debug, Default)]
pub struct TransactionDeltasContainer {
	/// Delta for message payload.
	pub transaction: Delta<VerifiedMessage<Transaction>>,

	/// Delta for task error.
	pub task_error: Delta<AegeriTaskError>,
}

impl DeltasContainer<TransactionContainer> for TransactionDeltasContainer {
	/// Apply all component deltas to an existing container instance.
	fn apply_deltas(self, container: &mut TransactionContainer) {
		self.transaction.apply(&mut container.transaction);
	}

	fn into_container(self) -> TransactionContainer {
		TransactionContainer {
			transaction: self.transaction.into_component(),
			task_error: self.task_error.into_component(),
		}
	}
}

impl ContainerStores<VerifiedMessage<Transaction>> for TransactionDeltasContainer {
	fn from_data(data: VerifiedMessage<Transaction>) -> Self {
		Self { transaction: Delta::Modified(data), task_error: Delta::Unchanged }
	}

	fn from_removed_data() -> Self {
		Self { transaction: Delta::Removed, task_error: Delta::Unchanged }
	}

	fn update_with_data(&mut self, data: VerifiedMessage<Transaction>) {
		self.transaction = Delta::Modified(data);
	}

	fn remove_from_container(&mut self) {
		self.transaction = Delta::Removed;
	}
}

impl ContainerStores<AegeriTaskError> for TransactionDeltasContainer {
	fn from_data(data: AegeriTaskError) -> Self {
		Self { transaction: Delta::Unchanged, task_error: Delta::Modified(data) }
	}

	fn from_removed_data() -> Self {
		Self { transaction: Delta::Unchanged, task_error: Delta::Removed }
	}

	fn update_with_data(&mut self, data: AegeriTaskError) {
		self.task_error = Delta::Modified(data);
	}

	fn remove_from_container(&mut self) {
		self.task_error = Delta::Removed;
	}
}
