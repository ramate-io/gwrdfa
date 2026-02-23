use crate::{NoOp, NoOpOn};

pub trait Querylike<Entity> {
	type Item;

	fn next(&mut self) -> Option<(Entity, Self::Item)>;
	fn get(&self, entity: Entity) -> Option<Self::Item>;
}

pub trait IntoQuery<Entity, T> {
	type Query: Querylike<Entity>;

	fn into_query(self, plan: T) -> Self::Query;
}

impl<Entity> Querylike<Entity> for NoOp {
	type Item = NoOp;

	fn next(&mut self) -> Option<(Entity, NoOp)> {
		None
	}

	fn get(&self, _entity: Entity) -> Option<NoOp> {
		None
	}
}

impl<Entity> IntoQuery<Entity, NoOp> for NoOp {
	type Query = NoOp;

	fn into_query(self, _plan: NoOp) -> Self::Query {
		NoOp
	}
}

impl<Entity, T> Querylike<Entity> for NoOpOn<T> {
	type Item = T;

	fn next(&mut self) -> Option<(Entity, T)> {
		None
	}

	fn get(&self, _entity: Entity) -> Option<T> {
		None
	}
}

impl<Entity, T> IntoQuery<Entity, NoOpOn<T>> for NoOpOn<T> {
	type Query = NoOpOn<T>;

	fn into_query(self, _plan: NoOpOn<T>) -> Self::Query {
		NoOpOn::new()
	}
}
