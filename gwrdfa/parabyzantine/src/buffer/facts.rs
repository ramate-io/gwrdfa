use crate::buffer::{Bufferlike, Bundle, Querylike};
use core::marker::PhantomData;

/// Facts are entities that exist at a particular snapshot of the buffer.
#[derive(Debug, Copy, Clone)]
pub struct Facts<'a, Entity: Sized, Buffer: Bufferlike<Entity>> {
	inner: &'a Buffer,
	__phantom: PhantomData<Entity>,
}

impl<'a, Entity: Sized, Buffer: Bufferlike<Entity>> Facts<'a, Entity, Buffer> {
	pub fn new(inner: &'a Buffer) -> Self {
		Self { inner, __phantom: PhantomData }
	}

	/// Queries the facts in the buffer.
	pub fn query<B: Bundle + 'a, Query: Querylike<Entity, Buffer, B> + 'a>(
		&'a self,
		mut query: Query,
	) -> impl Iterator<Item = (Entity, B)> + 'a {
		core::iter::from_fn(move || query.next(self.inner))
	}

	/// Gets a bundle from the facts in the buffer.
	pub fn get<B: Bundle, Query: Querylike<Entity, Buffer, B>>(
		&self,
		entity: Entity,
		query: Query,
	) -> Option<B> {
		query.get(self.inner, entity)
	}
}

impl<'a, Entity: Sized, Buffer: Bufferlike<Entity>> From<&'a Buffer> for Facts<'a, Entity, Buffer> {
	fn from(inner: &'a Buffer) -> Self {
		Facts::new(inner)
	}
}
