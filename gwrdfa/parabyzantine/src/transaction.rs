pub trait Transactionlike {}

/// Transactions are messages that originate outside the system.
/// That is, from the outside world.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Transaction<T: Transactionlike> {
	inner: T,
}

impl<T: Transactionlike> Transaction<T> {
	pub fn new(inner: T) -> Self {
		Self { inner }
	}

	pub fn inner(&self) -> &T {
		&self.inner
	}
}
