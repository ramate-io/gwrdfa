use crate::buffer::Bufferlike;
use core::marker::PhantomData;

pub struct TypedNoOp<T>(PhantomData<T>);

impl<T> TypedNoOp<T> {
	pub fn new() -> Self {
		Self(PhantomData)
	}
}

pub trait Querylike<Entity> {
	type Item;

	fn next(&mut self) -> Option<(Entity, Self::Item)>;
	fn get(&self, entity: Entity) -> Option<Self::Item>;
}

pub struct QueryIterator<Entity, Query: Querylike<Entity>> {
	__marker: PhantomData<Entity>,
	query: Query,
}

impl<Entity, Query: Querylike<Entity>> QueryIterator<Entity, Query> {
	pub fn new(query: Query) -> Self {
		Self { __marker: PhantomData, query }
	}
}

impl<Entity, Query: Querylike<Entity>> Iterator for QueryIterator<Entity, Query> {
	type Item = (Entity, Query::Item);

	fn next(&mut self) -> Option<Self::Item> {
		self.query.next()
	}
}

pub trait IntoQuery<Entity, T> {
	type Query: Querylike<Entity>;

	fn into_query(self, plan: T) -> Self::Query;
}

impl<Entity, T> Querylike<Entity> for TypedNoOp<T> {
	type Item = T;

	fn next(&mut self) -> Option<(Entity, T)> {
		None
	}

	fn get(&self, _entity: Entity) -> Option<T> {
		None
	}
}
