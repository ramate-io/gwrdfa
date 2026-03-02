use crate::{NoOp, NoOpOn};

pub trait Querylike<Entity> {
	type Item;

	fn next(&mut self) -> Option<(Entity, Self::Item)>;
	fn get(&self, entity: Entity) -> Option<Self::Item>;
}

pub trait QueryPlanlike<Entity, C> {
	type Query: Querylike<Entity>;

	fn into_query(self, plan: C) -> Self::Query;
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

impl<Entity, C> QueryPlanlike<Entity, C> for NoOp {
	type Query = NoOp;

	fn into_query(self, _plan: C) -> Self::Query {
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

impl<'a, Entity, T> Querylike<Entity> for &'a NoOpOn<T> {
	type Item = T;

	fn next(&mut self) -> Option<(Entity, T)> {
		None
	}

	fn get(&self, _entity: Entity) -> Option<T> {
		None
	}
}
