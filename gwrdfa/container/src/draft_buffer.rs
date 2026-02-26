use crate::{ContainerEntity, ContainerEntityBuffer, ContainerStores, DeltaContainer};
use parabyzantine::buffer::{DraftBufferlike, Stores};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct ContainerEntityDraftBuffer<T: Sized> {
	// NOTE: currently, we don't support double updates on the draft buffer.
	// So, it is fine to simply stack insertions that don't know their entity yet.
	new_insertions: Vec<(Option<ContainerEntity>, T)>,
	entities: HashMap<ContainerEntity, T>,
}

impl<T: Sized> ContainerEntityDraftBuffer<T> {
	pub fn new() -> Self {
		Self { new_insertions: Vec::new(), entities: HashMap::new() }
	}

	pub fn add_delta_container(&mut self, value: T) {
		self.new_insertions.push((None, value));
	}

	pub fn upsert_delta_container(&mut self, entity: ContainerEntity, value: T) {
		self.entities.insert(entity, value);
	}

	pub fn remove_delta_container(&mut self, entity: ContainerEntity) {
		self.entities.remove(&entity);
	}

	pub fn get_delta_container_mut(&mut self, entity: ContainerEntity) -> Option<&mut T> {
		self.entities.get_mut(&entity)
	}
}

impl<B, T: ContainerStores<B>> Stores<B, ContainerEntity> for ContainerEntityDraftBuffer<T> {
	fn insert_record(
		&mut self,
		entity: Option<ContainerEntity>,
		value: B,
	) -> Option<ContainerEntity> {
		match entity {
			Some(entity) => {
				match self.get_delta_container_mut(entity) {
					Some(data) => data.update_with_data(value),
					None => self.upsert_delta_container(entity, T::from_data(value)),
				}
				Some(entity)
			}
			None => {
				self.add_delta_container(T::from_data(value));
				// We return none because we don't assign an entity to the delta container yet.
				None
			}
		}
	}

	fn remove_record(&mut self, entity: ContainerEntity) {
		match self.get_delta_container_mut(entity) {
			Some(container) => container.remove_from_container(),
			None => self.new_insertions.push((Some(entity), T::from_removed_data())),
		}
	}
}

impl<D: DeltaContainer<C>, C: Sized> DraftBufferlike<ContainerEntity, ContainerEntityBuffer<C>>
	for ContainerEntityDraftBuffer<D>
{
	fn draft_insert<B>(&mut self, entity: Option<ContainerEntity>, value: B)
	where
		Self: Stores<B, ContainerEntity>,
	{
		self.insert_record(entity, value);
	}

	fn draft_remove<B>(&mut self, entity: ContainerEntity)
	where
		Self: Stores<B, ContainerEntity>,
	{
		self.remove_record(entity);
	}

	fn draft_remove_entity(&mut self, entity: ContainerEntity) {
		self.remove_delta_container(entity);
	}

	fn commit(self, buffer: &mut ContainerEntityBuffer<C>) {
		// Apply all deltas to the containers in the buffer.
		for (entity, deltas) in self.entities.into_iter() {
			match buffer.get_mut(entity) {
				Some(container) => deltas.apply_deltas(container),
				None => {
					// for now, do nothing
				}
			}
		}

		// Insert all new containers into the buffer.
		for (entity, delta) in self.new_insertions {
			if let Some(entity) = entity {
				match buffer.get_mut(entity) {
					Some(container) => delta.apply_deltas(container),
					None => {
						buffer.insert_container(delta.into_container());
					}
				}
			} else {
				buffer.insert_container(delta.into_container());
			}
		}
	}
}
