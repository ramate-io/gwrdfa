use crate::buffer::{Bufferlike, QueryPlanlike, Querylike};
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
	pub fn query<B: 'a, QueryPlan: QueryPlanlike<Entity, Buffer, B> + 'a>(
		&'a self,
		query_plan: QueryPlan,
	) -> impl Iterator<Item = (Entity, B)> + 'a {
		let mut query = query_plan.build(self.inner);
		core::iter::from_fn(move || query.next())
	}

	/// Gets a bundle from the facts in the buffer.
	pub fn get<B: 'a, QueryPlan: QueryPlanlike<Entity, Buffer, B> + 'a>(
		&self,
		entity: Entity,
		query_plan: QueryPlan,
	) -> Option<B> {
		let query = query_plan.build(self.inner);
		query.get(entity)
	}
}

impl<'a, Entity: Sized, Buffer: Bufferlike<Entity>> From<&'a Buffer> for Facts<'a, Entity, Buffer> {
	fn from(inner: &'a Buffer) -> Self {
		Facts::new(inner)
	}
}
