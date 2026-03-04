use crate::{Component, ContainerEntity, ContainerEntityBuffer, ContainerGiving};
use core::marker::PhantomData;
use parabyzantine::buffer::query::{QueryPlanlike, Querylike};

/// A query over a container.
pub struct AllComponentsQuery<'a, T: ContainerGiving<Item> + Sized, Item> {
	buffer: &'a ContainerEntityBuffer<T>,
	iter: std::collections::hash_map::Iter<'a, ContainerEntity, T>,
	_phantom: PhantomData<Item>,
}

impl<'a, T: ContainerGiving<Item> + Sized, Item> AllComponentsQuery<'a, T, Item> {
	/// Creates a new query over a container.
	pub fn new(buffer: &'a ContainerEntityBuffer<T>) -> Self {
		Self { buffer, iter: buffer.iter(), _phantom: PhantomData }
	}
}

impl<'a, T: ContainerGiving<Item> + Sized, Item> Querylike<ContainerEntity, Component<&'a Item>>
	for AllComponentsQuery<'a, T, Item>
where
	Item: 'a,
{
	fn next(&mut self) -> Option<(ContainerEntity, Component<&'a Item>)> {
		self.iter.next().map(|(entity, data)| (entity.clone(), data.as_component()))
	}

	fn get(&self, entity: ContainerEntity) -> Option<Component<&'a Item>> {
		self.buffer.get(entity).map(|container| container.as_component())
	}
}

/// The plan to query all container entities.
#[derive(Debug, Clone, Copy)]
pub struct AllComponents<B>(PhantomData<B>);

impl<B> AllComponents<B> {
	pub fn new() -> Self {
		Self(PhantomData)
	}
}

impl<'a, T, Item>
	QueryPlanlike<
		ContainerEntity,
		&'a ContainerEntityBuffer<T>,
		Component<&'a Item>,
		AllComponentsQuery<'a, T, Item>,
	> for AllComponents<Item>
where
	T: ContainerGiving<Item> + Sized,
	Item: 'a,
{
	fn into_query(self, buffer: &'a ContainerEntityBuffer<T>) -> AllComponentsQuery<'a, T, Item> {
		AllComponentsQuery::new(buffer)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::container::test::TestContainer;

	#[test]
	fn test_all_container_entities() {
		let mut buffer: ContainerEntityBuffer<TestContainer> = ContainerEntityBuffer::new();
		let container = TestContainer::default();
		let entity = buffer.insert_container(container);
		let query_plan = AllComponents::<i32>::new();
		let mut query = query_plan.into_query(&buffer);
		assert_eq!(query.next(), Some((entity, Component::Present(&0))));
	}
}
