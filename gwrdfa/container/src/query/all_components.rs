use crate::{Component, ContainerEntity, ContainerEntityBuffer, ContainerGiving};
use core::marker::PhantomData;
use parabyzantine::buffer::query::{QueryPlanlike, Querylike};

/// A query over a container.
pub struct AllComponents<'a, T: ContainerGiving<'a, B> + Sized, B> {
	buffer: &'a ContainerEntityBuffer<T>,
	iter: std::collections::hash_map::Iter<'a, ContainerEntity, T>,
	_phantom: PhantomData<B>,
}

impl<'a, T: ContainerGiving<'a, B> + Sized, B> AllComponents<'a, T, B> {
	/// Creates a new query over a container.
	pub fn new(buffer: &'a ContainerEntityBuffer<T>) -> Self {
		Self { buffer, iter: buffer.iter(), _phantom: PhantomData }
	}
}

impl<'a, T: ContainerGiving<'a, B> + Sized, B>
	Querylike<'a, ContainerEntity, ContainerEntityBuffer<T>> for AllComponents<'a, T, B>
where
	B: 'a,
{
	type Item = Component<&'a B>;

	fn next(&mut self) -> Option<(ContainerEntity, Component<&'a B>)> {
		self.iter.next().map(|(entity, data)| (entity.clone(), data.as_component()))
	}

	fn get(&self, entity: ContainerEntity) -> Option<Component<&'a B>> {
		self.buffer.get(entity).map(|container| container.as_component())
	}
}

/// The plan to query all container entities.
#[derive(Debug, Clone, Copy)]
pub struct AllContainerEntities<B>(PhantomData<B>);

impl<B> AllContainerEntities<B> {
	pub fn new() -> Self {
		Self(PhantomData)
	}
}

impl<T, B> QueryPlanlike<ContainerEntity, ContainerEntityBuffer<T>> for AllContainerEntities<B>
where
	for<'a> T: ContainerGiving<'a, B> + Sized,
	B: 'static,
{
	type Query<'a>
		= AllComponents<'a, T, B>
	where
		ContainerEntityBuffer<T>: 'a;

	fn build<'a>(self, buffer: &'a ContainerEntityBuffer<T>) -> AllComponents<'a, T, B> {
		AllComponents::new(buffer)
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
		let query_plan = AllContainerEntities::<i32>::new();
		let mut query = query_plan.build(&buffer);
		assert_eq!(query.next(), Some((entity, Component::Present(&0))));
	}
}
