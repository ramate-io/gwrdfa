use super::{
	AegeriExecutionError, AegeriExecutor, Mempool, MempoolError, TransactionStore,
	TransactionStoreError,
};
use aegeri_message::{Certificate, Index, Proposal, Transaction, VerifiedMessage};

/// High-level task flow that coordinates mempool, storage, and execution.
pub struct AegeriTask {
	mempool: Mempool,
	transaction_store: TransactionStore,
	executor: AegeriExecutor,
}

#[derive(Debug, thiserror::Error)]
pub enum AegeriTaskError {
	#[error("mempool error: {0}")]
	Mempool(#[from] MempoolError),
	#[error("transaction store error: {0}")]
	TransactionStore(#[from] TransactionStoreError),
	#[error("execution error: {0}")]
	Execution(#[from] AegeriExecutionError),
	#[error("provided index does not match certificate index")]
	IndexMismatch,
	#[error("provided index does not match certificate proposal stage")]
	StageMismatch,
}

impl AegeriTask {
	pub fn new(slot_width_ms: u64) -> Result<Self, AegeriTaskError> {
		Ok(Self {
			mempool: Mempool::new(slot_width_ms)?,
			transaction_store: TransactionStore::new(),
			executor: AegeriExecutor::new(),
		})
	}

	/// Adds a transaction to both persistent verified storage and mempool scheduling.
	pub fn add_transaction(
		&mut self,
		transaction: VerifiedMessage<Transaction>,
	) -> Result<(), AegeriTaskError> {
		let id = *transaction.id();
		self.transaction_store.insert(transaction);
		self.mempool.insert_now(id)?;
		Ok(())
	}

	/// Handles agreement for a specific index and returns the next-stage proposal.
	pub fn handle_agreement(
		&mut self,
		index: &Index,
		certificate: &Certificate,
	) -> Result<Option<Proposal>, AegeriTaskError> {
		if certificate.index() != index {
			return Err(AegeriTaskError::IndexMismatch);
		}
		if !Self::proposal_matches_index(index, certificate.value()) {
			return Err(AegeriTaskError::StageMismatch);
		}

		match certificate.value() {
			Proposal::Availability(availability) => {
				let confirmation = self.mempool.build_confirmation_proposal(index, availability)?;
				Ok(Some(Proposal::Confirmation(confirmation)))
			}
			Proposal::Confirmation(confirmation) => {
				let block_header = self.mempool.build_block_header_proposal(index, confirmation)?;
				Ok(Some(Proposal::BlockHeader(block_header)))
			}
			Proposal::BlockHeader(block_header) => {
				let block =
					self.transaction_store.build_block_from_header_proposal(block_header)?;
				let transition = self.executor.execute_block(&block)?;

				for id in block_header.transactions().iter_ids() {
					self.transaction_store.remove(id);
					self.mempool.remove(*id);
				}

				Ok(Some(Proposal::Transition(transition)))
			}
			Proposal::Transition(_) => Ok(None),
		}
	}

	fn proposal_matches_index(index: &Index, proposal: &Proposal) -> bool {
		matches!(
			(index, proposal),
			(Index::Availability(_), Proposal::Availability(_))
				| (Index::Confirmation(_), Proposal::Confirmation(_))
				| (Index::Block(_), Proposal::BlockHeader(_))
				| (Index::Transition(_), Proposal::Transition(_))
		)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use aegeri_message::{BlockHeader, IndexValue, Message, Nonce, TransactionSet};
	use anyhow::Result;
	use ml_dsa::{MlDsa44, SigningKey, B32};

	fn tx(seed: u8, nonce: &[u8]) -> Result<VerifiedMessage<Transaction>> {
		let signer = SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![seed; 32]));
		let message =
			Message::<Transaction>::try_new(&signer, Transaction::Join, Nonce::new(nonce))?;
		Ok(message.into_verified()?)
	}

	#[test]
	fn test_handle_agreement_returns_error_on_stage_mismatch() -> Result<()> {
		let mut task = AegeriTask::new(100)?;
		let index = Index::Availability(IndexValue(1));
		let certificate = Certificate::new(index, Proposal::Confirmation(Default::default()));
		let result = task.handle_agreement(&index, &certificate);
		assert!(matches!(result, Err(AegeriTaskError::StageMismatch)));
		Ok(())
	}

	#[test]
	fn test_handle_agreement_returns_error_on_index_mismatch() -> Result<()> {
		let mut task = AegeriTask::new(100)?;
		let index = Index::Availability(IndexValue(1));
		let certificate = Certificate::new(
			Index::Availability(IndexValue(2)),
			Proposal::Availability(Default::default()),
		);
		let result = task.handle_agreement(&index, &certificate);
		assert!(matches!(result, Err(AegeriTaskError::IndexMismatch)));
		Ok(())
	}

	#[test]
	fn test_handle_agreement_block_stage_executes_and_prunes() -> Result<()> {
		let mut task = AegeriTask::new(100)?;
		let transaction = tx(7, b"a")?;
		let id = *transaction.id();
		task.add_transaction(transaction)?;

		let mut txs = TransactionSet::new();
		txs.add_id(id);
		let header = BlockHeader::from_transactions(txs);
		let index = Index::Block(IndexValue(5));
		let certificate = Certificate::new(index, Proposal::BlockHeader(header));

		let output = task.handle_agreement(&index, &certificate)?;
		assert!(matches!(output, Some(Proposal::Transition(_))));
		assert!(task.transaction_store.get(&id).is_none());
		Ok(())
	}
}
