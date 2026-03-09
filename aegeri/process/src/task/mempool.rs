use aegeri_message::{Message, Transaction};
use std::collections::{BTreeMap, BTreeSet};
use std::time::{SystemTime, UNIX_EPOCH};

/// Slot-bucketed mempool for Aegeri transactions.
///
/// Transactions are inserted with a receive timestamp (epoch millis) and grouped
/// by `slot = received_at_ms / slot_width_ms`.
///
/// Pop behavior is intentionally conservative for expected global delivery lag:
/// only transactions from the previous slot (`t - 1`) are eligible to pop.
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

	/// Pops up to `max_items` transactions from the previous slot (`t - 1`) only.
	///
	/// Within that slot, items are popped from the end of the `BTreeSet`, i.e. the
	/// largest transaction value according to `Ord`.
	pub fn pop_ready_at(&mut self, now_epoch_ms: u64, max_items: usize) -> Vec<Message<Transaction>> {
		if max_items == 0 {
			return Vec::new();
		}

		let current_slot = self.slot_for_epoch_ms(now_epoch_ms);
		if current_slot == 0 {
			return Vec::new();
		}
		let eligible_slot = current_slot - 1;

		let mut popped = Vec::new();
		if let Some(slot_set) = self.by_slot.get_mut(&eligible_slot) {
			while popped.len() < max_items {
				let Some(message) = slot_set.pop_last() else {
					break;
				};
				popped.push(message);
			}
			if slot_set.is_empty() {
				self.by_slot.remove(&eligible_slot);
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
	use ml_dsa::{B32, MlDsa44, SigningKey};

	fn tx(seed: u8, payload: Transaction, nonce: &[u8]) -> Message<Transaction> {
		let signer = SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![seed; 32]));
		Message::<Transaction>::try_new(&signer, payload, aegeri_message::Nonce::new(nonce))
			.expect("message should sign")
	}

	#[test]
	fn does_not_pop_transactions_from_current_slot() {
		let mut mempool = Mempool::new(100).expect("valid slot width");
		let message = tx(1, Transaction::Join, b"a");

		// Slot 10
		mempool.insert_at(1000, message.clone());

		// Still slot 10 -> eligible slot is 9, so nothing pops.
		assert!(mempool.pop_ready_at(1099, 10).is_empty());

		// Slot 11 -> eligible slot is 10, now it pops.
		let popped = mempool.pop_ready_at(1100, 10);
		assert_eq!(popped.len(), 1);
		assert_eq!(popped[0], message);
	}

	#[test]
	fn pops_only_from_previous_slot_even_if_older_slots_exist() {
		let mut mempool = Mempool::new(100).expect("valid slot width");
		let older = tx(2, Transaction::Join, b"older");
		let previous = tx(3, Transaction::Join, b"previous");

		// Slots 8 and 9
		mempool.insert_at(850, older);
		mempool.insert_at(950, previous.clone());

		// Current slot 10: only slot 9 is eligible.
		let popped = mempool.pop_ready_at(1000, 10);
		assert_eq!(popped, vec![previous]);

		// Current slot 11: now slot 10 is eligible (empty), slot 8 still not popped.
		assert!(mempool.pop_ready_at(1100, 10).is_empty());
	}

	#[test]
	fn pops_highest_ordered_transactions_first_within_slot() {
		let mut mempool = Mempool::new(100).expect("valid slot width");
		let a = tx(4, Transaction::Join, b"a");
		let b = tx(5, Transaction::Join, b"b");
		let c = tx(6, Transaction::Join, b"c");

		// Insert all into slot 3.
		mempool.insert_at(300, a.clone());
		mempool.insert_at(310, b.clone());
		mempool.insert_at(320, c.clone());

		// Build expected descending-by-Ord order.
		let mut expected = vec![a, b, c];
		expected.sort();
		expected.reverse();

		// now in slot 4, so slot 3 is eligible.
		let popped = mempool.pop_ready_at(400, 3);
		assert_eq!(popped, expected);
	}
}
