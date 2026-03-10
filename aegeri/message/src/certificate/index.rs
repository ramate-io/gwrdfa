use gwrdfa_resample::agreement::std::NextRound;
use serde::{Deserialize, Serialize};

/// Index of a certificate round in the layered consensus pipeline.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Index {
	Availability(u64),
	Confirmation(u64),
	Block(u64),
	Transition(u64),
}

impl NextRound for Index {
	fn next(&self) -> Option<Self> {
		match self {
			Index::Availability(index) => Some(Index::Confirmation(*index)),
			Index::Confirmation(index) => Some(Index::Block(*index)),
			Index::Block(index) => Some(Index::Transition(*index)),
			Index::Transition(index) => Some(Index::Availability(index + 1)),
		}
	}
}

impl Index {
	pub fn is_availability(&self) -> bool {
		matches!(self, Index::Availability(_))
	}

	pub fn is_transition(&self) -> bool {
		matches!(self, Index::Transition(_))
	}

	pub fn value(&self) -> u64 {
		match self {
			Index::Availability(index)
			| Index::Confirmation(index)
			| Index::Block(index)
			| Index::Transition(index) => *index,
		}
	}
}
