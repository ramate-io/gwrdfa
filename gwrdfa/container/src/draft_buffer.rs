use crate::{ContainerEntity, ContainerEntityBuffer, ContainerStores, DeltasContainer};
use parabyzantine::buffer::{DraftBufferlike, Stores};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
pub struct ContainerEntityDraftBuffer<T: Sized> {
	/// NOTE: currently, we don't support double updates on the draft buffer.
	/// So, it is fine to simply stack insertions that don't know their entity yet.
	/// In the future, we would assign an entity here and simply
	/// put everything into [ContainerEntityDraftBuffer::known_entities].
	new_entities: Vec<T>,
	/// The hash map allows for compaction.
	known_entities: HashMap<ContainerEntity, T>,
	/// Keep track of the entities for removal.
	///
	/// An known entity modification that occurs after removal,
	/// will compact away the removal.
	///
	/// A removal that occurs after a modification will compact away the modification.
	entities_for_removal: HashSet<ContainerEntity>,
}

impl<T: Sized> ContainerEntityDraftBuffer<T> {
	pub fn new() -> Self {
		Self {
			new_entities: Vec::new(),
			known_entities: HashMap::new(),
			entities_for_removal: HashSet::new(),
		}
	}

	pub fn add_delta_container(&mut self, value: T) {
		self.new_entities.push(value);
	}

	pub fn get_delta_container_mut(&mut self, entity: ContainerEntity) -> Option<&mut T> {
		self.known_entities.get_mut(&entity)
	}

	pub fn insert_delta_container(&mut self, entity: ContainerEntity, value: T) {
		self.known_entities.insert(entity, value);
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
				// Update or insert the delta container.
				match self.get_delta_container_mut(entity) {
					Some(data) => data.update_with_data(value),
					None => self.insert_delta_container(entity, T::from_data(value)),
				}

				// Remove the removal intent from the entities for removal.
				self.entities_for_removal.remove(&entity);

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
		// Insert the removal intent for the entity.
		self.entities_for_removal.insert(entity);

		// Remove the delta container from the known entities.
		self.known_entities.remove(&entity);
	}
}

/*impl<D: DeltasContainer<C>, C: Sized> DraftBufferlike<ContainerEntity, ContainerEntityBuffer<C>>
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

	/// Commits the draft buffer to the main buffer.
	///
	/// As a last resort, we will insert entities which were known in the draft buffer,
	/// but not in the main buffer. Typically, this will occur because
	/// draft buffer conflicts have not been resolves. Higher-order
	/// systems are responsible for resolving these conflicts.
	/// At this point, if we see an entity in the draft buffer, but not in the main buffer,
	/// we just create a new entity with the corresponding delta in the main buffer.
	fn commit(self, buffer: &mut ContainerEntityBuffer<C>) {
		// Apply all deltas to the containers in the buffer.
		for (entity, deltas) in self.entities.into_iter() {
			match buffer.get_mut(entity) {
				Some(container) => deltas.apply_deltas(container),
				None => {
					buffer.insert_container(deltas.into_container());
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
}*/
