pub mod facts;
pub mod inferences;

pub use facts::Facts;
pub use inferences::Inferences;

use crate::NoOp;

/// Bundles are entities plus metadata that exist in the buffer.
pub trait Bundle<Entity, Buf: Bufferlike<Entity>> {
	/// Inserts a bundle into a buffer.
	fn insert_into(self, buffer: &mut Buf, entity: Option<Entity>);

	/// Removes a bundle from a buffer.
	fn remove_from(self, buffer: &mut Buf, entity: Entity);
}

/// Marks when a bundle is just an entity.
///
/// This is the canonical way to:
/// 1. Query a buffer for only entities.
/// 2. Handle ignorant buffers (buffers that don't support extended bundle semantics).
///
/// Ignorant buffers are a useful pattern for simple or highly constrained systems,
/// wherein implementing bundle arenas is not worthwhile or tractable.
///
/// The struct itself does not carry meaninful data. It is simply a marker type.
#[derive(Debug, Clone, Copy)]
pub struct JustEntity;

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
	fn insert<B: Bundle<Entity, Self>>(&mut self, entity: Option<Entity>, bundle: B) {
		bundle.insert_into(self, entity);
	}

	fn remove<B: Bundle<Entity, Self>>(&mut self, entity: Entity, bundle: B) {
		bundle.remove_from(self, entity);
	}

	fn remove_entity(&mut self, entity: Entity);

	fn commit_inferences<D: DraftBufferlike<Entity, Self>>(
		&mut self,
		inferences: Inferences<Entity, Self, D>,
	) {
		let mut draft_buffer = inferences.into_inner();
		draft_buffer.commit(self);
	}
}

/// Query plans are used to build queries over the buffer.
pub trait QueryPlanlike<
	Entity: Sized,
	Buffer: Bufferlike<Entity>,
	B: Bundle<Entity, Buffer>,
	Query: Querylike<Entity, Buffer, B>,
>
{
	fn build<'a>(self, buffer: &'a Buffer) -> Query
	where
		Query: 'a;
}

/// Queries are view helpers that are used to look at values in the buffer.
/// They can also perform logical optimizations w.r.t. the buffer and the intended types or values.
///
/// They are constructed from indexes and contain bespoke ways to work with the buffer.
pub trait Querylike<Entity: Sized, Buffer: Bufferlike<Entity>, B: Bundle<Entity, Buffer>> {
	fn next(&mut self) -> Option<(Entity, B)>;

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
pub trait DraftBufferlike<Entity: Sized, Buffer: Bufferlike<Entity>>: Sized {
	/// Inserts a bundle into the draft buffer.
	///
	/// Canonically, inserting Self::Entity as a bundle is equivalent to upserting the entity itself.
	/// Systems which are not capable of complex bundle semantics may use this pattern to achieve
	/// data augmentation.
	fn draft_insert<B: Bundle<Entity, Buffer>>(&mut self, entity: Option<Entity>, bundle: B);

	/// Removes a bundle from the draft buffer.
	fn draft_remove<B: Bundle<Entity, Buffer>>(&mut self, entity: Entity, bundle: B);

	/// Removes an entity from the draft buffer.
	fn draft_remove_entity(&mut self, entity: Entity);

	/// Commits the draft buffer to the main buffer.
	fn commit(&mut self, buffer: &mut Buffer);
}

impl<Entity: Sized> Bufferlike<Entity> for NoOp {
	fn insert<B: Bundle<Entity, Self>>(&mut self, _entity: Option<Entity>, _bundle: B) {}

	fn remove<B: Bundle<Entity, Self>>(&mut self, _entity: Entity, _bundle: B) {}

	fn remove_entity(&mut self, _entity: Entity) {}
}

impl<Entity: Sized, Buffer: Bufferlike<Entity>> DraftBufferlike<Entity, Buffer> for NoOp {
	fn draft_insert<B: Bundle<Entity, Buffer>>(&mut self, _entity: Option<Entity>, _bundle: B) {}

	fn draft_remove<B: Bundle<Entity, Buffer>>(&mut self, _entity: Entity, _bundle: B) {}

	fn draft_remove_entity(&mut self, _entity: Entity) {}

	fn commit(&mut self, _buffer: &mut Buffer) {}
}

impl<Entity: Sized, Buffer: Bufferlike<Entity>, B: Bundle<Entity, Buffer>>
	Querylike<Entity, Buffer, B> for NoOp
{
	fn next(&mut self) -> Option<(Entity, B)> {
		None
	}

	fn get(&self, _entity: Entity) -> Option<B> {
		None
	}
}

impl<Entity: Sized, Buffer: Bufferlike<Entity>, B: Bundle<Entity, Buffer>>
	QueryPlanlike<Entity, Buffer, B, NoOp> for NoOp
{
	fn build<'a>(self, _buffer: &'a Buffer) -> NoOp
	where
		NoOp: 'a,
	{
		NoOp
	}
}

impl<Entity, Buffer: Bufferlike<Entity>> Bundle<Entity, Buffer> for NoOp {
	fn insert_into(self, _buffer: &mut Buffer, _entity: Option<Entity>) {
		// do nothing
	}

	fn remove_from(self, _buffer: &mut Buffer, _entity: Entity) {
		// do nothing
	}
}
