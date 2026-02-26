use crate::Component;

/// A Delta indicates a change on a container.
///
/// While we don't use this directly herein,
/// it is useful when implement custom [DeltasContainer] types.
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

impl<T: Sized + Default> Delta<T> {
	/// Applies the delta to a type.
	///
	/// This is useful when the type is not available, i.e., you provide a
	/// delta which represents a new type.
	pub fn apply_to_type(self, container_data: &mut T) {
		match self {
			Self::Modified(data) => *container_data = data,
			Self::Unchanged => (),
			Self::Removed => *container_data = Default::default(),
		}
	}

	/// Converts the delta to a type.
	///
	/// This is useful when the type is not available, i.e., you provide a
	/// delta which represents a new type.
	pub fn into_type(self) -> T {
		match self {
			Self::Modified(data) => data,
			Self::Unchanged => Default::default(),
			Self::Removed => Default::default(),
		}
	}
}

/// Delta containers know how to apply themselves to a container.
///
/// Again, note that there is no requirement that the [Delta] API,
/// be used here. A [DeltasContainer] can decide to apply itself to a container
/// in any way it seems fit.
///
/// Further, obeserve that if we wanted to have blanket implementation,
/// we would need to enumerate all the possible deltas on
/// a [DeltasContainer] type and enumerate all the possible
/// components on a Container type. Rust does not yet support this kind
/// of pattern matching.
pub trait DeltasContainer<C> {
	/// Applies all deltas to the container.
	fn apply_deltas(self, container: &mut C);

	/// Builds a new container from the deltas.
	fn into_container(self) -> C;
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::container::test::{TestContainer, TestField};

	#[derive(Debug, Clone, Default, PartialEq, Eq)]
	pub struct TestDeltasContainer {
		pub num: Delta<i32>,
		pub slice: Delta<[i32; 10]>,
		pub field: Delta<TestField>,
	}

	impl DeltasContainer<TestContainer> for TestDeltasContainer {
		fn apply_deltas(self, container: &mut TestContainer) {
			self.num.apply_to_type(&mut container.num);
			self.slice.apply_to_type(&mut container.slice);
			self.field.apply(&mut container.field);
		}

		fn into_container(self) -> TestContainer {
			TestContainer {
				num: self.num.into_type(),
				slice: self.slice.into_type(),
				field: self.field.into_component(),
			}
		}
	}

	#[test]
	fn test_delta_container_modify() {
		let deltas = TestDeltasContainer { num: Delta::Modified(1), ..Default::default() };
		let mut container = TestContainer::default();
		deltas.apply_deltas(&mut container);
		assert_eq!(container, TestContainer { num: 1, slice: [0; 10], field: Component::Absent });
	}

	#[test]
	fn test_delta_container_unchanged() {
		let deltas = TestDeltasContainer { ..Default::default() };
		let mut container =
			TestContainer { num: 1, slice: [0; 10], field: Component::Present(TestField(1)) };
		deltas.apply_deltas(&mut container);
		assert_eq!(
			container,
			TestContainer { num: 1, slice: [0; 10], field: Component::Present(TestField(1)) }
		);
	}

	#[test]
	fn test_delta_container_remove() {
		let deltas = TestDeltasContainer { field: Delta::Removed, ..Default::default() };
		let mut container =
			TestContainer { num: 1, slice: [0; 10], field: Component::Present(TestField(1)) };
		deltas.apply_deltas(&mut container);
		assert_eq!(container, TestContainer { num: 1, slice: [0; 10], field: Component::Absent });
	}
}
