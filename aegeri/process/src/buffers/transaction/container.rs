use aegeri_message::{Transaction, VerifiedMessage};
use gwrdfa_container::{Component, ContainerGiving};

/// Container for a verified transaction.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TransactionContainer {
	pub transaction: Component<VerifiedMessage<Transaction>>,
}

impl ContainerGiving<VerifiedMessage<Transaction>> for TransactionContainer {
	fn as_component(&self) -> Component<&VerifiedMessage<Transaction>> {
		self.transaction.as_ref()
	}
}
