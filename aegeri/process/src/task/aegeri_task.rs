use super::{
	AegeriExecutionError, AegeriExecutor, Mempool, MempoolError, TransactionStore,
	TransactionStoreError,
};
use aegeri_message::{AegeriSubcommittee, Index, Proposal, Transaction, VerifiedMessage};
use gwrdfa_resample::agreement::std::NextRound;

/// High-level task flow that coordinates mempool, storage, and execution.
pub struct AegeriTask {
	mempool: Mempool,
	transaction_store: TransactionStore,
	executor: AegeriExecutor,
	pings: bool,
	last_ping: Option<(Index, AegeriSubcommittee)>,
	ping_frequency_ms: u64,
	last_ping_time_ms: u64,
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
	#[error("provided index is not the next round of the provided proposal index")]
	NotNextRound,
	#[error("subcommittee broadcast not supported")]
	SubcommitteeBroadcastNotSupported,
}

impl AegeriTask {
	pub fn new(slot_width_ms: u64) -> Result<Self, AegeriTaskError> {
		Ok(Self {
			mempool: Mempool::new(slot_width_ms)?,
			transaction_store: TransactionStore::new(),
			executor: AegeriExecutor::new(),
			pings: false,
			last_ping: None,
			ping_frequency_ms: 1000,
			last_ping_time_ms: 0,
		})
	}

	pub fn with_pings(mut self, pings: bool) -> Self {
		self.pings = pings;
		self
	}

	pub fn pings(&self) -> bool {
		self.pings
	}

	pub fn last_ping(&self) -> Option<(&Index, &AegeriSubcommittee)> {
		self.last_ping.as_ref().map(|(index, subcommittee)| (index, subcommittee))
	}

	pub fn set_last_ping(&mut self, last_ping: Option<(Index, AegeriSubcommittee)>) {
		self.last_ping = last_ping;
	}

	pub fn ping_frequency_ms(&self) -> u64 {
		self.ping_frequency_ms
	}

	pub fn set_ping_frequency_ms(&mut self, ping_frequency_ms: u64) {
		self.ping_frequency_ms = ping_frequency_ms;
	}

	pub fn last_ping_time_ms(&self) -> u64 {
		self.last_ping_time_ms
	}

	pub fn set_last_ping_time_ms(&mut self, last_ping_time_ms: u64) {
		self.last_ping_time_ms = last_ping_time_ms;
	}

	pub fn slot_width_ms(&self) -> u64 {
		self.mempool.slot_width_ms()
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
	pub fn handle_value_agreement(
		&mut self,
		index: &Index,
		proposal: &Proposal,
	) -> Result<(Index, Proposal), AegeriTaskError> {
		if !Self::proposal_matches_index(index, proposal) {
			return Err(AegeriTaskError::StageMismatch);
		}

		match proposal {
			Proposal::Availability(availability) => {
				let confirmation = self.mempool.build_confirmation_proposal(index, availability)?;
				let next_index = index.next().ok_or(AegeriTaskError::NotNextRound)?;
				Ok((next_index, Proposal::Confirmation(confirmation)))
			}
			Proposal::Confirmation(confirmation) => {
				let block_header = self.mempool.build_block_header_proposal(index, confirmation)?;
				let next_index = index.next().ok_or(AegeriTaskError::NotNextRound)?;
				Ok((next_index, Proposal::BlockHeader(block_header)))
			}
			Proposal::BlockHeader(block_header) => {
				let block =
					self.transaction_store.build_block_from_header_proposal(block_header)?;
				let transition = self.executor.execute_block(&block)?;

				for id in block_header.transactions().iter_ids() {
					self.transaction_store.remove(id);
					self.mempool.remove(*id);
				}

				let next_index = index.next().ok_or(AegeriTaskError::NotNextRound)?;
				Ok((next_index, Proposal::Transition(transition)))
			}
			Proposal::Transition(_) => {
				let next_index = index.next().ok_or(AegeriTaskError::NotNextRound)?;
				let now_epoch_ms = std::time::SystemTime::now()
					.duration_since(std::time::UNIX_EPOCH)
					.unwrap()
					.as_millis() as u64;
				let availability_proposal =
					self.mempool.build_availability_proposal(now_epoch_ms, 128, &next_index)?;
				Ok((next_index, Proposal::Availability(availability_proposal)))
			}
			Proposal::SubcommitteeBroadcast(_subcommittee) => {
				// This is not the channel for handling subcommittee broadcasts.
				// They trigger no value agreement handling.
				Err(AegeriTaskError::SubcommitteeBroadcastNotSupported)
			}
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
		let proposal = Proposal::Confirmation(Default::default());
		let result = task.handle_value_agreement(&index, &proposal);
		assert!(matches!(result, Err(AegeriTaskError::StageMismatch)));
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
		let proposal = Proposal::BlockHeader(header);

		let output = task.handle_value_agreement(&index, &proposal)?;
		assert!(matches!(output, (_, Proposal::Transition(_))));
		assert!(task.transaction_store.get(&id).is_none());
		Ok(())
	}
}
