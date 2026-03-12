use crate::task::AegeriTaskError;
use aegeri_message::{Transaction, VerifiedMessage};
use gwrdfa_container::{Component, ContainerGiving};

/// Container for a verified transaction.
#[derive(Debug, Default)]
pub struct TransactionContainer {
	pub transaction: Component<VerifiedMessage<Transaction>>,
	pub task_error: Component<AegeriTaskError>,
}

impl ContainerGiving<VerifiedMessage<Transaction>> for TransactionContainer {
	fn as_component(&self) -> Component<&VerifiedMessage<Transaction>> {
		self.transaction.as_ref()
	}
}
