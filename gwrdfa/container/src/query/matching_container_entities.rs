use super::all_container_entities::ContainerQuery;
use crate::{ContainerEntity, ContainerEntityBuffer, ContainerGiving};
use parabyzantine::buffer::{QueryPlanlike, Querylike};

pub struct MatchingContainerQuery<'a, T: ContainerGiving<'a, Option<B>> + Sized, B: 'a> {
	container_query: ContainerQuery<'a, T, Option<B>>,
}

impl<'a, T: ContainerGiving<'a, Option<B>> + Sized, B: 'a> MatchingContainerQuery<'a, T, B> {
	pub fn new(buffer: &'a ContainerEntityBuffer<T>) -> Self {
		Self { container_query: ContainerQuery::new(buffer) }
	}
}

impl<'a, T: ContainerGiving<'a, Option<B>> + Sized, B: 'a>
	Querylike<ContainerEntity, ContainerEntityBuffer<T>, B> for MatchingContainerQuery<'a, T, B>
{
	fn next(&mut self) -> Option<(ContainerEntity, B)> {
		self.container_query
			.next()
			.and_then(|(entity, data)| data.map(|data| (entity, data)))
	}

	fn get(&self, entity: ContainerEntity) -> Option<B> {
		self.container_query.get(entity).flatten()
	}
}

#[derive(Debug, Clone, Copy)]
pub struct MatchingContainerEntities;

impl<'a, T: ContainerGiving<'a, Option<B>> + Sized, B: 'a>
	QueryPlanlike<
		'a,
		ContainerEntity,
		ContainerEntityBuffer<T>,
		B,
		MatchingContainerQuery<'a, T, B>,
	> for MatchingContainerEntities
{
	fn build(self, buffer: &'a ContainerEntityBuffer<T>) -> MatchingContainerQuery<'a, T, B> {
		MatchingContainerQuery::new(buffer)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::container::test::TestContainer;

	#[test]
	fn test_matching_container_entities() {
		let mut buffer: ContainerEntityBuffer<TestContainer> = ContainerEntityBuffer::new();
		let container = TestContainer::default();
		let entity = buffer.insert_container(container);
		let query_plan = MatchingContainerEntities;
		let mut query = query_plan.build(&buffer);
		assert_eq!(query.next(), Some((entity, 0 as i32)));
	}
}
