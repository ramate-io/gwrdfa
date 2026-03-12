pub mod aegeri_task;
pub mod executor;
pub mod mempool;
pub mod transaction_store;

use crate::aegeri::AegeriParabyzantineData;
use aegeri_message::{Transaction, VerifiedMessage};
pub use aegeri_task::{AegeriTask, AegeriTaskError};
pub use executor::{AegeriExecutionError, AegeriExecutor};
use gwrdfa_container::query::matching_components::MatchingComponents;
pub use mempool::{Mempool, MempoolError};
use parabyzantine::task::{ParabyzantineTask, TaskWorld};
pub use transaction_store::{TransactionStore, TransactionStoreError};

impl ParabyzantineTask<AegeriParabyzantineData> for AegeriTask {
	fn update_parabyzantine_task(&mut self, data: &mut TaskWorld<AegeriParabyzantineData>) {
		// insert all the transactions
		for (container_entity, transaction) in data
			.transaction_facts
			.query(MatchingComponents::<VerifiedMessage<Transaction>>::new())
		{
			self.add_transaction(transaction.clone());
		}
	}
}
