use super::all_components::AllComponents;
use crate::{Component, ContainerEntity, ContainerEntityBuffer, ContainerGiving};
use core::marker::PhantomData;
use parabyzantine::buffer::query::{QueryPlanlike, Querylike};

pub struct MatchingAllComponents<'a, T: ContainerGiving<'a, B> + Sized, B: 'a> {
	container_query: AllComponents<'a, T, B>,
}

impl<'a, T: ContainerGiving<'a, B> + Sized, B: 'a> MatchingAllComponents<'a, T, B> {
	pub fn new(buffer: &'a ContainerEntityBuffer<T>) -> Self {
		Self { container_query: AllComponents::new(buffer) }
	}
}

impl<'a, T: ContainerGiving<'a, B> + Sized, B: 'a>
	Querylike<'a, ContainerEntity, ContainerEntityBuffer<T>> for MatchingAllComponents<'a, T, B>
{
	type Item = &'a B;

	fn next(&mut self) -> Option<(ContainerEntity, &'a B)> {
		while let Some((entity, data)) = self.container_query.next() {
			if let Component::Present(data) = data {
				return Some((entity, data));
			}
		}
		None
	}

	fn get(&self, entity: ContainerEntity) -> Option<&'a B> {
		match self.container_query.get(entity) {
			Some(Component::Present(data)) => Some(data),
			Some(Component::Absent) => None,
			None => None,
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct MatchingContainerEntities<B>(PhantomData<B>);

impl<B> MatchingContainerEntities<B> {
	pub fn new() -> Self {
		Self(PhantomData)
	}
}

impl<T, B> QueryPlanlike<ContainerEntity, ContainerEntityBuffer<T>> for MatchingContainerEntities<B>
where
	for<'a> T: ContainerGiving<'a, B> + Sized,
	B: 'static,
{
	type Query<'a>
		= MatchingAllComponents<'a, T, B>
	where
		ContainerEntityBuffer<T>: 'a;

	fn build<'a>(self, buffer: &'a ContainerEntityBuffer<T>) -> MatchingAllComponents<'a, T, B> {
		MatchingAllComponents::new(buffer)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::buffer::ToContainer;
	use crate::container::test::{TestContainer, TestField};
	use parabyzantine::buffer::Bufferlike;
	use std::collections::HashSet;

	#[test]
	fn test_matching_container_entities() {
		let mut buffer: ContainerEntityBuffer<TestContainer> = ContainerEntityBuffer::new();
		let container = TestContainer::default();
		let entity = buffer.insert_container(container);
		let query_plan = MatchingContainerEntities::<i32>::new();
		let mut query = query_plan.build(&buffer);
		assert_eq!(query.next(), Some((entity, &0)));

		buffer.insert(None, ToContainer(0 as i32));
		buffer.insert(None, ToContainer(TestField(1)));
		buffer.insert(None, ToContainer(TestField(2)));

		let mut query = MatchingContainerEntities::<TestField>::new().build(&buffer);
		let mut matched_entities: HashSet<(ContainerEntity, &TestField)> = HashSet::new();
		while let Some(tuple) = query.next() {
			matched_entities.insert(tuple);
		}

		let expected_entities = HashSet::from([
			(ContainerEntity::new(2), &TestField(1)),
			(ContainerEntity::new(3), &TestField(2)),
		]);
		assert_eq!(matched_entities, expected_entities);
	}
}
