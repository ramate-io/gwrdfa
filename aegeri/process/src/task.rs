pub mod aegeri_task;
pub mod executor;
pub mod mempool;
pub mod transaction_store;

use crate::aegeri::AegeriParabyzantineData;
use aegeri_message::{
	AegeriSubcommittee, Index as AegeriIndex, Proposal as AegeriProposal, Transaction,
	VerifiedMessage,
};
pub use aegeri_task::{AegeriTask, AegeriTaskError};
pub use executor::{AegeriExecutionError, AegeriExecutor};
use gwrdfa_container::query::{
	matching_components::MatchingComponents, matching_tuple::MatchingTuple,
};
use gwrdfa_resample::{
	agreement::std::{Index, Subcom, Value},
	Resample,
};
pub use mempool::{Mempool, MempoolError};
use parabyzantine::task::{ParabyzantineTask, TaskWorld};
pub use transaction_store::{TransactionStore, TransactionStoreError};

impl ParabyzantineTask<AegeriParabyzantineData> for AegeriTask {
	fn update_parabyzantine_task(&mut self, data: &mut TaskWorld<AegeriParabyzantineData>) {
		// All of the participant logic is wrapped in this if statement.
		if self.is_participant() {
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
				match self.handle_value_agreement(&index.0, &value.0) {
					Ok((index, proposal)) => {
						// If we got a proposal, insert it into the task inferences.
						data.task_inferences.insert(None, (index, proposal));

						// Remove the [Resample] value on the agreement s.t. this doesn't get processed again
						data.agreement_inferences.remove::<Resample>(container_entity);
					}
					Err(e) => {
						log::error!("error handling agreement: {:?}", e);
					}
				}
			}

			// Handle pings.
			if self.pings() {
				if let Ok(now) = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
				{
					// Reset the ping if it's too old.
					let now_ms = now.as_millis() as u64;
					if now_ms - self.last_ping_time_ms() > self.ping_frequency_ms() {
						// ping record is expired, reset it
						self.set_last_ping(None);
					}

					// If there's a new ping, insert it into the task inferences.
					if let Some((_entity, (index, subcommittee))) = data
						.agreement_facts
						.query(
							MatchingTuple::<(Index<AegeriIndex>, Subcom<AegeriSubcommittee>)>::new(
							),
						)
						.next()
					{
						if Some((&index.0, &subcommittee.0)) != self.last_ping() {
							data.task_inferences.insert(
								None,
								(
									index.0.clone(),
									AegeriProposal::SubcommitteeBroadcast(subcommittee.0.clone()),
								),
							);

							self.set_last_ping(Some((index.0.clone(), subcommittee.0.clone())));
							self.set_last_ping_time_ms(now_ms);
						}
					}
				}
			}
		}

		// Receive all the transactions on the receiver and send the statuses.
		for transaction_result in self.receive_transaction_batch() {
			match transaction_result {
				Ok(transaction) => {
					self.add_inflight_transaction_id(*transaction.id());
					data.task_inferences.insert(None, transaction);
				}
				Err(e) => {
					data.task_inferences.insert(None, e);
				}
			}
		}

		// For each agreement, check if our inflight transaction ids are included in the agreement.
		for (_container_entity, (_resample, index, value)) in data
			.agreement_facts
			.query(MatchingTuple::<(Resample, Index<AegeriIndex>, Value<AegeriProposal>)>::new())
		{
			// TODO: we can optimize out this clone via disjoint borrows,
			// but we'll move this into a module refactor first.
			let inflight_transaction_ids = self.inflight_transaction_ids().clone();
			for inflight_transaction_id in inflight_transaction_ids.iter() {
				if value.0.contains_transaction_id(inflight_transaction_id) {
					if let Err(e) =
						self.try_send_transaction_status(index.0.clone(), *inflight_transaction_id)
					{
						data.task_inferences.insert(None, e);
					}
					self.remove_inflight_transaction_id(inflight_transaction_id);
				}
			}
		}
	}
}
