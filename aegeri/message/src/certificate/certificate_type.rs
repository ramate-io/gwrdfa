use super::{Index, Proposal};
use serde::{Deserialize, Serialize};

/// Signed certificate payload.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Certificate {
	index: Index,
	value: Proposal,
}

impl Certificate {
	pub fn new(index: Index, value: Proposal) -> Self {
		Self { index, value }
	}

	pub fn index(&self) -> &Index {
		&self.index
	}

	pub fn value(&self) -> &Proposal {
		&self.value
	}
}
