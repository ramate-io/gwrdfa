use aegeri_message::{Transaction, VerifiedMessage};
use gwrdfa_container::{Component, ContainerGiving};

/// Canonical message container used by the Gossamer Hart integration.
///
/// The container keeps the message payload plus lifecycle/error markers as
/// components so Parabyzantine queries can reason over transport state.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TransactionContainer {
	pub transaction: Component<VerifiedMessage<Transaction>>,
}

impl ContainerGiving<VerifiedMessage<Transaction>> for TransactionContainer {
	fn as_component(&self) -> Component<&VerifiedMessage<Transaction>> {
		self.transaction.as_ref()
	}
}
