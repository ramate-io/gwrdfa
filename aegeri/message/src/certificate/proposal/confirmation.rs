use super::ByzantineRequirement;
use crate::TransactionSet;
use gwrdfa_resample::agreement::Condition;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Confirmation proposal from one replica.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Confirmation(TransactionSet);

impl Confirmation {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn from_transactions(transactions: TransactionSet) -> Self {
		Self(transactions)
	}

	pub fn transactions(&self) -> &TransactionSet {
		&self.0
	}

	/// Aggregates confirmation proposals by counting per-transaction confirmations.
	pub fn aggregate<'a>(
		proposals: impl Iterator<Item = &'a Confirmation>,
		requirement: ByzantineRequirement,
	) -> Condition<Confirmation> {
		let collected = proposals.collect::<Vec<_>>();
		if collected.len() < requirement.quorum {
			return Condition::InProgress;
		}

		let mut counts = BTreeMap::new();
		for confirmation in collected {
			for id in confirmation.transactions().iter_ids() {
				*counts.entry(id.clone()).or_insert(0usize) += 1;
			}
		}

		let mut merged = TransactionSet::new();
		for (id, count) in counts {
			if count >= requirement.quorum {
				merged.add_id(id);
			}
		}
		Condition::Consensus(Confirmation::from_transactions(merged))
	}
}
