use super::all_components::AllComponents;
use crate::{ContainerEntity, ContainerEntityBuffer, ContainerGiving};
use parabyzantine::buffer::{QueryPlanlike, Querylike};

pub struct MatchingAllComponents<'a, T: ContainerGiving<'a, B> + Sized, B: 'a> {
	container_query: AllComponents<'a, T, B>,
}

impl<'a, T: ContainerGiving<'a, B> + Sized, B: 'a> MatchingAllComponents<'a, T, B> {
	pub fn new(buffer: &'a ContainerEntityBuffer<T>) -> Self {
		Self { container_query: AllComponents::new(buffer) }
	}
}

impl<'a, T: ContainerGiving<'a, B> + Sized, B: 'a>
	Querylike<ContainerEntity, ContainerEntityBuffer<T>, B> for MatchingAllComponents<'a, T, B>
{
	fn next(&mut self) -> Option<(ContainerEntity, B)> {
		while let Some((entity, data)) = self.container_query.next() {
			if let Some(data) = data {
				return Some((entity, data));
			}
		}
		None
	}

	fn get(&self, entity: ContainerEntity) -> Option<B> {
		self.container_query.get(entity).flatten()
	}
}

#[derive(Debug, Clone, Copy)]
pub struct MatchingContainerEntities;

impl<T, B> QueryPlanlike<ContainerEntity, ContainerEntityBuffer<T>, B> for MatchingContainerEntities
where
	for<'a> T: ContainerGiving<Option<B>> + Sized,
	for<'a> B: 'a,
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
		let query_plan = MatchingContainerEntities;
		let mut query = query_plan.build(&buffer);
		assert_eq!(query.next(), Some((entity, 0 as i32)));

		buffer.insert(None, ToContainer(0 as i32));
		buffer.insert(None, ToContainer(TestField(1)));
		buffer.insert(None, ToContainer(TestField(2)));

		let mut query = MatchingContainerEntities.build(&buffer);
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
