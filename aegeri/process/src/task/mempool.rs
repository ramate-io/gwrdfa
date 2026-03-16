use aegeri_message::{
	Availability, BlockHeader, Confirmation, Id, Index, IndexValue, TransactionSet,
};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Slot(u64);

/// Slot-bucketed mempool for verified Aegeri transactions.
///
/// Design notes:
/// - `by_slot` is the pool backing storage and ordering index.
/// - `inflight` maps transaction IDs to `(index, original_slot)` ownership.
/// - Popping from slots moves IDs into `inflight`.
/// - Advancing to a new index preserves the original slot.
/// - Returning IDs to the pool re-inserts them with the preserved slot.
pub struct Mempool {
	slot_width_ms: u64,
	/// Id to slot mapping (bijection) for returning ids to the pool.
	by_id: HashMap<Id, Slot>,
	/// Ordered backing storage used for candidate selection.
	by_slot: BTreeMap<Slot, BTreeSet<Id>>,
	/// In-flight ownership by consensus index and original slot.
	inflight_by_id: HashMap<Id, IndexValue>,
	/// In-flight ownership by consensus index and original slot.
	inflight_by_index: HashMap<IndexValue, HashSet<Id>>,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone)]
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
		Ok(Self {
			slot_width_ms,
			by_id: HashMap::new(),
			by_slot: BTreeMap::new(),
			inflight_by_id: HashMap::new(),
			inflight_by_index: HashMap::new(),
		})
	}

	pub fn slot_width_ms(&self) -> u64 {
		self.slot_width_ms
	}

	fn slot_for_epoch_ms(&self, epoch_ms: u64) -> Slot {
		Slot(epoch_ms / self.slot_width_ms)
	}

	fn insert_entry(&mut self, received_at_epoch_ms: u64, id: Id) {
		if self.by_id.contains_key(&id) {
			return;
		}
		let slot = self.slot_for_epoch_ms(received_at_epoch_ms);
		self.by_slot.entry(slot).or_default().insert(id);
		self.by_id.insert(id, slot);
	}

	pub fn insert_at(&mut self, received_at_epoch_ms: u64, id: Id) {
		self.insert_entry(received_at_epoch_ms, id);
	}

	pub fn insert_now(&mut self, id: Id) -> Result<(), MempoolError> {
		let now_ms = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.map_err(|_| MempoolError::SystemTimeBeforeEpoch)?
			.as_millis() as u64;
		self.insert_at(now_ms, id);
		Ok(())
	}

	/// Marks an id as inflight at a given index value.
	pub fn mark_inflight(&mut self, id: Id, index_value: IndexValue) {
		// Make sure to get rid of the id in the old index value mapping.
		if let Some(existing_index_value) = self.inflight_by_id.get(&id) {
			if existing_index_value != &index_value {
				self.inflight_by_index.entry(*existing_index_value).or_default().remove(&id);
			}
		}

		self.inflight_by_id.insert(id, index_value);
		self.inflight_by_index.entry(index_value).or_default().insert(id);
	}

	/// Unmarks an id as inflight at a given index value.
	pub fn unmark_inflight_at(&mut self, id: Id, index_value: IndexValue) {
		self.inflight_by_id.remove(&id);
		self.inflight_by_index.entry(index_value).or_default().remove(&id);
	}

	/// Removes an id from inflight
	pub fn unmark_inflight(&mut self, id: Id) {
		if let Some(index_value) = self.inflight_by_id.get(&id) {
			self.unmark_inflight_at(id, *index_value);
		}
	}

	/// Removes an id from the mempool entirely.
	pub fn remove(&mut self, id: Id) {
		self.unmark_inflight(id);

		if let Some(slot) = self.by_id.get(&id) {
			self.by_slot.entry(*slot).or_default().remove(&id);
		}
	}

	/// Pops up to `max_items` transactions from any eligible slot `< t - 1`.
	///
	/// Pop priority:
	/// 1. Newer eligible slots first.
	/// 2. Within each slot, larger `Id` values first.
	fn pop_ready_at(
		&mut self,
		now_epoch_ms: u64,
		max_items: usize,
		index_value: IndexValue,
	) -> Vec<Id> {
		if max_items == 0 {
			return Vec::new();
		}

		let current_slot = self.slot_for_epoch_ms(now_epoch_ms);
		if current_slot.0 <= 1 {
			return Vec::new();
		}
		let upper_exclusive = Slot(current_slot.0 - 1);
		let eligible_slots =
			self.by_slot.range(..upper_exclusive).map(|(slot, _)| *slot).collect::<Vec<_>>();

		let mut selected: Vec<Id> = Vec::new();
		for slot in eligible_slots.into_iter().rev() {
			let Some(ids) = self.by_slot.get_mut(&slot) else {
				continue;
			};

			for id in ids.iter() {
				if selected.len() > max_items - 1 {
					break;
				}

				// If the ID is already in-flight, it should not be selected again.
				if self.inflight_by_id.contains_key(&id) {
					continue;
				}

				selected.push(*id);
			}

			// If the slot is empty, it should be removed from the pool.
			if ids.is_empty() {
				self.by_slot.remove(&slot);
			}

			// If we have selected enough items, we can stop.
			if selected.len() >= max_items {
				break;
			}
		}

		// Mark the selected ids as inflight at the given index value.
		for id in &selected {
			self.mark_inflight(*id, index_value);
		}

		selected
	}

	/// Builds an availability proposal from the mempool.
	///
	/// This is done without any reference to an agreed transaction set.
	pub fn build_availability_proposal(
		&mut self,
		now_epoch_ms: u64,
		max_items: usize,
		index: &Index,
	) -> Result<Availability, MempoolError> {
		let index_value = index.value();
		let selected_ids = self.pop_ready_at(now_epoch_ms, max_items, index_value);
		let mut ids = TransactionSet::new();
		for id in selected_ids {
			ids.add_id(id);
		}
		Ok(Availability::from_transactions(ids))
	}

	/// Reconciles the mempool with a given transaction set.
	pub fn reconcile_transactions(
		&mut self,
		index_value: IndexValue,
		transactions: &TransactionSet,
	) -> TransactionSet {
		let mut reconciled = TransactionSet::new();

		// Mark all the ids as inflight at the given index value.
		for id in transactions.iter_ids() {
			if self.by_id.contains_key(id) {
				reconciled.add_id(*id);
				self.mark_inflight(*id, index_value);
			}
		}

		// We have an agreement that doesn't include some of our transactions.
		// We can try to repropose them later
		// but for we aren't going to at this index given the agreement
		// we were handed down in the [TransactionSet].
		let mut to_unmark = Vec::new();
		for id in self.inflight_by_index.get(&index_value).into_iter().flatten() {
			if !reconciled.contains(id) {
				to_unmark.push(*id);
			}
		}
		for id in to_unmark {
			self.unmark_inflight(id);
		}

		reconciled
	}

	/// Builds a confirmation proposal from the mempool.
	///
	/// This is done with a reference to an availability proposal agreement.
	pub fn build_confirmation_proposal(
		&mut self,
		index: &Index,
		availability: &Availability,
	) -> Result<Confirmation, MempoolError> {
		let index_value = index.value();
		let reconciled = self.reconcile_transactions(index_value, availability.transactions());
		Ok(Confirmation::from_transactions(reconciled))
	}

	/// Builds a block header proposal from the mempool.
	///
	/// This is done with a reference to a confirmation proposal agreement.
	pub fn build_block_header_proposal(
		&mut self,
		index: &Index,
		confirmation: &Confirmation,
	) -> Result<BlockHeader, MempoolError> {
		// This does the same thing as build_confirmation_proposal,
		// the difference between the two is at the consensus condition level.
		let index_value = index.value();
		let reconciled = self.reconcile_transactions(index_value, confirmation.transactions());
		Ok(BlockHeader::from_transactions(reconciled))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use aegeri_message::{Message, Transaction, VerifiedMessage};
	use anyhow::Result;
	use ml_dsa::{MlDsa44, SigningKey, B32};

	fn tx(seed: u8, payload: Transaction, nonce: &[u8]) -> Result<VerifiedMessage<Transaction>> {
		let signer = SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![seed; 32]));
		let message =
			Message::<Transaction>::try_new(&signer, payload, aegeri_message::Nonce::new(nonce))?;
		Ok(message.into_verified()?)
	}

	fn tx_id(seed: u8, nonce: &[u8]) -> Result<Id> {
		Ok(*tx(seed, Transaction::Join, nonce)?.id())
	}

	#[test]
	fn test_does_not_pop_transactions_from_current_slot() -> Result<()> {
		let mut mempool = Mempool::new(100)?;
		let id = tx_id(1, b"a")?;
		mempool.insert_at(1000, id);
		assert!(mempool.pop_ready_at(1099, 10, IndexValue(0)).is_empty());
		assert!(mempool.pop_ready_at(1100, 10, IndexValue(0)).is_empty());
		assert_eq!(mempool.pop_ready_at(1200, 10, IndexValue(0)), vec![id]);
		Ok(())
	}

	#[test]
	fn test_pops_from_any_eligible_slot_less_than_current_minus_one() -> Result<()> {
		let mut mempool = Mempool::new(100)?;
		let older = tx_id(2, b"older")?;
		let previous = tx_id(3, b"previous")?;
		mempool.insert_at(850, older);
		mempool.insert_at(950, previous);
		assert_eq!(mempool.pop_ready_at(1000, 10, IndexValue(0)), vec![older]);
		assert_eq!(mempool.pop_ready_at(1100, 10, IndexValue(0)), vec![previous]);
		Ok(())
	}

	#[test]
	fn test_inflight_ids_are_not_selected_again() -> Result<()> {
		let mut mempool = Mempool::new(100)?;
		let index = Index::Availability(IndexValue(9));
		let tx_a = tx_id(9, b"a")?;
		let tx_b = tx_id(10, b"b")?;
		mempool.insert_at(700, tx_a);
		mempool.insert_at(700, tx_b);

		let availability = mempool.build_availability_proposal(1000, 1, &index)?;
		let first_selected = availability.transactions().ids().iter().copied().collect::<Vec<_>>();
		assert_eq!(first_selected.len(), 1);
		let selected_id = match first_selected.as_slice() {
			[selected] => *selected,
			_ => anyhow::bail!("expected exactly one selected id"),
		};

		let next = mempool.pop_ready_at(1000, 10, IndexValue(0));
		let expected = if tx_a == selected_id { vec![tx_b] } else { vec![tx_a] };
		assert_eq!(next, expected);
		Ok(())
	}

	#[test]
	fn test_build_availability_confirmation_and_block_header() -> Result<()> {
		let mut mempool = Mempool::new(100)?;
		let index = Index::Availability(IndexValue(7));
		let tx_a = tx_id(7, b"a")?;
		let tx_b = tx_id(8, b"b")?;
		mempool.insert_at(10, tx_a);
		mempool.insert_at(20, tx_b);

		let availability = mempool.build_availability_proposal(210, 100, &index)?;
		assert!(availability.transactions().ids().contains(&tx_a));
		assert!(availability.transactions().ids().contains(&tx_b));

		let confirmation = mempool.build_confirmation_proposal(&index, &availability)?;
		assert_eq!(confirmation.transactions().ids(), availability.transactions().ids());

		let block_header = mempool.build_block_header_proposal(&index, &confirmation)?;
		assert_eq!(block_header.transactions().ids(), confirmation.transactions().ids());
		Ok(())
	}

	#[test]
	fn test_build_block_header_returns_unconfirmed_ids_to_original_slot() -> Result<()> {
		let mut mempool = Mempool::new(100)?;
		let index = Index::Availability(IndexValue(11));
		let tx_a = tx_id(11, b"a")?;
		let tx_b = tx_id(12, b"b")?;
		mempool.insert_at(700, tx_a);
		mempool.insert_at(700, tx_b);

		let _availability = mempool.build_availability_proposal(1000, 100, &index)?;
		let mut only_a = aegeri_message::TransactionSet::new();
		only_a.add_id(tx_a);
		let confirmation = Confirmation::from_transactions(only_a);
		let _block_header = mempool.build_block_header_proposal(&index, &confirmation)?;

		let next = mempool.pop_ready_at(1000, 10, IndexValue(0));
		assert_eq!(next, vec![tx_b]);
		Ok(())
	}

	#[test]
	fn test_mark_inflight_tracks_id_in_both_maps() -> Result<()> {
		let mut mempool = Mempool::new(100)?;
		let id = tx_id(13, b"a")?;
		let index_value = IndexValue(2);

		mempool.mark_inflight(id, index_value);

		assert_eq!(mempool.inflight_by_id.get(&id), Some(&index_value));
		assert!(mempool.inflight_by_index.get(&index_value).is_some_and(|ids| ids.contains(&id)));
		Ok(())
	}

	#[test]
	fn test_mark_inflight_reassigns_index_membership() -> Result<()> {
		let mut mempool = Mempool::new(100)?;
		let id = tx_id(14, b"a")?;
		let first = IndexValue(3);
		let second = IndexValue(4);

		mempool.mark_inflight(id, first);
		mempool.mark_inflight(id, second);

		assert_eq!(mempool.inflight_by_id.get(&id), Some(&second));
		assert!(mempool.inflight_by_index.get(&first).is_none_or(|ids| !ids.contains(&id)));
		assert!(mempool.inflight_by_index.get(&second).is_some_and(|ids| ids.contains(&id)));
		Ok(())
	}

	#[test]
	fn test_unmark_inflight_removes_id_from_tracking() -> Result<()> {
		let mut mempool = Mempool::new(100)?;
		let id = tx_id(15, b"a")?;
		let index_value = IndexValue(5);
		mempool.mark_inflight(id, index_value);

		mempool.unmark_inflight(id);

		assert!(!mempool.inflight_by_id.contains_key(&id));
		assert!(mempool.inflight_by_index.get(&index_value).is_none_or(|ids| !ids.contains(&id)));
		Ok(())
	}

	#[test]
	fn test_remove_clears_slot_and_inflight_membership() -> Result<()> {
		let mut mempool = Mempool::new(100)?;
		let id = tx_id(16, b"a")?;
		let index_value = IndexValue(6);
		mempool.insert_at(700, id);
		mempool.mark_inflight(id, index_value);

		mempool.remove(id);

		assert!(mempool.by_slot.values().all(|ids| !ids.contains(&id)));
		assert!(!mempool.inflight_by_id.contains_key(&id));
		assert!(mempool.inflight_by_index.get(&index_value).is_none_or(|ids| !ids.contains(&id)));
		Ok(())
	}

	#[test]
	fn test_reconcile_transactions_filters_to_known_ids() -> Result<()> {
		let mut mempool = Mempool::new(100)?;
		let known = tx_id(17, b"known")?;
		let unknown = tx_id(18, b"unknown")?;
		let index_value = IndexValue(7);
		mempool.insert_at(700, known);

		let mut transactions = TransactionSet::new();
		transactions.add_id(known);
		transactions.add_id(unknown);

		let reconciled = mempool.reconcile_transactions(index_value, &transactions);

		assert_eq!(reconciled.ids(), &BTreeSet::from([known]));
		assert_eq!(mempool.inflight_by_id.get(&known), Some(&index_value));
		assert!(!mempool.inflight_by_id.contains_key(&unknown));
		Ok(())
	}

	#[test]
	fn test_reconcile_transactions_unmarks_stale_ids_only_for_target_index() -> Result<()> {
		let mut mempool = Mempool::new(100)?;
		let index_value = IndexValue(8);
		let other_index = IndexValue(9);
		let keep = tx_id(19, b"keep")?;
		let drop = tx_id(20, b"drop")?;
		let untouched = tx_id(21, b"untouched")?;

		mempool.insert_at(700, keep);
		mempool.insert_at(700, drop);
		mempool.insert_at(700, untouched);
		mempool.mark_inflight(keep, index_value);
		mempool.mark_inflight(drop, index_value);
		mempool.mark_inflight(untouched, other_index);

		let mut agreed = TransactionSet::new();
		agreed.add_id(keep);

		let reconciled = mempool.reconcile_transactions(index_value, &agreed);

		assert_eq!(reconciled.ids(), &BTreeSet::from([keep]));
		assert_eq!(mempool.inflight_by_id.get(&keep), Some(&index_value));
		assert!(!mempool.inflight_by_id.contains_key(&drop));
		assert_eq!(mempool.inflight_by_id.get(&untouched), Some(&other_index));
		assert_eq!(
			mempool.inflight_by_index.get(&index_value).cloned().unwrap_or_default(),
			HashSet::from([keep])
		);
		assert_eq!(
			mempool.inflight_by_index.get(&other_index).cloned().unwrap_or_default(),
			HashSet::from([untouched])
		);
		Ok(())
	}
}
