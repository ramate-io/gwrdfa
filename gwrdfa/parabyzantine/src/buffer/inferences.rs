use crate::buffer::{Bundle, DraftBufferlike};
use core::marker::PhantomData;

/// Inferences are entities that should be written to or modified in the buffer.
///
/// They act on the draft buffer, and are then committed to the main buffer in a single operation.
/// This a constrained semantic model that ensures atomicity of the commit operation.
#[derive(Debug)]
pub struct Inferences<'a, Entity: Sized, D: DraftBufferlike<Entity>> {
	inner: &'a mut D,
	__phantom: PhantomData<Entity>,
}

impl<'a, Entity: Sized, D: DraftBufferlike<Entity>> Inferences<'a, Entity, D> {
	pub fn new(inner: &'a mut D) -> Self {
		Self { inner, __phantom: PhantomData }
	}

	pub fn insert<B: Bundle<Entity>>(&mut self, entity: Option<Entity>, bundle: B) {
		self.inner.draft_insert(entity, bundle);
	}

	pub fn remove<B: Bundle<Entity>>(&mut self, entity: Entity, bundle: B) {
		self.inner.draft_remove(entity, bundle);
	}

	pub fn remove_entity(&mut self, entity: Entity) {
		self.inner.draft_remove_entity(entity);
	}
}
