//! Consensus-state primitives used by resample agreement.

/// Agreement condition over a candidate value.
///
/// - `Consensus(v)`: enough support exists to finalize `v`.
/// - `Hung`: enough conflicting/insufficient information exists to prevent
///   progress under current evidence.
/// - `InProgress`: more evidence may still produce consensus.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Condition<Value: Eq> {
	Consensus(Value),
	Hung,
	InProgress,
}

impl<Value: Eq> Condition<Value> {
	/// Maps the consensus payload while preserving condition shape.
	pub fn map<T: Eq>(self, f: impl FnOnce(Value) -> T) -> Condition<T> {
		match self {
			Condition::Consensus(value) => Condition::Consensus(f(value)),
			Condition::Hung => Condition::Hung,
			Condition::InProgress => Condition::InProgress,
		}
	}
}
