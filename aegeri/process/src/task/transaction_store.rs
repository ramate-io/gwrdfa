use aegeri_message::{Block, BlockHeader, Id, Transaction, VerifiedMessage};
use std::collections::HashMap;

/// Stores verified transactions by ID for block materialization.
#[derive(Debug, Default)]
pub struct TransactionStore {
	by_id: HashMap<Id, VerifiedMessage<Transaction>>,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum TransactionStoreError {
	#[error("missing transaction for id {0:?}")]
	MissingTransaction(Id),
}

impl TransactionStore {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn insert(&mut self, message: VerifiedMessage<Transaction>) {
		self.by_id.insert(*message.id(), message);
	}

	pub fn get(&self, id: &Id) -> Option<&VerifiedMessage<Transaction>> {
		self.by_id.get(id)
	}

	pub fn remove(&mut self, id: &Id) -> Option<VerifiedMessage<Transaction>> {
		self.by_id.remove(id)
	}

	/// Builds a block by fetching all transactions referenced by a block header.
	pub fn build_block_from_header_proposal(
		&self,
		block_header: &BlockHeader,
	) -> Result<Block, TransactionStoreError> {
		let transactions = block_header
			.transactions()
			.iter_ids()
			.map(|id| self.by_id.get(id).cloned().ok_or(TransactionStoreError::MissingTransaction(*id)))
			.collect::<Result<Vec<_>, _>>()?;
		Ok(Block::new(transactions))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use aegeri_message::{Message, Nonce};
	use anyhow::Result;
	use ml_dsa::{B32, MlDsa44, SigningKey};
	use std::collections::BTreeSet;

	fn tx(seed: u8, payload: Transaction, nonce: &[u8]) -> Result<VerifiedMessage<Transaction>> {
		let signer = SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![seed; 32]));
		let message = Message::<Transaction>::try_new(&signer, payload, Nonce::new(nonce))?;
		Ok(message.into_verified()?)
	}

	#[test]
	fn test_build_block_from_header_proposal_fetches_all_transactions() -> Result<()> {
		let mut store = TransactionStore::new();
		let tx_a = tx(1, Transaction::Join, b"a")?;
		let tx_b = tx(2, Transaction::Join, b"b")?;
		store.insert(tx_a.clone());
		store.insert(tx_b.clone());

		let mut ids = aegeri_message::TransactionSet::new();
		ids.add_id(*tx_a.id());
		ids.add_id(*tx_b.id());
		let header = BlockHeader::from_transactions(ids);

		let block = store.build_block_from_header_proposal(&header)?;
		let block_messages = block.transactions().cloned().collect::<BTreeSet<_>>();
		assert_eq!(block_messages, BTreeSet::from([tx_a, tx_b]));
		Ok(())
	}

	#[test]
	fn test_build_block_from_header_proposal_errors_when_missing_transaction() -> Result<()> {
		let mut store = TransactionStore::new();
		let tx_a = tx(3, Transaction::Join, b"a")?;
		let tx_missing = tx(4, Transaction::Join, b"missing")?;
		store.insert(tx_a.clone());

		let mut ids = aegeri_message::TransactionSet::new();
		ids.add_id(*tx_a.id());
		ids.add_id(*tx_missing.id());
		let header = BlockHeader::from_transactions(ids);

		let result = store.build_block_from_header_proposal(&header);
		assert_eq!(
			result.err(),
			Some(TransactionStoreError::MissingTransaction(*tx_missing.id()))
		);
		Ok(())
	}
}
