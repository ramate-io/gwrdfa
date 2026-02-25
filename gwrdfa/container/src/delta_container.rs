use crate::{
	Component, ContainerAccepting, ContainerEntity, ContainerEntityBuffer, ContainerGiving,
};
use core::mem;
use parabyzantine::buffer::{DraftBufferlike, Stores};

/// A Delta indicates a change on a container.
///
/// While we don't use this directly herein,
/// it is useful when implement custom [DeltaContainer] types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Delta<T: Sized> {
	Modified(T),
	Unchanged,
	Removed,
}

impl<T: Sized> Delta<T> {
	fn apply(&mut self, component: &mut Component<T>) {
		let owned = mem::replace(self, Self::Unchanged);
		match owned {
			Self::Modified(data) => *component = Component::Present(data),
			Self::Unchanged => (),
			Self::Removed => *component = Component::Absent,
		}
	}
}

pub trait ContainerModifying<T: Sized> {
	fn component_mut(&mut self) -> &mut Component<T>;
}

pub trait DeltaContainerGiving<T: Sized> {
	fn delta_mut(&mut self) -> &mut Delta<T>;
}

pub trait DeltaContainerToContainer<T: Sized, C> {
	fn apply_delta(&mut self, container: &mut C);
}

impl<T: Sized, D: DeltaContainerGiving<T>, C: ContainerModifying<T>> DeltaContainerToContainer<T, C>
	for D
{
	fn apply_delta(&mut self, container: &mut C) {
		self.delta_mut().apply(container.component_mut());
	}
}

pub trait DeltaContainer<C> {
	/// Applies all deltas to the container.
	fn apply_deltas(&mut self, container: &mut C);
}
