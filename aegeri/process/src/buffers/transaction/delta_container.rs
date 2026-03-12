use super::container::TransactionContainer;
use aegeri_message::{Transaction, VerifiedMessage};
use gwrdfa_container::{ContainerStores, Delta, DeltasContainer};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TransactionDeltasContainer {
	/// Delta for message payload.
	pub transaction: Delta<VerifiedMessage<Transaction>>,
}

impl DeltasContainer<TransactionContainer> for TransactionDeltasContainer {
	/// Apply all component deltas to an existing container instance.
	fn apply_deltas(self, container: &mut TransactionContainer) {
		self.transaction.apply(&mut container.transaction);
	}

	fn into_container(self) -> TransactionContainer {
		TransactionContainer { transaction: self.transaction.into_component() }
	}
}

impl ContainerStores<VerifiedMessage<Transaction>> for TransactionDeltasContainer {
	fn from_data(data: VerifiedMessage<Transaction>) -> Self {
		Self { transaction: Delta::Modified(data) }
	}

	fn from_removed_data() -> Self {
		Self { transaction: Delta::Removed }
	}

	fn update_with_data(&mut self, data: VerifiedMessage<Transaction>) {
		self.transaction = Delta::Modified(data);
	}

	fn remove_from_container(&mut self) {
		self.transaction = Delta::Removed;
	}
}
