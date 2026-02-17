use crate::{ContainerEntity, ContainerEntityBuffer, ContainerGiving};
use core::marker::PhantomData;
use parabyzantine::buffer::{QueryPlanlike, Querylike};

/// A query over a container.
pub struct ContainerQuery<'a, T: ContainerGiving<'a, B> + Sized, B> {
	buffer: &'a ContainerEntityBuffer<T>,
	iter: std::collections::hash_map::Iter<'a, ContainerEntity, T>,
	_phantom: PhantomData<B>,
}

impl<'a, T: ContainerGiving<'a, B> + Sized, B> ContainerQuery<'a, T, B> {
	/// Creates a new query over a container.
	pub fn new(buffer: &'a ContainerEntityBuffer<T>) -> Self {
		Self { buffer, iter: buffer.iter(), _phantom: PhantomData }
	}
}

impl<'a, T: ContainerGiving<'a, B> + Sized, B>
	Querylike<ContainerEntity, ContainerEntityBuffer<T>, B> for ContainerQuery<'a, T, B>
{
	fn next(&mut self) -> Option<(ContainerEntity, B)> {
		self.iter.next().map(|(entity, data)| (entity.clone(), data.as_item()))
	}

	fn get(&self, entity: ContainerEntity) -> Option<B> {
		self.buffer.get(entity).map(|container| container.as_item())
	}
}

/// The plan to query all container entities.
#[derive(Debug, Clone, Copy)]
pub struct AllContainerEntities;

impl<'a, T: ContainerGiving<'a, B> + Sized, B: 'a>
	QueryPlanlike<'a, ContainerEntity, ContainerEntityBuffer<T>, B, ContainerQuery<'a, T, B>>
	for AllContainerEntities
{
	fn build(self, buffer: &'a ContainerEntityBuffer<T>) -> ContainerQuery<'a, T, B> {
		ContainerQuery::new(buffer)
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
		let query_plan = AllContainerEntities;
		let mut query = query_plan.build(&buffer);
		assert_eq!(query.next(), Some((entity, 0 as i32)));
	}
}
