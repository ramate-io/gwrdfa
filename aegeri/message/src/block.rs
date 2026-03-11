use crate::{Id, Transaction, VerifiedMessage};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// Deterministic block representation.
///
/// Using a set makes block content order-independent with respect to mempool
/// insertion timing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Block(BTreeSet<VerifiedMessage<Transaction>>);

impl Block {
	pub fn new(transactions: impl IntoIterator<Item = VerifiedMessage<Transaction>>) -> Self {
		Self(transactions.into_iter().collect())
	}

	pub fn transactions(&self) -> impl Iterator<Item = &VerifiedMessage<Transaction>> {
		self.0.iter()
	}
}

/// Transaction identifier set used by proposal layers.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct TransactionSet(BTreeSet<Id>);

impl TransactionSet {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn ids(&self) -> &BTreeSet<Id> {
		&self.0
	}

	pub fn iter_ids(&self) -> impl Iterator<Item = &Id> {
		self.0.iter()
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn add_id(&mut self, id: Id) {
		self.0.insert(id);
	}

	pub fn intersection<'a>(&'a self, other: &'a TransactionSet) -> BTreeSet<&'a Id> {
		self.0.intersection(&other.0).collect()
	}

	pub fn intersect_all<'a>(
		mut iter: impl Iterator<Item = &'a TransactionSet>,
	) -> BTreeSet<&'a Id> {
		let Some(first) = iter.next() else {
			return BTreeSet::new();
		};
		let mut acc = first.0.iter().collect::<BTreeSet<&Id>>();
		for set in iter {
			let rhs = set.0.iter().collect::<BTreeSet<&Id>>();
			acc = acc.intersection(&rhs).copied().collect();
		}
		acc
	}
}
