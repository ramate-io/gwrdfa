use crate::buffer::Bufferlike;
use core::marker::PhantomData;

pub struct TypedNoOp<T>(PhantomData<T>);

impl<T> TypedNoOp<T> {
	pub fn new() -> Self {
		Self(PhantomData)
	}
}

pub trait Querylike<Entity, Buffer>
where
	Buffer: Bufferlike<Entity>,
{
	type Item;

	fn next(&mut self) -> Option<(Entity, Self::Item)>;
	fn get(&self, entity: Entity) -> Option<Self::Item>;
}

pub trait QueryPlanlike<Entity, Buffer>
where
	Buffer: Bufferlike<Entity>,
{
	type Query<'a>: Querylike<Entity, Buffer>
	where
		Buffer: 'a;

	fn build<'a>(self, buffer: &'a Buffer) -> Self::Query<'a>;
}

impl<Entity, Buffer, T> Querylike<Entity, Buffer> for TypedNoOp<T>
where
	Buffer: Bufferlike<Entity>,
{
	type Item = T;

	fn next(&mut self) -> Option<(Entity, T)> {
		None
	}

	fn get(&self, _entity: Entity) -> Option<T> {
		None
	}
}

impl<Entity, Buffer, T> QueryPlanlike<Entity, Buffer> for TypedNoOp<T>
where
	Buffer: Bufferlike<Entity>,
	T: 'static,
{
	type Query<'a>
		= TypedNoOp<T>
	where
		Buffer: 'a;

	fn build<'a>(self, _buffer: &'a Buffer) -> Self::Query<'a> {
		TypedNoOp::new()
	}
}
