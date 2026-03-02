#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Condition<Value: Eq> {
	Consensus(Value),
	Hung,
	InProgress,
}
