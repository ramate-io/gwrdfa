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

pub trait IntoQuery<Entity, T> {
	type Query<'a>: Querylike<Entity>
	where
		Self: 'a;

	fn into_query<'a>(&'a self, plan: T) -> Self::Query<'a>;
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
