use crate::buffer::{Bufferlike, Bundle, DraftBufferlike};
use core::marker::PhantomData;

/// Inferences are entities that should be written to or modified in the buffer.
///
/// They act on the draft buffer, and are then committed to the main buffer in a single operation.
/// This a constrained semantic model that ensures atomicity of the commit operation.
#[derive(Debug)]
pub struct Inferences<Entity: Sized, Buffer: Bufferlike<Entity>, D: DraftBufferlike<Entity, Buffer>>
{
	inner: D,
	__phantom: PhantomData<(Entity, Buffer)>,
}

impl<Entity: Sized, Buffer: Bufferlike<Entity>, D: DraftBufferlike<Entity, Buffer>>
	Inferences<Entity, Buffer, D>
{
	pub fn new(inner: D) -> Self {
		Self { inner, __phantom: PhantomData }
	}

	/// Inserts an entity and a bundle into the inferences.
	pub fn insert<B: Bundle<Entity, Buffer>>(&mut self, entity: Option<Entity>, bundle: B) {
		self.inner.draft_insert(entity, bundle);
	}

	/// Removes an entity and a bundle from the inferences.
	pub fn remove<B: Bundle<Entity, Buffer>>(&mut self, entity: Entity) {
		self.inner.draft_remove::<B>(entity);
	}

	/// Removes an entity from the inferences.
	pub fn remove_entity(&mut self, entity: Entity) {
		self.inner.draft_remove_entity(entity);
	}

	pub(crate) fn into_inner(self) -> D {
		self.inner
	}
}

impl<Entity: Sized, Buffer: Bufferlike<Entity>, D: DraftBufferlike<Entity, Buffer>> From<D>
	for Inferences<Entity, Buffer, D>
{
	fn from(inner: D) -> Self {
		Inferences::new(inner)
	}
}
