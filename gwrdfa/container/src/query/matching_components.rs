use super::all_components::AllComponentsQuery;
use crate::{Component, ContainerEntity, ContainerEntityBuffer, ContainerGiving};
use core::marker::PhantomData;
use parabyzantine::buffer::query::{QueryPlanlike, Querylike};

pub struct MatchingComponentsQuery<'a, T: ContainerGiving<B> + Sized, B> {
	container_query: AllComponentsQuery<'a, T, B>,
}

impl<'a, T: ContainerGiving<B> + Sized, B> MatchingComponentsQuery<'a, T, B> {
	pub fn new(buffer: &'a ContainerEntityBuffer<T>) -> Self {
		Self { container_query: AllComponentsQuery::new(buffer) }
	}
}

impl<'a, T: ContainerGiving<B> + Sized, B> Querylike<ContainerEntity, &'a B>
	for MatchingComponentsQuery<'a, T, B>
where
	B: 'a,
{
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
pub struct MatchingComponents<B>(PhantomData<B>);

impl<B> MatchingComponents<B> {
	pub fn new() -> Self {
		Self(PhantomData)
	}
}

impl<'a, T, B>
	QueryPlanlike<
		ContainerEntity,
		&'a ContainerEntityBuffer<T>,
		&'a B,
		MatchingComponentsQuery<'a, T, B>,
	> for MatchingComponents<B>
where
	T: ContainerGiving<B> + Sized,
	B: 'a,
{
	fn into_query(self, buffer: &'a ContainerEntityBuffer<T>) -> MatchingComponentsQuery<'a, T, B> {
		MatchingComponentsQuery::new(buffer)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::container::test::{TestContainer, TestField};
	use parabyzantine::buffer::Bufferlike;
	use std::collections::HashSet;

	#[test]
	fn test_matching_container_entities() {
		let mut buffer: ContainerEntityBuffer<TestContainer> = ContainerEntityBuffer::new();
		let container = TestContainer::default();
		let entity = buffer.insert_container(container);
		let query_plan = MatchingComponents::<i32>::new();
		let mut query = query_plan.into_query(&buffer);
		assert_eq!(query.next(), Some((entity, &0)));

		buffer.insert(None, 0 as i32);
		buffer.insert(None, TestField(1));
		buffer.insert(None, TestField(2));

		let mut query = MatchingComponents::<TestField>::new().into_query(&buffer);
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
