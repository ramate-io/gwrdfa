use parabyzantine::buffer::JustEntity;

/// A trait representing valid containers for bundles.
pub trait ContainerHolding<B: Sized> {
	/// Creates a new container from a bundle.
	fn from_data(data: B) -> Self;

	/// Updates a container with a bundle.
	fn update_with_data(&mut self, data: B);

	/// Removes the value from the container.
	fn remove_from_data(&mut self);
}

pub trait ContainerGiving<'a, B: Sized> {
	/// Gets a the bundle from a reference to the container
	fn as_item(&'a self) -> B;
}

/// All container types that have a container giving themselves also have a container giving an optional version of themselves.
///
/// This is a hard-baked container semantics.
/// But doing this, we disallow aggregating fields to determine whether
/// a certain type is present or not.
///
/// This also prohibits overloading types,
/// as you will get conflicting implementations of the `OnContainer` trait.
/// Just as in an ECS, you're going to want to use different types for different fields.
impl<'a, T: ContainerGiving<'a, B> + Sized, B> ContainerGiving<'a, Option<B>> for T {
	fn as_item(&'a self) -> Option<B> {
		Some(self.as_item())
	}
}

/// [JustEntity] is trivially containable in any type.
impl<'a, T: Default + Sized> ContainerGiving<'a, JustEntity> for T {
	fn as_item(&'a self) -> JustEntity {
		JustEntity
	}
}

/// All container types can contain themselves.
impl<T: Sized> ContainerHolding<T> for T {
	fn from_data(data: T) -> Self {
		data
	}

	fn update_with_data(&mut self, data: T) {
		*self = data;
	}

	fn remove_from_data(&mut self) {
		// do nothing
		// for now the user should remove the whole entity
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[derive(Debug, Clone, Default, PartialEq, Eq)]
	pub struct TestField(pub i32);

	#[derive(Debug, Clone, Default, PartialEq, Eq)]
	struct TestContainer {
		num: i32,
		slice: [i32; 10],
		field: Option<TestField>,
	}

	impl ContainerGiving<'_, i32> for TestContainer {
		fn as_item(&'_ self) -> i32 {
			self.num
		}
	}

	impl<'a> ContainerGiving<'a, &'a [i32]> for TestContainer {
		fn as_item(&'a self) -> &'a [i32] {
			&self.slice
		}
	}

	impl<'a> ContainerGiving<'a, Option<&'a TestField>> for TestContainer {
		fn as_item(&'a self) -> Option<&'a TestField> {
			self.field.as_ref()
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
	}

	#[test]
	fn test_container_giving_optional() {
		// Allocate the container
		let mut container = TestContainer::default();
		container.field = Some(TestField(1));

		let container = &container;

		// Get the field from the container
		let field: Option<&TestField> = container.as_item();
		assert_eq!(field, Some(&TestField(1)));

		// Get the num as an option
		let num: Option<i32> = container.as_item();
		assert_eq!(num, Some(0));

		// Get the slice as an option
		let slice: Option<&[i32]> = container.as_item();
		assert_eq!(slice, Some(&container.slice[..]));
	}
}
