use super::ByzantineRequirement;
use crate::TransactionSet;
use gwrdfa_resample::agreement::Condition;
use serde::{Deserialize, Serialize};

/// Availability proposal from one replica.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Availability(TransactionSet);

impl Availability {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn from_transactions(transactions: TransactionSet) -> Self {
		Self(transactions)
	}

	pub fn transactions(&self) -> &TransactionSet {
		&self.0
	}

	/// Aggregates availability proposals into a union once quorum proposals exist.
	pub fn aggregate<'a>(
		proposals: impl Iterator<Item = &'a Availability>,
		requirement: ByzantineRequirement,
	) -> Condition<Availability> {
		let collected = proposals.collect::<Vec<_>>();
		if collected.len() < requirement.quorum {
			return Condition::InProgress;
		}
		let mut union = TransactionSet::new();
		for availability in collected {
			for id in availability.transactions().iter_ids() {
				union.add_id(id.clone());
			}
		}
		Condition::Consensus(Availability::from_transactions(union))
	}
}
