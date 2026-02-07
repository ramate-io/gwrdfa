pub mod facts;
pub mod inferences;

pub use facts::Facts;
pub use inferences::Inferences;

/// Bundles are entities plus metadata that exist in the buffer.
pub trait Bundle<T> {
	/// Gets the entity of the bundle.
	///
	/// Entites should be ownable, global references.
	fn entity(&self) -> T;
}

/// Buffers store entities and components associated with those entities.
/// They are the ultimate source of truth for the system.
///
/// Using them should mostly be encapsulated behind [Facts]
pub trait Bufferlike: Sized {
	type Entity: Sized;

	fn query<B: Bundle<Self::Entity>>(&self) -> impl Iterator<Item = B>;

	fn insert<B: Bundle<Self::Entity>>(&mut self, bundle: B);

	fn remove<B: Bundle<Self::Entity>>(&mut self, bundle: B);

	fn remove_entity(&mut self, entity: Self::Entity);
}

/// Draft buffers are buffers that are not yet committed to the main buffer.
/// Often this will simply be a reference to the same type as the main buffer
/// which is then swapped out for the main buffer.
///
/// Using them should mostly be encapsulated behind [Inferences]
pub trait DraftBufferlike: Sized {
	type Entity: Sized;

	fn draft_insert<B: Bundle<Self::Entity>>(&mut self, bundle: B);

	fn draft_remove<B: Bundle<Self::Entity>>(&mut self, bundle: B);

	fn draft_remove_entity(&mut self, entity: Self::Entity);

	fn commit(&mut self, buffer: &mut impl Bufferlike<Entity = Self::Entity>);
}
