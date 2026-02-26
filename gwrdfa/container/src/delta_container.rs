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

impl<T: Sized> Default for Delta<T> {
	fn default() -> Self {
		Self::Unchanged
	}
}

impl<T: Sized> Delta<T> {
	/// Applies the delta to a component.
	///
	/// Deltas are consume when they are applied to a component.
	pub fn apply(self, component: &mut Component<T>) {
		match self {
			Self::Modified(data) => *component = Component::Present(data),
			Self::Unchanged => (),
			Self::Removed => *component = Component::Absent,
		}
	}

	/// Converts the delta to a component.
	///
	/// This is useful when the component is not available, i.e., you provide a
	/// delta which represents a new component.
	pub fn into_component(self) -> Component<T> {
		match self {
			Self::Modified(data) => Component::Present(data),
			Self::Unchanged => Component::Absent,
			Self::Removed => Component::Absent,
		}
	}
}

/// Delta containers know how to apply themselves to a container.
///
/// Again, note that there is no requirement that the [Delta] API,
/// be used here. A [DeltaContainer] can decide to apply itself to a container
/// in any way it seems fit.
///
/// Further, obeserve that if we wanted to have blanket implementation,
/// we would need to enumerate all the possible deltas on
/// a [DeltaContainer] type and enumerate all the possible
/// components on a Container type. Rust does not yet support this kind
/// of pattern matching.
pub trait DeltaContainer<C> {
	/// Applies all deltas to the container.
	fn apply_deltas(self, container: &mut C);

	/// Builds a new container from the deltas.
	fn into_container(self) -> C;
}
