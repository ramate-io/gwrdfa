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
	use anyhow::{bail, Result};
	use crate::{Message, Nonce, Transaction};
	use ml_dsa::{B32, MlDsa44, SigningKey};

	fn tx_id(seed: u8) -> Result<crate::Id> {
		let signer = SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![seed; 32]));
		let message = Message::<Transaction>::try_new(
			&signer,
			Transaction::Join,
			Nonce::new([seed]),
		)?;
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
		let a = Availability::from_transactions(set([tx_id(1)?, tx_id(2)?]));
		let b = Availability::from_transactions(set([tx_id(2)?, tx_id(3)?]));
		let condition = Availability::consensus_condition(
			[&a, &b].into_iter(),
			ByzantineRequirement { total_voters: 3, quorum: 2 },
		);
		match condition {
			Condition::Consensus(availability) => assert_eq!(availability.transactions().len(), 3),
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
		assert!(matches!(condition, Condition::InProgress));
		Ok(())
	}
}
