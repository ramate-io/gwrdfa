use crate::buffer::{
	query::{IntoQuery, Querylike},
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
	pub fn query<QueryPlan>(
		&self,
		query_plan: QueryPlan,
	) -> impl Iterator<
		Item = (
			Entity,
			<<&'a Buffer as IntoQuery<Entity, QueryPlan>>::Query as Querylike<Entity>>::Item,
		),
	> + 'a
	where
		&'a Buffer: IntoQuery<Entity, QueryPlan>,
		Entity: 'a,
		QueryPlan: 'a,
	{
		let mut q = self.inner.into_query(query_plan);
		core::iter::from_fn(move || q.next())
	}

	/// Gets a bundle from the facts in the buffer.
	pub fn get<B, Query: Querylike<Entity, Item = B>, QueryPlan>(
		&self,
		entity: Entity,
		query_plan: QueryPlan,
	) -> Option<B>
	where
		&'a Buffer: IntoQuery<Entity, QueryPlan, Query = Query>,
	{
		let query = self.inner.into_query(query_plan);
		query.get(entity)
	}
}

impl<'a, Entity: Sized, Buffer: Bufferlike<Entity>> From<&'a Buffer> for Facts<'a, Entity, Buffer> {
	fn from(inner: &'a Buffer) -> Self {
		Facts::new(inner)
	}
}
