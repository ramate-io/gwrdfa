use super::ByzantineRequirement;
use crate::TransactionSet;
use gwrdfa_resample::agreement::Condition;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Confirmation proposal from one replica.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
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
	pub fn consensus_condition<'a>(
		proposals: impl Iterator<Item = &'a Confirmation>,
		requirement: ByzantineRequirement,
	) -> Condition<Confirmation> {
		let collected = proposals.collect::<Vec<_>>();
		if !requirement.reaches_quorum(collected.len()) {
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
			if requirement.reaches_quorum(count) {
				merged.add_id(id);
			}
		}
		Condition::Consensus(Confirmation::from_transactions(merged))
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
	fn test_consensus_condition_merges_quorum_confirmed_ids() -> Result<()> {
		let id1 = tx_id(1)?;
		let id2 = tx_id(2)?;
		let id3 = tx_id(3)?;

		let a = Confirmation::from_transactions(set([id1.clone(), id2]));
		let b = Confirmation::from_transactions(set([id1.clone(), id3]));
		let c = Confirmation::from_transactions(set([id1.clone()]));
		let condition = Confirmation::consensus_condition(
			[&a, &b, &c].into_iter(),
			ByzantineRequirement { total_voters: 4, quorum: 3 },
		);

		match condition {
			Condition::Consensus(merged) => {
				let expected = BTreeSet::from([id1]);
				assert_eq!(merged.transactions().ids(), &expected);
			}
			other => bail!("unexpected condition: {other:?}"),
		}
		Ok(())
	}

	#[test]
	fn test_consensus_condition_in_progress_without_quorum() -> Result<()> {
		let a = Confirmation::from_transactions(set([tx_id(1)?]));
		let condition = Confirmation::consensus_condition(
			[&a].into_iter(),
			ByzantineRequirement { total_voters: 3, quorum: 2 },
		);
		assert_eq!(condition, Condition::InProgress);
		Ok(())
	}
}
