use crate::{ContainerAccepting, ContainerEntity, ContainerHoldingOps};
use parabyzantine::buffer::{Bufferlike, Stores};
use std::collections::HashMap;

pub struct ContainerEntityBuffer<T: Sized> {
	next_entity: ContainerEntity,
	entities: HashMap<ContainerEntity, T>,
}

impl<T: Sized> ContainerEntityBuffer<T> {
	pub fn new() -> Self {
		Self { next_entity: ContainerEntity::new(0), entities: HashMap::new() }
	}

	/// Iterates over the entities in the buffer.
	pub fn iter<'a>(&'a self) -> std::collections::hash_map::Iter<'a, ContainerEntity, T> {
		self.entities.iter()
	}

	/// Inserts new data for a new entity.
	pub fn insert_container(&mut self, data: T) -> ContainerEntity {
		let entity = self.next_entity;
		self.next_entity = self.next_entity.next();
		self.entities.insert(entity, data);
		entity
	}

	/// Inserts entirely new data for a known entity.
	pub fn upsert_container(&mut self, entity: ContainerEntity, data: T) {
		self.entities.insert(entity, data);
	}

	/// Removes an entity from the buffer.
	pub fn remove_container(&mut self, entity: ContainerEntity) {
		self.entities.remove(&entity);
	}

	/// Gets a reference to the data for a given entity.
	pub fn get(&self, entity: ContainerEntity) -> Option<&T> {
		self.entities.get(&entity)
	}

	/// Gets a mutable reference to the data for a given entity.
	pub fn get_mut(&mut self, entity: ContainerEntity) -> Option<&mut T> {
		self.entities.get_mut(&entity)
	}
}

impl<T: Sized> Bufferlike<ContainerEntity> for ContainerEntityBuffer<T> {
	/// Removes an entity from the buffer.
	fn remove_entity(&mut self, entity: ContainerEntity) {
		self.remove_container(entity);
	}
}

impl<B, T: ContainerAccepting<B>> Stores<B, ContainerEntity> for ContainerEntityBuffer<T> {
	fn insert_record(
		&mut self,
		entity: Option<ContainerEntity>,
		value: B,
	) -> Option<ContainerEntity> {
		match entity {
			Some(entity) => {
				match self.get_mut(entity) {
					Some(data) => data.update_with_data(value),
					None => self.upsert_container(entity, T::from_data(value)),
				}
				Some(entity)
			}
			None => Some(self.insert_container(T::from_data(value))),
		}
	}

	fn remove_record(&mut self, entity: ContainerEntity) {
		self.remove_container(entity);
	}
}

#[cfg(test)]
pub mod test {
	use super::*;
	use crate::container::test::TestContainer;
	use crate::container::test::TestField;
	use crate::Component;
	use parabyzantine::buffer::Bufferlike;

	#[test]
	fn test_insert() {
		let mut buffer = ContainerEntityBuffer::new();
		let container = TestContainer::default();
		let entity = buffer.insert_container(container);
		assert_eq!(buffer.get(entity), Some(&TestContainer::default()));
	}

	#[test]
	fn test_insert_into_with_to_container() {
		// Insert the whole container
		let mut buffer: ContainerEntityBuffer<TestContainer> = ContainerEntityBuffer::new();
		let container = TestContainer::default();
		let entity = buffer.insert_container(container);
		assert_eq!(entity, ContainerEntity::new(0));
		assert_eq!(buffer.get(entity), Some(&TestContainer::default()));

		// Insert a container component as a new entity
		let num: i32 = 1;
		let entity = buffer.insert_record(None, num);
		assert_eq!(entity, Some(ContainerEntity::new(1)));
		assert_eq!(
			buffer.get(ContainerEntity::new(1)),
			Some(&TestContainer::default().with_num(1))
		);

		// Insert a container component on an existing entity
		let container = TestContainer::default().with_num(2);
		let entity = buffer.insert_container(container);
		assert_eq!(entity, ContainerEntity::new(2));
		let num: i32 = 3;
		let entity = buffer.insert_record(Some(entity), num);
		assert_eq!(entity, Some(ContainerEntity::new(2)));
		assert_eq!(
			buffer.get(ContainerEntity::new(2)),
			Some(&TestContainer::default().with_num(3))
		);

		// Insert using the bufferlike API
		let entity = buffer.insert_record(None, 4 as i32);
		assert_eq!(entity, Some(ContainerEntity::new(3)));
		assert_eq!(
			buffer.get(ContainerEntity::new(3)),
			Some(&TestContainer::default().with_num(4))
		);
	}

	#[test]
	fn test_remove() {
		let mut buffer: ContainerEntityBuffer<TestContainer> = ContainerEntityBuffer::new();
		let container = TestContainer::default();
		let entity = buffer.insert_container(container);
		buffer.remove_container(entity);
		assert_eq!(buffer.get(entity), None);
	}

	#[test]
	fn test_bufferlike_remove() {
		let mut buffer: ContainerEntityBuffer<TestContainer> = ContainerEntityBuffer::new();
		let container = TestContainer::default().with_field(Component::Present(TestField(1)));
		let entity = buffer.insert_container(container);
		buffer.remove::<TestField>(entity);
		assert_eq!(
			buffer.get(entity),
			Some(&TestContainer::default().with_field(Component::Absent))
		);
	}

	#[test]
	fn test_multiple_inserts_and_removes() {
		// First insert
		let mut buffer: ContainerEntityBuffer<TestContainer> = ContainerEntityBuffer::new();
		let container = TestContainer::default();
		let entity = buffer.insert_container(container);
		buffer.remove_container(entity);
		assert_eq!(buffer.get(entity), None);

		// Second insert
		let container = TestContainer::default().with_num(1);
		let entity = buffer.insert_container(container);
		assert_eq!(buffer.get(entity), Some(&TestContainer::default().with_num(1)));

		// Third insert
		let container = TestContainer::default().with_num(2);
		let entity = buffer.insert_container(container);
		assert_eq!(buffer.get(entity), Some(&TestContainer::default().with_num(2)));
	}
}
