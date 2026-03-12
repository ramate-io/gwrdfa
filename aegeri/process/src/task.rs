pub mod aegeri_task;
pub mod executor;
pub mod mempool;
pub mod transaction_store;

use crate::aegeri::AegeriParabyzantineData;
use aegeri_message::{
	Index as AegeriIndex, Proposal as AegeriProposal, Transaction, VerifiedMessage,
};
pub use aegeri_task::{AegeriTask, AegeriTaskError};
pub use executor::{AegeriExecutionError, AegeriExecutor};
use gwrdfa_container::query::{
	matching_components::MatchingComponents, matching_tuple::MatchingTuple,
};
use gwrdfa_resample::{
	agreement::std::{Index, Value},
	Resample,
};
pub use mempool::{Mempool, MempoolError};
use parabyzantine::task::{ParabyzantineTask, TaskWorld};
pub use transaction_store::{TransactionStore, TransactionStoreError};

impl ParabyzantineTask<AegeriParabyzantineData> for AegeriTask {
	fn update_parabyzantine_task(&mut self, data: &mut TaskWorld<AegeriParabyzantineData>) {
		// Insert all the transactions and report errors.
		for (container_entity, transaction) in data
			.transaction_facts
			.query(MatchingComponents::<VerifiedMessage<Transaction>>::new())
		{
			// If there's an error inserting, report it.
			if let Err(e) = self.add_transaction(transaction.clone()) {
				data.transaction_inferences.insert(Some(container_entity), e);
			} else {
				// Otherwise, remove the transaction.
				data.task_inferences.remove_entity(container_entity);
			}
		}

		// Handle agreements.
		for (container_entity, (_resample, index, value)) in data
			.agreement_facts
			.query(MatchingTuple::<(Resample, Index<AegeriIndex>, Value<AegeriProposal>)>::new())
		{
			// If there's an error handling the agreement, for now just log it.
			match self.handle_agreement(&index.0, &value.0) {
				Ok(Some(proposal)) => {
					// If we got a proposal, insert it into the task inferences.
					data.task_inferences.insert(None, proposal);
					// For now, we will do very simple eviction of the agreement by removing
					// the resample value.
					// This will keep the tape of the agreement,
					// but prevent it from being reprocessed.
					data.agreement_inferences.remove::<Resample>(container_entity);
				}
				Ok(None) => {
					// Same as above, but for no proposal.
					data.agreement_inferences.remove::<Resample>(container_entity);
				}
				Err(e) => {
					log::error!("error handling agreement: {:?}", e);
				}
			}
		}
	}
}
