use parabyzantine::buffer::{Bufferlike, Bundle};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
}

impl<T: Sized> ContainerEntityBuffer<T> {
	pub fn new() -> Self {
		Self { next_entity: ContainerEntity::new(0), entities: HashMap::new() }
	}

	/// Inserts new data for a new entity.
	pub fn insert(&mut self, data: T) -> ContainerEntity {
		let entity = self.next_entity;
		self.next_entity = self.next_entity.next();
		self.entities.insert(entity, data);
		entity
	}

	/// Inserts entirely new data for a known entity.
	pub fn upsert(&mut self, entity: ContainerEntity, data: T) {
		self.entities.insert(entity, data);
	}

	pub fn remove(&mut self, entity: ContainerEntity) {
		self.entities.remove(&entity);
	}

	pub fn get(&self, entity: ContainerEntity) -> Option<&T> {
		self.entities.get(&entity)
	}

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
