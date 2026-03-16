use super::ByzantineRequirement;
use crate::TransactionSet;
use gwrdfa_resample::agreement::Condition;
use serde::{Deserialize, Serialize};

/// Availability proposal from one replica.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Availability(TransactionSet);

impl Availability {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn genesis() -> Self {
		Self(TransactionSet::new())
	}

	pub fn from_transactions(transactions: TransactionSet) -> Self {
		Self(transactions)
	}

	pub fn transactions(&self) -> &TransactionSet {
		&self.0
	}

	/// Aggregates availability proposals into a union once quorum proposals exist.
	pub fn consensus_condition<'a>(
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{Message, Nonce, Transaction};
	use anyhow::{bail, Result};
	use ml_dsa::{MlDsa44, SigningKey, B32};
	use std::collections::BTreeSet;

	fn tx_id(seed: u8) -> Result<crate::Id> {
		let signer = SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![seed; 32]));
		let message =
			Message::<Transaction>::try_new(&signer, Transaction::Join, Nonce::new([seed]))?;
		Ok(message.id().clone())
	}

	fn set(ids: impl IntoIterator<Item = crate::Id>) -> TransactionSet {
		let mut s = TransactionSet::new();
		for id in ids {
			s.add_id(id);
		}
		s
	}

	#[test]
	fn test_consensus_condition_unions_on_quorum() -> Result<()> {
		let id1 = tx_id(1)?;
		let id2 = tx_id(2)?;
		let id3 = tx_id(3)?;
		let a = Availability::from_transactions(set([id1.clone(), id2.clone()]));
		let b = Availability::from_transactions(set([id2.clone(), id3.clone()]));
		let condition = Availability::consensus_condition(
			[&a, &b].into_iter(),
			ByzantineRequirement { total_voters: 3, quorum: 2 },
		);
		match condition {
			Condition::Consensus(availability) => {
				let expected = BTreeSet::from([id1, id2, id3]);
				assert_eq!(availability.transactions().ids(), &expected);
			}
			other => bail!("unexpected condition: {other:?}"),
		}
		Ok(())
	}

	#[test]
	fn test_consensus_condition_in_progress_without_quorum() -> Result<()> {
		let a = Availability::from_transactions(set([tx_id(1)?]));
		let condition = Availability::consensus_condition(
			[&a].into_iter(),
			ByzantineRequirement { total_voters: 3, quorum: 2 },
		);
		assert_eq!(condition, Condition::InProgress);
		Ok(())
	}
}
