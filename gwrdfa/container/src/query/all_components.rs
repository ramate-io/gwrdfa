use crate::{Component, ContainerEntity, ContainerEntityBuffer, ContainerGiving};
use core::marker::PhantomData;
use parabyzantine::buffer::query::{IntoQuery, Querylike};

/// A query over a container.
pub struct AllComponentsQuery<'a, T: ContainerGiving<B> + Sized, B> {
	buffer: &'a ContainerEntityBuffer<T>,
	iter: std::collections::hash_map::Iter<'a, ContainerEntity, T>,
	_phantom: PhantomData<B>,
}

impl<'a, T: ContainerGiving<B> + Sized, B> AllComponentsQuery<'a, T, B> {
	/// Creates a new query over a container.
	pub fn new(buffer: &'a ContainerEntityBuffer<T>) -> Self {
		Self { buffer, iter: buffer.iter(), _phantom: PhantomData }
	}
}

impl<'a, T: ContainerGiving<B> + Sized, B> Querylike<ContainerEntity>
	for AllComponentsQuery<'a, T, B>
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
pub struct AllComponents<B>(PhantomData<B>);

impl<B> AllComponents<B> {
	pub fn new() -> Self {
		Self(PhantomData)
	}
}

impl<'a, T, B> IntoQuery<ContainerEntity, AllComponents<B>> for &'a ContainerEntityBuffer<T>
where
	T: ContainerGiving<B> + Sized + 'static,
	B: 'static,
{
	type Query = AllComponentsQuery<'a, T, B>;

	fn into_query(self, _plan: AllComponents<B>) -> AllComponentsQuery<'a, T, B> {
		AllComponentsQuery::new(self)
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
		let mut query = (&buffer).into_query(query_plan);
		assert_eq!(query.next(), Some((entity, Component::Present(&0))));
	}
}
