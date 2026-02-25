pub mod facts;
pub mod inferences;
pub mod query;

pub use facts::Facts;
pub use inferences::Inferences;

use crate::NoOp;

pub trait Stores<T, Entity> {
	/// Inserts a value into the store.
	fn insert_record(&mut self, entity: Option<Entity>, value: T) -> Option<Entity>;

	/// Removes a value from the store.
	fn remove_record(&mut self, entity: Entity);
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
	fn insert<B>(&mut self, entity: Option<Entity>, bundle: B) -> Option<Entity>
	where
		Self: Stores<B, Entity>,
	{
		self.insert_record(entity, bundle)
	}

	fn remove<B>(&mut self, entity: Entity)
	where
		Self: Stores<B, Entity>,
	{
		self.remove_record(entity);
	}

	fn remove_entity(&mut self, entity: Entity);

	fn commit_inferences<D: DraftBufferlike<Entity, Self>>(
		&mut self,
		inferences: Inferences<Entity, Self, D>,
	) {
		let draft_buffer = inferences.into_inner();
		draft_buffer.commit(self);
	}
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
	fn draft_insert<B>(&mut self, entity: Option<Entity>, bundle: B)
	where
		Buffer: Stores<B, Entity>,
		Self: Stores<B, Entity>;

	/// Removes a bundle from the draft buffer.
	fn draft_remove<B>(&mut self, entity: Entity)
	where
		Buffer: Stores<B, Entity>,
		Self: Stores<B, Entity>;

	/// Removes an entity from the draft buffer.
	fn draft_remove_entity(&mut self, entity: Entity);

	/// Commits the draft buffer to the main buffer.
	fn commit(self, buffer: &mut Buffer);
}

impl<Entity: Sized> Bufferlike<Entity> for NoOp {
	fn insert<B>(&mut self, _entity: Option<Entity>, _bundle: B) -> Option<Entity>
	where
		Self: Stores<B, Entity>,
	{
		None
	}

	fn remove<B>(&mut self, _entity: Entity)
	where
		Self: Stores<B, Entity>,
	{
		// do nothing
	}

	fn remove_entity(&mut self, _entity: Entity) {}
}

impl<Entity: Sized, Buffer: Bufferlike<Entity>> DraftBufferlike<Entity, Buffer> for NoOp {
	fn draft_insert<B>(&mut self, _entity: Option<Entity>, _bundle: B)
	where
		Buffer: Stores<B, Entity>,
	{
		// do nothing
	}

	fn draft_remove<B>(&mut self, _entity: Entity)
	where
		Buffer: Stores<B, Entity>,
	{
		// do nothing
	}

	fn draft_remove_entity(&mut self, _entity: Entity) {}

	fn commit(self, _buffer: &mut Buffer) {}
}
