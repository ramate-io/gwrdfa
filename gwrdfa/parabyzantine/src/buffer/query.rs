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

pub trait QueryPlanlike<Entity, C> {
	type Query: Querylike<Entity>;

	fn into_query_plan(self, plan: C) -> Self::Query;
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

impl<'a, Entity> IntoQuery<Entity, NoOp> for &'a NoOp {
	type Query = NoOp;

	fn into_query(self, _plan: NoOp) -> Self::Query {
		NoOp
	}
}

impl<'a, Entity, C> QueryPlanlike<Entity, C> for NoOp {
	type Query = NoOp;

	fn into_query_plan(self, _plan: C) -> Self::Query {
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

impl<'a, Entity, Plan, Item> IntoQuery<Entity, Plan> for &'a NoOpOn<(Plan, Item)> {
	type Query = NoOpOn<(Plan, Item)>;

	fn into_query(self, _plan: Plan) -> Self::Query {
		NoOpOn::new()
	}
}
