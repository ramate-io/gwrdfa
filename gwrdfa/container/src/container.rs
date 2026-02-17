use parabyzantine::buffer::JustEntity;

/// A trait representing types that are containable in a particular container.
///
/// Generally, this is what you will want to use to implement container bindings.
/// More often, you will have crate-local types that are containable in a particular container,
/// while the container may come from elsewhere.
pub trait IntoContainer<T: Sized> {
	fn into_container(self) -> T;

	fn update_container(self, container: &mut T);

	fn remove_from_container(container: &mut T);
}

pub trait OnContainer<'a, T: Sized> {
	fn container_as(container: &'a T) -> Self;
}

/// A trait representing valid containers for bundles.
pub trait ContainerHolding<B: Sized> {
	/// Creates a new container from a bundle.
	fn from_bundle(bundle: B) -> Self;

	/// Updates a container with a bundle.
	fn update_with_bundle(&mut self, bundle: B);

	/// Removes the value from the container.
	fn remove_from_container(&mut self);
}

pub trait ContainerGiving<'a, B: Sized> {
	/// Gets a the bundle from a reference to the container
	fn as_item(&'a self) -> B;
}

/// All types that are containable in a type induce a container on that type.
impl<T: Sized, B: IntoContainer<T>> ContainerHolding<B> for T {
	fn from_bundle(bundle: B) -> Self {
		bundle.into_container()
	}

	fn update_with_bundle(&mut self, bundle: B) {
		bundle.update_container(self);
	}

	fn remove_from_container(&mut self) {
		B::remove_from_container(self);
	}
}

/// All types that are containable in a type induce a container on that type.
impl<'a, T: Sized, B: OnContainer<'a, T>> ContainerGiving<'a, B> for &'a T {
	fn as_item(&'a self) -> B {
		B::container_as(self)
	}
}

/// All container types that have a container giving themselves also have a container giving an optional version of themselves.
///
/// This is a hard-baked container semantics.
/// But doing this, we disallow aggregating fields to determine whether
/// a certain type is present or not.
impl<'a, T: Sized, B: OnContainer<'a, T>> OnContainer<'a, T> for Option<B> {
	fn container_as(container: &'a T) -> Self {
		Some(B::container_as(container))
	}
}

/// [JustEntity] is trivially containable in any type.
impl<'a, T: Default + Sized> OnContainer<'a, T> for JustEntity {
	fn container_as(_container: &'a T) -> Self {
		JustEntity
	}
}

/// All container types can contain themselves.
impl<T: Sized> IntoContainer<T> for T {
	fn into_container(self) -> T {
		self
	}

	fn update_container(self, container: &mut T) {
		*container = self;
	}

	fn remove_from_container(_container: &mut T) {
		// do nothing
		// for now the user should remove the whole entity
	}
}

/// All container types can give a refernence to themselves.
impl<'a, T: Sized> OnContainer<'a, T> for &'a T {
	fn container_as(container: &'a T) -> Self {
		container
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[derive(Debug, Clone, Default, PartialEq, Eq)]
	struct TestContainer {
		num: i32,
		slice: [i32; 10],
	}

	impl OnContainer<'_, TestContainer> for i32 {
		fn container_as(container: &'_ TestContainer) -> Self {
			container.num
		}
	}

	impl<'a> OnContainer<'a, TestContainer> for &'a [i32] {
		fn container_as(container: &'a TestContainer) -> &'a [i32] {
			&container.slice
		}
	}

	#[test]
	fn test_container_giving() {
		// Allocate the container
		let container = &TestContainer::default();

		// Get the num from the container
		let num: i32 = container.as_item();
		assert_eq!(num, container.num);

		// Get the slice from the container
		let slice: &[i32] = container.as_item();
		assert_eq!(slice, &container.slice);

		// Get the container itself
		let container: &TestContainer = container.as_item();
		assert_eq!(container, &TestContainer { num: 0, slice: [0; 10] });
	}
}
