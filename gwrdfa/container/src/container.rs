/// A trait representing types that are containable in a particular container.
///
/// Generally, this is what you will want to use to implement container bindings.
/// More often, you will have crate-local types that are containable in a particular container,
/// while the container may come from elsewhere.
pub trait ContainableIn<T: Sized> {
	fn into_container(self) -> T;

	fn update_container(self, container: &mut T);

	fn from_container(container: &T) -> Self;
}

/// A trait representing valid conntainers for bundles.
pub trait Container<B: Sized> {
	/// Creates a new container from a bundle.
	fn from_bundle(bundle: B) -> Self;

	/// Updates a container with a bundle.
	fn update_with_bundle(&mut self, bundle: B);

	/// Gets a the bundle from a reference to the container
	fn as_bundle(&self) -> B;
}

/// All types that are containable in a type induce a container on that type.
impl<T: Sized, B: ContainableIn<T>> Container<B> for T {
	fn from_bundle(bundle: B) -> Self {
		bundle.into_container()
	}

	fn update_with_bundle(&mut self, bundle: B) {
		bundle.update_container(self);
	}

	fn as_bundle(&self) -> B {
		B::from_container(self)
	}
}
