use aegeri_message::{Availability, BlockHeader, Confirmation, Id, Index};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Slot(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct IndexValue(u64);

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
	/// Ordered backing storage used for candidate selection.
	by_slot: BTreeMap<Slot, BTreeSet<Id>>,
	/// In-flight ownership by consensus index and original slot.
	inflight: HashMap<Id, (IndexValue, Slot)>,
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
		Ok(Self {
			slot_width_ms,
			by_slot: BTreeMap::new(),
			inflight: HashMap::new(),
		})
	}

	pub fn slot_width_ms(&self) -> u64 {
		self.slot_width_ms
	}

	fn slot_for_epoch_ms(&self, epoch_ms: u64) -> Slot {
		Slot(epoch_ms / self.slot_width_ms)
	}

	fn index_value(index: &Index) -> IndexValue {
		IndexValue(index.value())
	}

	pub fn has_id(&self, id: &Id) -> bool {
		self.inflight.contains_key(id)
			|| self.by_slot.values().any(|ids| ids.contains(id))
	}

	fn insert_entry(&mut self, received_at_epoch_ms: u64, id: Id) {
		if self.has_id(&id) {
			return;
		}
		let slot = self.slot_for_epoch_ms(received_at_epoch_ms);
		self.by_slot.entry(slot).or_default().insert(id);
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

	/// Pops up to `max_items` transactions from any eligible slot `< t - 1`.
	///
	/// Pop priority:
	/// 1. Newer eligible slots first.
	/// 2. Within each slot, larger `Id` values first.
	fn pop_ready_ids_at(&mut self, now_epoch_ms: u64, max_items: usize) -> Vec<(Id, Slot)> {
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

		let mut selected: Vec<(Id, Slot)> = Vec::new();
		for slot in eligible_slots.into_iter().rev() {
			let Some(ids) = self.by_slot.get_mut(&slot) else {
				continue;
			};
			while selected.len() < max_items {
				let Some(id) = ids.pop_last() else {
					break;
				};
				if self.inflight.contains_key(&id) {
					continue;
				}
				selected.push((id, slot));
			}
			if ids.is_empty() {
				self.by_slot.remove(&slot);
			}
			if selected.len() >= max_items {
				break;
			}
		}

		selected
	}

	fn take_id_from_pool(&mut self, id: Id) -> Option<Slot> {
		let slot = self.by_slot.iter().find_map(|(slot, ids)| {
			if ids.contains(&id) {
				Some(*slot)
			} else {
				None
			}
		})?;
		let ids = self.by_slot.get_mut(&slot)?;
		ids.remove(&id);
		if ids.is_empty() {
			self.by_slot.remove(&slot);
		}
		Some(slot)
	}

	pub fn pop_ready_at(&mut self, now_epoch_ms: u64, max_items: usize) -> Vec<Id> {
		self.pop_ready_ids_at(now_epoch_ms, max_items)
			.into_iter()
			.map(|(id, _)| id)
			.collect()
	}

	pub fn pop_ready_now(&mut self, max_items: usize) -> Result<Vec<Id>, MempoolError> {
		let now_ms = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.map_err(|_| MempoolError::SystemTimeBeforeEpoch)?
			.as_millis() as u64;
		Ok(self.pop_ready_at(now_ms, max_items))
	}

	pub fn build_availability_proposal(
		&mut self,
		now_epoch_ms: u64,
		max_items: usize,
		index: &Index,
	) -> Result<Availability, MempoolError> {
		let index_value = Self::index_value(index);
		let selected_ids = self.pop_ready_ids_at(now_epoch_ms, max_items);
		let mut ids = aegeri_message::TransactionSet::new();
		for (id, slot) in selected_ids {
			ids.add_id(id);
			self.inflight.insert(id, (index_value, slot));
		}
		Ok(Availability::from_transactions(ids))
	}

	pub fn build_confirmation_proposal(
		&mut self,
		index: &Index,
		availability: &Availability,
	) -> Result<Confirmation, MempoolError> {
		let index_value = Self::index_value(index);
		let mut confirmed = aegeri_message::TransactionSet::new();

		for id in availability.transactions().iter_ids() {
			match self.inflight.get(id) {
				Some((existing, _)) if *existing != index_value => continue,
				_ => {
					if let Some((_, slot)) = self.inflight.get(id).copied() {
						self.inflight.insert(*id, (index_value, slot));
						confirmed.add_id(*id);
						continue;
					}
					if let Some(slot) = self.take_id_from_pool(*id) {
						self.inflight.insert(*id, (index_value, slot));
						confirmed.add_id(*id);
					}
				}
			}
		}

		Ok(Confirmation::from_transactions(confirmed))
	}

	pub fn build_block_header_proposal(
		&mut self,
		index: &Index,
		confirmation: &Confirmation,
	) -> Result<BlockHeader, MempoolError> {
		let index_value = Self::index_value(index);
		let confirmed_ids = confirmation.transactions().ids().clone();

		let mut to_return = Vec::new();
		for (id, (mapped_index, slot)) in &self.inflight {
			if *mapped_index == index_value && !confirmed_ids.contains(id) {
				to_return.push((*id, *slot));
			}
		}

		for (id, slot) in to_return {
			self.inflight.remove(&id);
			self.by_slot.entry(slot).or_default().insert(id);
		}

		for id in confirmation.transactions().iter_ids() {
			match self.inflight.get(id).copied() {
				Some((_, slot)) => {
					self.inflight.insert(*id, (index_value, slot));
				}
				None => {
					if let Some(slot) = self.take_id_from_pool(*id) {
						self.inflight.insert(*id, (index_value, slot));
					}
				}
			}
		}

		Ok(BlockHeader::from_transactions(confirmation.transactions().clone()))
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
		assert!(mempool.pop_ready_at(1099, 10).is_empty());
		assert!(mempool.pop_ready_at(1100, 10).is_empty());
		assert_eq!(mempool.pop_ready_at(1200, 10), vec![id]);
		Ok(())
	}

	#[test]
	fn test_pops_from_any_eligible_slot_less_than_current_minus_one() -> Result<()> {
		let mut mempool = Mempool::new(100)?;
		let older = tx_id(2, b"older")?;
		let previous = tx_id(3, b"previous")?;
		mempool.insert_at(850, older);
		mempool.insert_at(950, previous);
		assert_eq!(mempool.pop_ready_at(1000, 10), vec![older]);
		assert_eq!(mempool.pop_ready_at(1100, 10), vec![previous]);
		Ok(())
	}

	#[test]
	fn test_inflight_ids_are_not_selected_again() -> Result<()> {
		let mut mempool = Mempool::new(100)?;
		let index = Index::Availability(9);
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

		let next = mempool.pop_ready_at(1000, 10);
		let expected = if tx_a == selected_id { vec![tx_b] } else { vec![tx_a] };
		assert_eq!(next, expected);
		Ok(())
	}

	#[test]
	fn test_build_availability_confirmation_and_block_header() -> Result<()> {
		let mut mempool = Mempool::new(100)?;
		let index = Index::Availability(7);
		let tx_a = tx_id(7, b"a")?;
		let tx_b = tx_id(8, b"b")?;
		mempool.insert_at(10, tx_a);
		mempool.insert_at(20, tx_b);

		let availability = mempool.build_availability_proposal(30, 100, &index)?;
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
		let index = Index::Availability(11);
		let tx_a = tx_id(11, b"a")?;
		let tx_b = tx_id(12, b"b")?;
		mempool.insert_at(700, tx_a);
		mempool.insert_at(700, tx_b);

		let _availability = mempool.build_availability_proposal(1000, 100, &index)?;
		let mut only_a = aegeri_message::TransactionSet::new();
		only_a.add_id(tx_a);
		let confirmation = Confirmation::from_transactions(only_a);
		let _block_header = mempool.build_block_header_proposal(&index, &confirmation)?;

		let next = mempool.pop_ready_at(1000, 10);
		assert_eq!(next, vec![tx_b]);
		Ok(())
	}
}
