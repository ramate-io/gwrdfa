use crate::NoOp;

pub trait Querylike<Entity, Item> {
	fn next(&mut self) -> Option<(Entity, Item)>;
	fn get(&self, entity: Entity) -> Option<Item>;
}

pub trait QueryPlanlike<Entity, C, Item, Q: Querylike<Entity, Item>> {
	fn into_query(self, plan: C) -> Q;
}

impl<Entity, Item> Querylike<Entity, Item> for NoOp {
	fn next(&mut self) -> Option<(Entity, Item)> {
		None
	}

	fn get(&self, _entity: Entity) -> Option<Item> {
		None
	}
}

impl<Entity, C, Item, Q: Querylike<Entity, Item> + Default> QueryPlanlike<Entity, C, Item, Q>
	for NoOp
{
	fn into_query(self, _plan: C) -> Q {
		Q::default()
	}
}
