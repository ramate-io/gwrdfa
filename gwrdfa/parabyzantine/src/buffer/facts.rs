use crate::buffer::{Bufferlike, Bundle};

/// Facts are entities that exist at a particular snapshot of the buffer.
#[derive(Debug, Copy, Clone)]
pub struct Facts<'a, F: Bufferlike> {
	inner: &'a F,
}

impl<'a, F: Bufferlike> Facts<'a, F> {
	pub fn new(inner: &'a F) -> Self {
		Self { inner }
	}

	/// Queries the facts in the buffer.
	pub fn query<B: Bundle<F::Entity> + 'a>(&'a self) -> impl Iterator<Item = B> + 'a {
		self.inner.query()
	}
}
