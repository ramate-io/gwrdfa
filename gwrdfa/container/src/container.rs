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
impl<'a, T: Sized, B: OnContainer<'a, &'a T>> ContainerGiving<'a, B> for &'a T {
	fn as_item(&'a self) -> B {
		B::container_as(self)
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
