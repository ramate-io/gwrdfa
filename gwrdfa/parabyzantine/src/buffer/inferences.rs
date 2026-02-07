use crate::buffer::{Bundle, DraftBufferlike};

/// Inferences are entities that should be written to or modified in the buffer.
///
/// They act on the draft buffer, and are then committed to the main buffer in a single operation.
/// This a constrained semantic model that ensures atomicity of the commit operation.
#[derive(Debug)]
pub struct Inferences<'a, D: DraftBufferlike> {
	inner: &'a mut D,
}

impl<'a, D: DraftBufferlike> Inferences<'a, D> {
	pub fn new(inner: &'a mut D) -> Self {
		Self { inner }
	}

	pub fn insert<B: Bundle<D::Entity>>(&mut self, bundle: B) {
		self.inner.draft_insert(bundle);
	}

	pub fn remove<B: Bundle<D::Entity>>(&mut self, bundle: B) {
		self.inner.draft_remove(bundle);
	}

	pub fn remove_entity(&mut self, entity: D::Entity) {
		self.inner.draft_remove_entity(entity);
	}
}
