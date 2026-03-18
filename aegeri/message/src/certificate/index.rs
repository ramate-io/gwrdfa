use gwrdfa_resample::agreement::std::NextRound;
use serde::{Deserialize, Serialize};

/// The value of the index in the consensus pipeline.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexValue(pub u64);

impl IndexValue {
	pub fn new(value: u64) -> Self {
		Self(value)
	}

	pub fn genesis() -> Self {
		Self(0)
	}
}

/// Index of a certificate round in the layered consensus pipeline.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Index {
	Availability(IndexValue),
	Confirmation(IndexValue),
	Block(IndexValue),
	Transition(IndexValue),
	Unassigned,
}

impl NextRound for Index {
	fn next(&self) -> Option<Self> {
		match self {
			Index::Availability(index) => Some(Index::Confirmation(*index)),
			Index::Confirmation(index) => Some(Index::Block(*index)),
			Index::Block(index) => Some(Index::Transition(*index)),
			Index::Transition(index) => Some(Index::Availability(IndexValue(index.0 + 1))),
			Index::Unassigned => None,
		}
	}
}

impl Index {
	pub fn genesis() -> Self {
		Index::Availability(IndexValue::genesis())
	}

	pub fn is_availability(&self) -> bool {
		matches!(self, Index::Availability(_))
	}

	pub fn is_transition(&self) -> bool {
		matches!(self, Index::Transition(_))
	}

	pub fn is_confirmation(&self) -> bool {
		matches!(self, Index::Confirmation(_))
	}

	pub fn is_block(&self) -> bool {
		matches!(self, Index::Block(_))
	}

	pub fn value(&self) -> IndexValue {
		match self {
			Index::Availability(index)
			| Index::Confirmation(index)
			| Index::Block(index)
			| Index::Transition(index) => *index,
			Index::Unassigned => IndexValue(0),
		}
	}
}
