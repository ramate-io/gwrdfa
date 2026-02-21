use crate::{Component, ContainerEntity, ContainerEntityBuffer, ContainerGiving};
use core::marker::PhantomData;
use parabyzantine::buffer::query::{QueryPlanlike, Querylike};

pub struct MatchingTupleQuery<'a, Container, T> {
	buffer: &'a ContainerEntityBuffer<Container>,
	iter: std::collections::hash_map::Iter<'a, ContainerEntity, Container>,
	_phantom: PhantomData<T>,
}

impl<'a, Container, T> MatchingTupleQuery<'a, Container, T> {
	pub fn new(buffer: &'a ContainerEntityBuffer<Container>) -> Self {
		Self { buffer, iter: buffer.iter(), _phantom: PhantomData }
	}
}

impl<'a, T: ContainerGiving<'a, A> + ContainerGiving<'a, B> + Sized, A: 'a, B: 'a>
	Querylike<'a, ContainerEntity, ContainerEntityBuffer<T>> for MatchingTupleQuery<'a, T, (A, B)>
{
	type Item = (&'a A, &'a B);

	fn next(&mut self) -> Option<(ContainerEntity, (&'a A, &'a B))> {
		while let Some((entity, container)) = self.iter.next() {
			if let (Component::Present(a), Component::Present(b)) =
				(container.as_component(), container.as_component())
			{
				return Some((*entity, (a, b)));
			}
		}
		None
	}

	fn get(&self, entity: ContainerEntity) -> Option<(&'a A, &'a B)> {
		let a = self.buffer.get(entity).map(|container| container.as_component());
		let b = self.buffer.get(entity).map(|container| container.as_component());
		match (a, b) {
			(Some(Component::Present(a)), Some(Component::Present(b))) => Some((a, b)),
			_ => None,
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct MatchingTuple<T>(PhantomData<T>);

impl<T> MatchingTuple<T> {
	pub fn new() -> Self {
		Self(PhantomData)
	}
}

impl<T, A, B> QueryPlanlike<ContainerEntity, ContainerEntityBuffer<T>> for MatchingTuple<(A, B)>
where
	for<'a> T: ContainerGiving<'a, A> + ContainerGiving<'a, B> + Sized,
	A: 'static,
	B: 'static,
{
	type Query<'a>
		= MatchingTupleQuery<'a, T, (A, B)>
	where
		ContainerEntityBuffer<T>: 'a;

	fn build<'a>(self, buffer: &'a ContainerEntityBuffer<T>) -> MatchingTupleQuery<'a, T, (A, B)> {
		MatchingTupleQuery::new(buffer)
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
		let _entity = buffer.insert_container(container);
		let query_plan = MatchingTuple::<(i32, TestField)>::new();
		let mut query = query_plan.build(&buffer);
		assert_eq!(query.next(), None);

		buffer.insert(None, ToContainer(0 as i32));
		buffer.insert(None, ToContainer(TestField(1)));
		buffer.insert(None, ToContainer(TestField(2)));

		let mut query = MatchingTuple::<(i32, TestField)>::new().build(&buffer);
		let mut matched_entities: HashSet<(ContainerEntity, (&i32, &TestField))> = HashSet::new();

		while let Some(tuple) = query.next() {
			matched_entities.insert(tuple);
		}

		let expected_entities = HashSet::from([
			(ContainerEntity::new(2), (&0, &TestField(1))),
			(ContainerEntity::new(3), (&0, &TestField(2))),
		]);
		assert_eq!(matched_entities, expected_entities);
	}
}
