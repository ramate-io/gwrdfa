use aegeri_message::{
	Availability, Block, BlockHeader, Confirmation, Id, Index, Message, Transaction, VerifiedMessage,
};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Slot(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct IndexValue(u64);

/// Slot-bucketed mempool for verified Aegeri transactions.
///
/// Storage model (no duplicated transaction payloads):
/// - `by_id`: `Id -> VerifiedMessage<Transaction>`
/// - `by_slot`: `Slot -> ordered set of Id`
/// - `inflight`: `IndexValue -> (Id -> (message, original_timestamp))`
pub struct Mempool {
	slot_width_ms: u64,
	by_id: HashMap<Id, VerifiedMessage<Transaction>>,
	by_slot: BTreeMap<Slot, BTreeSet<Id>>,
	inflight: HashMap<IndexValue, HashMap<Id, (VerifiedMessage<Transaction>, u64)>>,
}

#[derive(Debug, thiserror::Error)]
pub enum MempoolError {
	#[error("slot_width_ms must be greater than zero")]
	InvalidSlotWidth,
	#[error("system time is before UNIX_EPOCH")]
	SystemTimeBeforeEpoch,
	#[error("missing in-flight index {0}")]
	MissingInflightIndex(u64),
	#[error("missing in-flight transaction id {0:?}")]
	MissingInflightTransaction(Id),
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

	pub fn insert_now(&mut self, message: VerifiedMessage<Transaction>) -> Result<(), MempoolError> {
		let now_ms = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.map_err(|_| MempoolError::SystemTimeBeforeEpoch)?
			.as_millis() as u64;
		self.insert_at(now_ms, message);
		Ok(())
	}

	fn pop_id_from_slot(&mut self, slot: Slot) -> Option<Id> {
		let ids = self.by_slot.get_mut(&slot)?;
		let id = ids.pop_last()?;
		if ids.is_empty() {
			self.by_slot.remove(&slot);
		}
		Some(id)
	}

	fn remove_by_id_from_slot_index(&mut self, id: &Id) -> Option<Slot> {
		let containing_slot = self.by_slot.iter().find_map(|(slot, ids)| {
			if ids.contains(id) {
				Some(*slot)
			} else {
				None
			}
		});
		if let Some(slot) = containing_slot {
			if let Some(ids) = self.by_slot.get_mut(&slot) {
				ids.remove(id);
				if ids.is_empty() {
					self.by_slot.remove(&slot);
				}
			}
		}
		containing_slot
	}

	fn take_from_pool_by_id(
		&mut self,
		id: &Id,
	) -> Option<(VerifiedMessage<Transaction>, u64)> {
		let message = self.by_id.remove(id)?;
		let slot = self.remove_by_id_from_slot_index(id)?;
		let original_timestamp = slot.0.saturating_mul(self.slot_width_ms);
		Some((message, original_timestamp))
	}

	/// Pops up to `max_items` transactions from any eligible slot `< t - 1`.
	///
	/// Pop priority:
	/// 1. Newer eligible slots first.
	/// 2. Within each slot, larger `Id` values first.
	fn pop_ready_entries_at(
		&mut self,
		now_epoch_ms: u64,
		max_items: usize,
	) -> Vec<(Id, VerifiedMessage<Transaction>, u64)> {
		if max_items == 0 {
			return Vec::new();
		}

		let current_slot = self.slot_for_epoch_ms(now_epoch_ms);
		if current_slot.0 <= 1 {
			return Vec::new();
		}
		let upper_exclusive = Slot(current_slot.0 - 1);
		let eligible_slots = self
			.by_slot
			.range(..upper_exclusive)
			.map(|(slot, _)| *slot)
			.collect::<Vec<_>>();

		let mut popped = Vec::new();
		for slot in eligible_slots.into_iter().rev() {
			while popped.len() < max_items {
				let Some(id) = self.pop_id_from_slot(slot) else {
					break;
				};
				let Some(message) = self.by_id.remove(&id) else {
					continue;
				};
				// "Original timestamp" is reconstructed from slot start since pool
				// stores slot index, not per-id arrival millis.
				let original_timestamp = slot.0.saturating_mul(self.slot_width_ms);
				popped.push((id, message, original_timestamp));
			}
			if popped.len() >= max_items {
				break;
			}
		}
		popped
	}

	pub fn pop_ready_at(
		&mut self,
		now_epoch_ms: u64,
		max_items: usize,
	) -> Vec<VerifiedMessage<Transaction>> {
		self.pop_ready_entries_at(now_epoch_ms, max_items)
			.into_iter()
			.map(|(_, message, _)| message)
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

	pub fn build_availability_proposal(&mut self, index: &Index) -> Result<Availability, MempoolError> {
		let index_value = Self::index_value(index);
		let popped = self.pop_ready_entries_at(u64::MAX, usize::MAX);
		let inflight_for_index = self.inflight.entry(index_value).or_default();
		let mut ids = aegeri_message::TransactionSet::new();
		for (id, message, original_timestamp) in popped {
			ids.add_id(id);
			inflight_for_index.insert(id, (message, original_timestamp));
		}
		Ok(Availability::from_transactions(ids))
	}

	pub fn build_confirmation_proposal(
		&mut self,
		index: &Index,
		availability: &Availability,
	) -> Result<Confirmation, MempoolError> {
		let index_value = Self::index_value(index);
		let inflight_for_index = self.inflight.entry(index_value).or_default();
		let mut confirmed = aegeri_message::TransactionSet::new();

		for id in availability.transactions().iter_ids() {
			if inflight_for_index.contains_key(id) {
				confirmed.add_id(*id);
				continue;
			}
			if let Some((message, original_timestamp)) = self.take_from_pool_by_id(id) {
				inflight_for_index.insert(*id, (message, original_timestamp));
				confirmed.add_id(*id);
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
		let inflight_for_index = self.inflight.entry(index_value).or_default();
		let confirmed_ids = confirmation.transactions().ids().clone();
		let inflight_ids = inflight_for_index.keys().copied().collect::<Vec<_>>();
		for id in inflight_ids {
			if confirmed_ids.contains(&id) {
				continue;
			}
			if let Some((message, original_timestamp)) = inflight_for_index.remove(&id) {
				self.insert_entry(original_timestamp, message);
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
		let Some(inflight_for_index) = self.inflight.get(&index_value) else {
			return Err(MempoolError::MissingInflightIndex(index_value.0));
		};
		for id in block_header.transactions().iter_ids() {
			if !inflight_for_index.contains_key(id) {
				return Err(MempoolError::MissingInflightTransaction(*id));
			}
		}

		let mut block_transactions = Vec::new();
		let inflight_for_index = self
			.inflight
			.get_mut(&index_value)
			.ok_or(MempoolError::MissingInflightIndex(index_value.0))?;
		for id in block_header.transactions().iter_ids() {
			let (message, _) = inflight_for_index
				.remove(id)
				.ok_or(MempoolError::MissingInflightTransaction(*id))?;
			block_transactions.push(message);
		}
		if inflight_for_index.is_empty() {
			self.inflight.remove(&index_value);
		}
		Ok(Block::new(block_transactions))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use anyhow::Result;
	use ml_dsa::{B32, MlDsa44, SigningKey};

	fn tx(seed: u8, payload: Transaction, nonce: &[u8]) -> Result<VerifiedMessage<Transaction>> {
		let signer = SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![seed; 32]));
		let message = Message::<Transaction>::try_new(
			&signer,
			payload,
			aegeri_message::Nonce::new(nonce),
		)?;
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
		assert_eq!(mempool.pop_ready_at(1000, 10), vec![older]);
		assert_eq!(mempool.pop_ready_at(1100, 10), vec![previous]);
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

		let availability = mempool.build_availability_proposal(&index)?;
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
