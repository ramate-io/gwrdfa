use parabyzantine::buffer::{Bufferlike, Bundle};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct ContainerEntity(usize);

impl ContainerEntity {
	pub fn new(index: usize) -> Self {
		Self(index)
	}

	pub fn index(&self) -> usize {
		self.0
	}

	pub fn next(&self) -> Self {
		Self(self.0 + 1)
	}
}

pub trait Container<B: Sized> {
	/// Creates a new container from a bundle.
	fn from_bundle(bundle: B) -> Self;

	/// Updates a container with a bundle.
	fn update_with_bundle(&mut self, bundle: B);

	/// Gets a the bundle from a reference to the container
	fn as_bundle(&self) -> B;
}

pub struct ContainerEntityBuffer<T: Sized> {
	next_entity: ContainerEntity,
	entities: HashMap<ContainerEntity, T>,
	doubly_linked_mapping:
		HashMap<ContainerEntity, (Option<ContainerEntity>, Option<ContainerEntity>)>,
	last_entity: Option<ContainerEntity>,
}

impl<T: Sized> ContainerEntityBuffer<T> {
	pub fn new() -> Self {
		Self {
			next_entity: ContainerEntity::new(0),
			entities: HashMap::new(),
			doubly_linked_mapping: HashMap::new(),
			last_entity: None,
		}
	}

	/// Inserts new data for a new entity.
	pub fn insert(&mut self, data: T) -> ContainerEntity {
		let entity = self.next_entity;
		self.next_entity = self.next_entity.next();
		self.entities.insert(entity, data);

		// update the doubly linked mapping
		self.doubly_linked_mapping.insert(entity, (self.last_entity, None));
		self.last_entity = Some(entity);

		entity
	}

	/// Inserts entirely new data for a known entity.
	pub fn upsert(&mut self, entity: ContainerEntity, data: T) {
		self.entities.insert(entity, data);
	}

	/// Removes an entity from the buffer.
	pub fn remove(&mut self, entity: ContainerEntity) {
		//
		if let Some((prev, next)) = self.doubly_linked_mapping.remove(&entity) {
			// point the previous entity to the next entity
			if let Some(prev) = prev {
				if let Some(prev) = self.doubly_linked_mapping.get_mut(&prev) {
					prev.1 = next;
				}
			}

			// point the next entity to the previous entity
			if let Some(next) = next {
				if let Some(next) = self.doubly_linked_mapping.get_mut(&next) {
					next.0 = prev;
				}
			}

			// if the entity is the last entity, update the last entity
			if let Some(last_entity) = self.last_entity {
				if last_entity == entity {
					self.last_entity = prev;
				}
			}
		}

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
	fn remove_entity(&mut self, entity: ContainerEntity) {
		self.remove(entity);
	}
}

#[derive(Debug, Clone)]
pub struct ToContainer<T: Sized>(pub T);

impl<T, B> Bundle<ContainerEntity, ContainerEntityBuffer<T>> for ToContainer<B>
where
	T: Container<B>,
	B: Sized,
{
	fn insert_into(self, buffer: &mut ContainerEntityBuffer<T>, entity: Option<ContainerEntity>) {
		match entity {
			Some(entity) => match buffer.get_mut(entity) {
				Some(data) => data.update_with_bundle(self.0),
				None => buffer.upsert(entity, T::from_bundle(self.0)),
			},
			None => {
				buffer.insert(T::from_bundle(self.0));
			}
		}
	}

	fn remove_from(self, buffer: &mut ContainerEntityBuffer<T>, entity: ContainerEntity) {
		buffer.remove(entity);
	}
}
