pub trait Indexlike {}

/// Indexes (indices) identify where a message is said to originate in the system.
///
/// It is precisely the job of most Parabzantine protocols to map messages to indices.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Index<I: Indexlike> {
	inner: I,
}

impl<I: Indexlike> Index<I> {
	pub fn new(inner: I) -> Self {
		Self { inner }
	}

	pub fn inner(&self) -> &I {
		&self.inner
	}
}
