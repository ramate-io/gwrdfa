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
/// Buffers represent entities which have been loaded into memory.
/// This is a bridge between the physical and logical layers of the system.
///
/// Systems implementing with buffers should make their own decisions about
/// entities which may be loaded in later.
///
/// Using them should mostly be encapsulated behind [Facts]
pub trait Bufferlike<Entity: Sized>: Sized {
	fn query<
		B: Bundle<Entity>,
		Query: Querylike<Entity, Self, B>,
		I: Indexlike<Entity, Self, B, Query>,
	>(
		&self,
		index: I,
	) -> Query {
		index.query(self)
	}

	fn insert<B: Bundle<Entity>>(&mut self, entity: Option<Entity>, bundle: B);

	fn remove<B: Bundle<Entity>>(&mut self, entity: Entity, bundle: B);

	fn remove_entity(&mut self, entity: Entity);
}

/// Indexes handle building query objects.
///
/// They are used for logical optimizations, i.e.,
/// so that we don't always have to construct queries that literally iterate over the entire buffer.
pub trait Indexlike<
	Entity: Sized,
	Buffer: Bufferlike<Entity>,
	B: Bundle<Entity>,
	Query: Querylike<Entity, Buffer, B>,
>: Sized
{
	fn query(&self, buffer: &Buffer) -> Query;
}

/// Queries are view helpers that are used to look at values in the buffer.
///
/// They are constructed from indexes and contain bespoke ways to work with the buffer.
pub trait Querylike<Entity: Sized, Buffer: Bufferlike<Entity>, B: Bundle<Entity>> {
	fn next(&mut self, buffer: &Buffer) -> Option<(Entity, B)>;

	fn get(&self, entity: Entity) -> Option<B>;
}

/// Draft buffers are buffers that are not yet committed to the main buffer.
/// Often this will simply be a reference to the same type as the main buffer
/// which is then swapped out for the main buffer.
///
/// This separation is important for the atomicity of the commit operation.
///
/// In cases where the implementer must model removals across multiple buffer frames,
/// often it is best to insert a a new component to the bundle which represents safety for removal
/// given a certain global condition is satisfied and then remove in a later stage when that condition is satisfied.
///
/// Using them should mostly be encapsulated behind [Inferences]
pub trait DraftBufferlike<Entity: Sized>: Sized {
	/// Inserts a bundle into the draft buffer.
	///
	/// Canonically, inserting Self::Entity as a bundle is equivalent to upserting the entity itself.
	/// Systems which are not capable of complex bundle semantics may use this pattern to achieve
	/// data augmentation.
	fn draft_insert<B: Bundle<Entity>>(&mut self, entity: Option<Entity>, bundle: B);

	/// Removes a bundle from the draft buffer.
	fn draft_remove<B: Bundle<Entity>>(&mut self, entity: Entity, bundle: B);

	/// Removes an entity from the draft buffer.
	fn draft_remove_entity(&mut self, entity: Entity);

	fn commit(&mut self, buffer: &mut impl Bufferlike<Entity>);
}
