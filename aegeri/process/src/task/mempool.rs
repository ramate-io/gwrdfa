use aegeri_message::{Message, Transaction};
use std::collections::{BTreeMap, BTreeSet};
use std::time::{SystemTime, UNIX_EPOCH};

/// Slot-bucketed mempool for Aegeri transactions.
///
/// Transactions are inserted with a receive timestamp (epoch millis) and grouped
/// by `slot = received_at_ms / slot_width_ms`.
///
/// Pop behavior uses a one-slot maturity delay:
/// transactions from slots `< t - 1` are eligible at slot `t`.
pub struct Mempool {
	slot_width_ms: u64,
	by_slot: BTreeMap<u64, BTreeSet<Message<Transaction>>>,
}

#[derive(Debug, thiserror::Error)]
pub enum MempoolError {
	#[error("slot_width_ms must be greater than zero")]
	InvalidSlotWidth,
	#[error("system time is before UNIX_EPOCH")]
	SystemTimeBeforeEpoch,
}

impl Mempool {
	pub fn new(slot_width_ms: u64) -> Result<Self, MempoolError> {
		if slot_width_ms == 0 {
			return Err(MempoolError::InvalidSlotWidth);
		}
		Ok(Self { slot_width_ms, by_slot: BTreeMap::new() })
	}

	pub fn slot_width_ms(&self) -> u64 {
		self.slot_width_ms
	}

	pub fn slot_for_epoch_ms(&self, epoch_ms: u64) -> u64 {
		epoch_ms / self.slot_width_ms
	}

	pub fn insert_at(&mut self, received_at_epoch_ms: u64, message: Message<Transaction>) {
		let slot = self.slot_for_epoch_ms(received_at_epoch_ms);
		self.by_slot.entry(slot).or_default().insert(message);
	}

	pub fn insert_now(&mut self, message: Message<Transaction>) -> Result<(), MempoolError> {
		let now_ms = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.map_err(|_| MempoolError::SystemTimeBeforeEpoch)?
			.as_millis() as u64;
		self.insert_at(now_ms, message);
		Ok(())
	}

	/// Pops up to `max_items` transactions from any eligible slot `< t - 1`.
	///
	/// Pop priority is:
	/// 1. Newer eligible slots first (global recency by slot).
	/// 2. Within each slot, larger `Ord` values first (`BTreeSet::pop_last`).
	pub fn pop_ready_at(&mut self, now_epoch_ms: u64, max_items: usize) -> Vec<Message<Transaction>> {
		if max_items == 0 {
			return Vec::new();
		}

		let current_slot = self.slot_for_epoch_ms(now_epoch_ms);
		if current_slot <= 1 {
			return Vec::new();
		}
		let upper_exclusive = current_slot - 1;
		let mut popped = Vec::new();
		let eligible_slots: Vec<u64> = self
			.by_slot
			.range(..upper_exclusive)
			.map(|(slot, _)| *slot)
			.collect();
		for slot in eligible_slots.into_iter().rev() {
			if popped.len() >= max_items {
				break;
			}
			if let Some(slot_set) = self.by_slot.get_mut(&slot) {
				while popped.len() < max_items {
					let Some(message) = slot_set.pop_last() else {
						break;
					};
					popped.push(message);
				}
				if slot_set.is_empty() {
					self.by_slot.remove(&slot);
				}
			}
		}
		popped
	}

	pub fn pop_ready_now(&mut self, max_items: usize) -> Result<Vec<Message<Transaction>>, MempoolError> {
		let now_ms = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.map_err(|_| MempoolError::SystemTimeBeforeEpoch)?
			.as_millis() as u64;
		Ok(self.pop_ready_at(now_ms, max_items))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use anyhow::Result;
	use ml_dsa::{B32, MlDsa44, SigningKey};

	fn tx(seed: u8, payload: Transaction, nonce: &[u8]) -> Result<Message<Transaction>> {
		let signer = SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![seed; 32]));
		Ok(Message::<Transaction>::try_new(
			&signer,
			payload,
			aegeri_message::Nonce::new(nonce),
		)?)
	}

	#[test]
	fn test_does_not_pop_transactions_from_current_slot() -> Result<()> {
		let mut mempool = Mempool::new(100)?;
		let message = tx(1, Transaction::Join, b"a")?;

		// Slot 10
		mempool.insert_at(1000, message.clone());

		// Still slot 10 -> no slot is < (10 - 1) while message is in slot 10.
		assert!(mempool.pop_ready_at(1099, 10).is_empty());

		// Slot 11 -> eligible slots are < 10, so slot 10 still does not pop.
		assert!(mempool.pop_ready_at(1100, 10).is_empty());

		// Slot 12 -> eligible slots are < 11, so slot 10 pops now.
		let popped = mempool.pop_ready_at(1200, 10);
		assert_eq!(popped.len(), 1);
		assert_eq!(popped[0], message);
		Ok(())
	}

	#[test]
	fn test_pops_from_any_eligible_slot_less_than_current_minus_one() -> Result<()> {
		let mut mempool = Mempool::new(100)?;
		let older = tx(2, Transaction::Join, b"older")?;
		let previous = tx(3, Transaction::Join, b"previous")?;

		// Slots 8 and 9
		mempool.insert_at(850, older.clone());
		mempool.insert_at(950, previous.clone());

		// Current slot 10: only slots < 9 are eligible (slot 8).
		let popped = mempool.pop_ready_at(1000, 10);
		assert_eq!(popped, vec![older]);

		// Current slot 11: slots < 10 are eligible, so slot 9 pops.
		assert_eq!(mempool.pop_ready_at(1100, 10), vec![previous]);
		Ok(())
	}

	#[test]
	fn test_pops_highest_ordered_transactions_first_within_slot() -> Result<()> {
		let mut mempool = Mempool::new(100)?;
		let a = tx(4, Transaction::Join, b"a")?;
		let b = tx(5, Transaction::Join, b"b")?;
		let c = tx(6, Transaction::Join, b"c")?;

		// Insert all into slot 3.
		mempool.insert_at(300, a.clone());
		mempool.insert_at(310, b.clone());
		mempool.insert_at(320, c.clone());

		// Build expected descending-by-Ord order.
		let mut expected = vec![a, b, c];
		expected.sort();
		expected.reverse();

		// Now in slot 4: eligible slots are < 3, so slot 3 is not eligible yet.
		assert!(mempool.pop_ready_at(400, 3).is_empty());
		// Now in slot 5: eligible slots are < 4, so slot 3 is eligible.
		let popped = mempool.pop_ready_at(500, 3);
		assert_eq!(popped, expected);
		Ok(())
	}
}
