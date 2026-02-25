use crate::buffer::{
	query::{QueryPlanlike, Querylike},
	Bufferlike,
};
use core::marker::PhantomData;

/// Facts are entities that exist at a particular snapshot of the buffer.
#[derive(Debug)]
pub struct Facts<'a, Entity: Sized, Buffer: Bufferlike<Entity>> {
	inner: &'a mut Buffer,
	__phantom: PhantomData<Entity>,
}

impl<'a, Entity: Sized, Buffer: Bufferlike<Entity>> Facts<'a, Entity, Buffer> {
	pub fn new(inner: &'a mut Buffer) -> Self {
		Self { inner, __phantom: PhantomData }
	}

	/// Queries the facts in the buffer.
	pub fn query<QueryPlan>(
		&'a mut self,
		query_plan: QueryPlan,
	) -> impl Iterator<
		Item = (
			Entity,
			<<QueryPlan as QueryPlanlike<Entity, &'a Buffer>>::Query as Querylike<Entity>>::Item,
		),
	> + 'a
	where
		Entity: 'a,
		QueryPlan: QueryPlanlike<Entity, &'a Buffer> + 'a,
	{
		let mut q = query_plan.into_query(self.inner);
		core::iter::from_fn(move || q.next())
	}

	/// Gets a bundle from the facts in the buffer.
	pub fn get<B, Query: Querylike<Entity, Item = B>, QueryPlan>(
		&'a mut self,
		entity: Entity,
		query_plan: QueryPlan,
	) -> Option<B>
	where
		QueryPlan: QueryPlanlike<Entity, &'a Buffer, Query = Query>,
	{
		let query = query_plan.into_query(self.inner);
		query.get(entity)
	}
}

impl<'a, Entity: Sized, Buffer: Bufferlike<Entity>> From<&'a mut Buffer>
	for Facts<'a, Entity, Buffer>
{
	fn from(inner: &'a mut Buffer) -> Self {
		Facts::new(inner)
	}
}
