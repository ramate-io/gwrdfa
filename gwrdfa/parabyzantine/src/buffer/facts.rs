use crate::buffer::{Bufferlike, Bundle, Indexlike, Querylike};
use core::marker::PhantomData;

/// Facts are entities that exist at a particular snapshot of the buffer.
#[derive(Debug, Copy, Clone)]
pub struct Facts<'a, Entity: Sized, F: Bufferlike<Entity>> {
	inner: &'a F,
	__phantom: PhantomData<Entity>,
}

impl<'a, Entity: Sized, F: Bufferlike<Entity>> Facts<'a, Entity, F> {
	pub fn new(inner: &'a F) -> Self {
		Self { inner, __phantom: PhantomData }
	}

	/// Queries the facts in the buffer.
	pub fn query<
		B: Bundle<Entity> + 'a,
		Query: Querylike<Entity, F, B> + 'a,
		Index: Indexlike<Entity, F, B, Query> + 'a,
	>(
		&'a self,
		index: Index,
	) -> Query {
		index.query(self.inner)
	}
}
