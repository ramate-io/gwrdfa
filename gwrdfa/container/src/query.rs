use crate::{Container, ContainerEntity, ContainerEntityBuffer};
use parabyzantine::buffer::{Bundle, QueryPlanlike, Querylike};

/// A query over a container.
pub struct ContainerQuery<'a, T: Sized> {
	buffer: &'a ContainerEntityBuffer<T>,
	iter: std::collections::hash_map::Iter<'a, ContainerEntity, T>,
}

impl<'a, T: Sized> ContainerQuery<'a, T> {
	/// Creates a new query over a container.
	pub fn new(buffer: &'a ContainerEntityBuffer<T>) -> Self {
		Self { buffer, iter: buffer.iter() }
	}
}

impl<'a, T: Container<B> + Sized, B: Bundle<ContainerEntity, ContainerEntityBuffer<T>>>
	Querylike<ContainerEntity, ContainerEntityBuffer<T>, B> for ContainerQuery<'a, T>
{
	fn next(&mut self) -> Option<(ContainerEntity, B)> {
		self.iter.next().map(|(entity, data)| (entity.clone(), data.as_bundle()))
	}

	fn get(&self, entity: ContainerEntity) -> Option<B> {
		self.buffer.get(entity).map(|container| container.as_bundle())
	}
}

/// The query plan for all container entities.
#[derive(Debug, Clone, Copy)]
pub struct AllContainerEntities;

impl<'a, T: Container<B> + Sized, B: Bundle<ContainerEntity, ContainerEntityBuffer<T>>>
	QueryPlanlike<'a, ContainerEntity, ContainerEntityBuffer<T>, B, ContainerQuery<'a, T>>
	for AllContainerEntities
{
	fn build(self, buffer: &'a ContainerEntityBuffer<T>) -> ContainerQuery<'a, T> {
		ContainerQuery::new(buffer)
	}
}
