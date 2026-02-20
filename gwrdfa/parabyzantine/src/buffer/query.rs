use crate::buffer::Bufferlike;
use core::marker::PhantomData;

pub struct TypedNoOp<T>(PhantomData<T>);

impl<T> TypedNoOp<T> {
	pub fn new() -> Self {
		Self(PhantomData)
	}
}

pub trait Querylike<'a, Entity, Buffer>
where
	Buffer: Bufferlike<Entity>,
{
	type Item: 'a;

	fn next(&mut self) -> Option<(Entity, Self::Item)>;
	fn get(&self, entity: Entity) -> Option<Self::Item>;
}

pub trait QueryPlanlike<Entity, Buffer>
where
	Buffer: Bufferlike<Entity>,
{
	type Query<'a>: Querylike<'a, Entity, Buffer>
	where
		Buffer: 'a;

	fn build<'a>(self, buffer: &'a Buffer) -> Self::Query<'a>;
}

impl<'a, Entity, Buffer, T> Querylike<'a, Entity, Buffer> for TypedNoOp<T>
where
	Buffer: Bufferlike<Entity>,
	T: 'a,
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
