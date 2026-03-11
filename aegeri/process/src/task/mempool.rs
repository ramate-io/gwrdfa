use aegeri_message::{
	Availability, Block, BlockHeader, Confirmation, Id, Index, Transaction, VerifiedMessage,
};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Slot(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct IndexValue(u64);

/// Slot-bucketed mempool for verified Aegeri transactions.
///
/// Design notes:
/// - `by_id` is the backing storage and O(1) lookup index.
/// - `by_slot` provides a deterministic scheduling order over IDs.
/// - `inflight` maps transaction IDs to a consensus index.
///   Internal methods must avoid selecting IDs already mapped in-flight.
/// - `by_slot` cleanup is lazy: stale IDs are dropped when discovered.
pub struct Mempool {
	slot_width_ms: u64,
	/// Backing storage.
	by_id: HashMap<Id, VerifiedMessage<Transaction>>,
	/// Ordered view used to pick candidates by age and deterministic ID order.
	by_slot: BTreeMap<Slot, BTreeSet<Id>>,
	/// In-flight ownership by consensus index.
	inflight: HashMap<Id, IndexValue>,
}

#[derive(Debug, thiserror::Error)]
pub enum MempoolError {
	#[error("slot_width_ms must be greater than zero")]
	InvalidSlotWidth,
	#[error("system time is before UNIX_EPOCH")]
	SystemTimeBeforeEpoch,
	#[error("missing transaction id {0:?} for block reification")]
	MissingTransaction(Id),
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
		self.by_id.contains_key(id)
	}

	fn insert_entry(&mut self, received_at_epoch_ms: u64, message: VerifiedMessage<Transaction>) {
		let id = *message.id();
		if self.by_id.contains_key(&id) {
			return;
		}
		let slot = self.slot_for_epoch_ms(received_at_epoch_ms);
		self.by_id.insert(id, message);
		self.by_slot.entry(slot).or_default().insert(id);
	}

	pub fn insert_at(&mut self, received_at_epoch_ms: u64, message: VerifiedMessage<Transaction>) {
		self.insert_entry(received_at_epoch_ms, message);
	}

	pub fn insert_now(
		&mut self,
		message: VerifiedMessage<Transaction>,
	) -> Result<(), MempoolError> {
		let now_ms = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.map_err(|_| MempoolError::SystemTimeBeforeEpoch)?
			.as_millis() as u64;
		self.insert_at(now_ms, message);
		Ok(())
	}

	/// Pops up to `max_items` transactions from any eligible slot `< t - 1`.
	///
	/// Pop priority:
	/// 1. Newer eligible slots first.
	/// 2. Within each slot, larger `Id` values first.
	fn pop_ready_ids_at(&mut self, now_epoch_ms: u64, max_items: usize) -> Vec<Id> {
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

		let mut selected = Vec::new();
		let mut stale = Vec::new();
		for slot in eligible_slots.into_iter().rev() {
			let Some(ids) = self.by_slot.get(&slot) else {
				continue;
			};
			for id in ids.iter().rev().copied() {
				if selected.len() >= max_items {
					break;
				}
				if !self.by_id.contains_key(&id) {
					stale.push((slot, id));
					continue;
				}
				if self.inflight.contains_key(&id) {
					continue;
				}
				selected.push(id);
			}
			if selected.len() >= max_items {
				break;
			}
		}

		for (slot, id) in stale {
			if let Some(ids) = self.by_slot.get_mut(&slot) {
				ids.remove(&id);
				if ids.is_empty() {
					self.by_slot.remove(&slot);
				}
			}
		}

		selected
	}

	pub fn pop_ready_at(
		&mut self,
		now_epoch_ms: u64,
		max_items: usize,
	) -> Vec<VerifiedMessage<Transaction>> {
		self.pop_ready_ids_at(now_epoch_ms, max_items)
			.into_iter()
			.filter_map(|id| self.by_id.get(&id).cloned())
			.collect()
	}

	pub fn pop_ready_now(
		&mut self,
		max_items: usize,
	) -> Result<Vec<VerifiedMessage<Transaction>>, MempoolError> {
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
		for id in selected_ids {
			ids.add_id(id);
			self.inflight.insert(id, index_value);
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
			if !self.by_id.contains_key(id) {
				continue;
			}
			match self.inflight.get(id) {
				Some(existing) if *existing != index_value => continue,
				_ => {
					self.inflight.insert(*id, index_value);
					confirmed.add_id(*id);
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

		// Keep only confirmed IDs at this index, and free the rest back to "available".
		let ids_for_index = self
			.inflight
			.iter()
			.filter_map(|(id, mapped)| if *mapped == index_value { Some(*id) } else { None })
			.collect::<Vec<_>>();
		for id in ids_for_index {
			if !confirmed_ids.contains(&id) {
				self.inflight.remove(&id);
			}
		}

		// If confirmation includes IDs we still have, map them in-flight to this index.
		for id in confirmation.transactions().iter_ids() {
			if self.by_id.contains_key(id) {
				self.inflight.insert(*id, index_value);
			}
		}

		Ok(BlockHeader::from_transactions(confirmation.transactions().clone()))
	}

	pub fn reify_block_header(
		&mut self,
		index: &Index,
		block_header: &BlockHeader,
	) -> Result<Block, MempoolError> {
		let index_value = Self::index_value(index);
		let mut block_transactions = Vec::new();
		for id in block_header.transactions().iter_ids() {
			let message =
				self.by_id.get(id).cloned().ok_or(MempoolError::MissingTransaction(*id))?;
			self.inflight.insert(*id, index_value);
			block_transactions.push(message);
		}
		Ok(Block::new(block_transactions))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use aegeri_message::Message;
	use anyhow::Result;
	use ml_dsa::{MlDsa44, SigningKey, B32};

	fn tx(seed: u8, payload: Transaction, nonce: &[u8]) -> Result<VerifiedMessage<Transaction>> {
		let signer = SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![seed; 32]));
		let message =
			Message::<Transaction>::try_new(&signer, payload, aegeri_message::Nonce::new(nonce))?;
		Ok(message.into_verified()?)
	}

	#[test]
	fn test_does_not_pop_transactions_from_current_slot() -> Result<()> {
		let mut mempool = Mempool::new(100)?;
		let message = tx(1, Transaction::Join, b"a")?;
		mempool.insert_at(1000, message.clone());
		assert!(mempool.pop_ready_at(1099, 10).is_empty());
		assert!(mempool.pop_ready_at(1100, 10).is_empty());
		assert_eq!(mempool.pop_ready_at(1200, 10), vec![message]);
		Ok(())
	}

	#[test]
	fn test_pops_from_any_eligible_slot_less_than_current_minus_one() -> Result<()> {
		let mut mempool = Mempool::new(100)?;
		let older = tx(2, Transaction::Join, b"older")?;
		let previous = tx(3, Transaction::Join, b"previous")?;
		mempool.insert_at(850, older.clone());
		mempool.insert_at(950, previous.clone());
		assert_eq!(mempool.pop_ready_at(1000, 10), vec![older.clone()]);
		assert_eq!(mempool.pop_ready_at(1100, 10), vec![previous, older]);
		Ok(())
	}

	#[test]
	fn test_inflight_ids_are_not_selected_again() -> Result<()> {
		let mut mempool = Mempool::new(100)?;
		let index = Index::Availability(9);
		let tx_a = tx(9, Transaction::Join, b"a")?;
		let tx_b = tx(10, Transaction::Join, b"b")?;
		mempool.insert_at(700, tx_a.clone());
		mempool.insert_at(700, tx_b.clone());

		let availability = mempool.build_availability_proposal(1000, 1, &index)?;
		let first_selected = availability.transactions().ids().iter().copied().collect::<Vec<_>>();
		assert_eq!(first_selected.len(), 1);
		let selected_id = match first_selected.as_slice() {
			[selected] => *selected,
			_ => anyhow::bail!("expected exactly one selected id"),
		};

		let next = mempool.pop_ready_at(1000, 10);
		let expected = if *tx_a.id() == selected_id { vec![tx_b] } else { vec![tx_a] };
		assert_eq!(next, expected);
		Ok(())
	}

	#[test]
	fn test_build_availability_confirmation_block_header_and_reify() -> Result<()> {
		let mut mempool = Mempool::new(100)?;
		let index = Index::Availability(7);
		let tx_a = tx(7, Transaction::Join, b"a")?;
		let tx_b = tx(8, Transaction::Join, b"b")?;
		mempool.insert_at(10, tx_a.clone());
		mempool.insert_at(20, tx_b.clone());

		let availability = mempool.build_availability_proposal(30, 100, &index)?;
		assert!(availability.transactions().ids().contains(tx_a.id()));
		assert!(availability.transactions().ids().contains(tx_b.id()));

		let confirmation = mempool.build_confirmation_proposal(&index, &availability)?;
		assert_eq!(confirmation.transactions().ids(), availability.transactions().ids());

		let block_header = mempool.build_block_header_proposal(&index, &confirmation)?;
		assert_eq!(block_header.transactions().ids(), confirmation.transactions().ids());

		let block = mempool.reify_block_header(&index, &block_header)?;
		let block_messages = block.transactions().cloned().collect::<BTreeSet<_>>();
		assert_eq!(block_messages, BTreeSet::from([tx_a, tx_b]));
		Ok(())
	}
}
