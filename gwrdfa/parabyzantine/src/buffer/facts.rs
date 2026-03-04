use crate::buffer::{
	query::{QueryPlanlike, Querylike},
	Bufferlike,
};
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
	pub fn query<
		Item,
		Query: Querylike<Entity, Item> + 'a,
		QueryPlan: QueryPlanlike<Entity, &'a Buffer, Item, Query>,
	>(
		&self,
		query_plan: QueryPlan,
	) -> impl Iterator<Item = (Entity, Item)> + 'a {
		let mut q = query_plan.into_query(self.inner);
		core::iter::from_fn(move || q.next())
	}

	/// Gets a bundle from the facts in the buffer.
	pub fn get<
		Item,
		Query: Querylike<Entity, Item> + 'a,
		QueryPlan: QueryPlanlike<Entity, &'a Buffer, Item, Query> + 'a,
	>(
		&self,
		entity: Entity,
		query_plan: QueryPlan,
	) -> Option<Item> {
		let q = query_plan.into_query(self.inner);
		q.get(entity)
	}
}

impl<'a, Entity: Sized, Buffer: Bufferlike<Entity>> From<&'a Buffer> for Facts<'a, Entity, Buffer> {
	fn from(inner: &'a Buffer) -> Self {
		Facts::new(inner)
	}
}
