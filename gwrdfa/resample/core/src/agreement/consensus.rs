#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Condition<Value: Eq> {
	Consensus(Value),
	Hung,
	InProgress,
}

impl<Value: Eq> Condition<Value> {
	pub fn map<T: Eq>(self, f: impl FnOnce(Value) -> T) -> Condition<T> {
		match self {
			Condition::Consensus(value) => Condition::Consensus(f(value)),
			Condition::Hung => Condition::Hung,
			Condition::InProgress => Condition::InProgress,
		}
	}
}
