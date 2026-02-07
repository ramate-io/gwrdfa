pub trait Chunklike {}

/// Chunks represent levels of detail for messages in the system.
///
/// For example, an index of a certificates may be loadable up to a number of chunks.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Chunk<C: Chunklike> {
	inner: C,
}

impl<C: Chunklike> Chunk<C> {
	pub fn new(inner: C) -> Self {
		Self { inner }
	}

	pub fn inner(&self) -> &C {
		&self.inner
	}
}
