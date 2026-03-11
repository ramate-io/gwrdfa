use super::ByzantineRequirement;
use crate::TransactionSet;
use gwrdfa_resample::agreement::Condition;
use serde::{Deserialize, Serialize};

/// Exact block-header proposal from one replica.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct BlockHeader(TransactionSet);

impl BlockHeader {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn transactions(&self) -> &TransactionSet {
		&self.0
	}

	pub fn from_transactions(transactions: TransactionSet) -> Self {
		Self(transactions)
	}

	/// Aggregates exact block-header proposals with majority-style condition logic.
	pub fn consensus_condition<'a>(
		proposals: impl Iterator<Item = &'a BlockHeader>,
		requirement: ByzantineRequirement,
	) -> Condition<BlockHeader> {
		requirement.aggregate_majority(proposals)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use anyhow::Result;
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

	#[test]
	fn test_consensus_condition_reaches_consensus_on_quorum() {
		let a = BlockHeader::new();
		let condition = BlockHeader::consensus_condition(
			[&a, &a].into_iter(),
			ByzantineRequirement { total_voters: 3, quorum: 2 },
		);
		assert!(matches!(condition, Condition::Consensus(_)));
	}

	#[test]
	fn test_consensus_condition_hung_when_quorum_impossible() -> Result<()> {
		let a = BlockHeader::new();
		let mut set = TransactionSet::new();
		set.add_id(tx_id(1)?);
		let b = BlockHeader::from_transactions(set);
		let condition = BlockHeader::consensus_condition(
			[&a, &b].into_iter(),
			ByzantineRequirement { total_voters: 2, quorum: 2 },
		);
		assert!(matches!(condition, Condition::Hung));
		Ok(())
	}
}
