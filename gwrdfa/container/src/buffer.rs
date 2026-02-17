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

/// A trait representing types that are containable in a particular container.
///
/// Generally, this is what you will want to use to implement container bindings.
/// More often, you will have crate-local types that are containable in a particular container,
/// while the container may come from elsewhere.
pub trait ContainableIn<T: Sized> {
	fn into_container(self) -> T;

	fn update_container(self, container: &mut T);

	fn from_container(container: &T) -> Self;
}

/// A trait representing valid conntainers for bundles.
pub trait Container<B: Sized> {
	/// Creates a new container from a bundle.
	fn from_bundle(bundle: B) -> Self;

	/// Updates a container with a bundle.
	fn update_with_bundle(&mut self, bundle: B);

	/// Gets a the bundle from a reference to the container
	fn as_bundle(&self) -> B;
}

/// All types that are containable in a type induce a container on that type.
impl<T: Sized, B: ContainableIn<T>> Container<B> for T {
	fn from_bundle(bundle: B) -> Self {
		bundle.into_container()
	}

	fn update_with_bundle(&mut self, bundle: B) {
		bundle.update_container(self);
	}

	fn as_bundle(&self) -> B {
		B::from_container(self)
	}
}

pub struct ContainerEntityBuffer<T: Sized> {
	next_entity: ContainerEntity,
	entities: HashMap<ContainerEntity, T>,
}

impl<T: Sized> ContainerEntityBuffer<T> {
	pub fn new() -> Self {
		Self { next_entity: ContainerEntity::new(0), entities: HashMap::new() }
	}

	pub fn iter(&self) -> std::collections::hash_map::Iter<ContainerEntity, T> {
		self.entities.iter()
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

	/// Removes an entity from the buffer.
	pub fn remove(&mut self, entity: ContainerEntity) {
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
