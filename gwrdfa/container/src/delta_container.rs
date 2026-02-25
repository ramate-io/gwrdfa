use crate::Component;

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
	fn apply(self, component: &mut Component<T>) {
		match self {
			Self::Modified(data) => *component = Component::Present(data),
			Self::Unchanged => (),
			Self::Removed => *component = Component::Absent,
		}
	}
}

pub trait DeltaContainer<C> {
	/// Applies all deltas to the container.
	fn apply_deltas(&mut self, container: &mut C);

	/// Builds a new container from the deltas.
	fn into_container(self) -> C;
}
