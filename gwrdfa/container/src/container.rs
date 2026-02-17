pub mod tuple;
use parabyzantine::buffer::JustEntity;

/// A trait representing valid containers for bundles.
pub trait ContainerHolding<B: Sized> {
	/// Creates a new container from a bundle.
	fn from_data(data: B) -> Self;

	/// Updates a container with a bundle.
	fn update_with_data(&mut self, data: B);

	/// Removes the value from the container.
	fn remove_from_container(&mut self);
}

/// A trait the localizes container holding operations to the call site.
pub trait ContainerHoldingOps: Sized {
	fn from_this_data<B: Sized>(data: B) -> Self
	where
		Self: ContainerHolding<B>,
	{
		Self::from_data(data)
	}

	fn update_this_with_data<B: Sized>(&mut self, data: B)
	where
		Self: ContainerHolding<B>,
	{
		self.update_with_data(data)
	}

	fn remove_this<B: Sized>(&mut self)
	where
		Self: ContainerHolding<B>,
	{
		self.remove_from_container()
	}
}

impl<T: Sized> ContainerHoldingOps for T {}

pub trait ContainerGiving<'a, B: Sized> {
	/// Gets a the bundle from a reference to the container
	fn as_item(&'a self) -> B;
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

	fn remove_from_container(&mut self) {
		// do nothing
		// for now the user should remove the whole entity
	}
}

/// Provisional API to replace option usage.
///
/// This would be a semantically-specific type to mark a field as present or absent.
#[derive(Debug, Clone)]
pub enum ContainerComponent<T: Sized> {
	Present(T),
	Absent,
}

impl<T: Sized> ContainerComponent<T> {
	pub fn new(data: T) -> Self {
		Self::Present(data)
	}

	pub fn as_ref(&self) -> ContainerComponent<&T> {
		match self {
			Self::Present(data) => ContainerComponent::Present(data),
			Self::Absent => ContainerComponent::Absent,
		}
	}
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

impl<'a, T: ContainerGiving<'a, B> + Sized, B> ContainerGiving<'a, ContainerComponent<B>> for T {
	fn as_item(&'a self) -> ContainerComponent<B> {
		ContainerComponent::Present(self.as_item())
	}
}

#[cfg(test)]
pub mod test {
	use super::*;

	#[derive(Debug, Clone, Default, PartialEq, Eq)]
	pub struct TestField(pub i32);

	#[derive(Debug, Clone, Default, PartialEq, Eq)]
	pub struct TestContainer {
		num: i32,
		slice: [i32; 10],
		field: Option<TestField>,
	}

	impl TestContainer {
		pub fn new(num: i32, slice: [i32; 10], field: Option<TestField>) -> Self {
			Self { num, slice, field }
		}

		pub fn with_num(mut self, num: i32) -> Self {
			self.num = num;
			self
		}

		pub fn with_slice(mut self, slice: [i32; 10]) -> Self {
			self.slice = slice;
			self
		}

		pub fn with_field(mut self, field: Option<TestField>) -> Self {
			self.field = field;
			self
		}
	}

	impl ContainerGiving<'_, i32> for TestContainer {
		fn as_item(&'_ self) -> i32 {
			self.num
		}
	}

	impl ContainerHolding<i32> for TestContainer {
		fn from_data(data: i32) -> Self {
			Self { num: data, slice: [0; 10], field: None }
		}

		fn update_with_data(&mut self, data: i32) {
			self.num = data;
		}

		fn remove_from_container(&mut self) {
			// not possible to remove, data remains unchanged
		}
	}

	impl<'a> ContainerGiving<'a, &'a [i32]> for TestContainer {
		fn as_item(&'a self) -> &'a [i32] {
			&self.slice
		}
	}

	impl ContainerHolding<[i32; 10]> for TestContainer {
		fn from_data(data: [i32; 10]) -> Self {
			Self { num: 0, slice: data, field: None }
		}

		fn update_with_data(&mut self, data: [i32; 10]) {
			self.slice = data;
		}

		fn remove_from_container(&mut self) {
			// not possible to remove, data remains unchanged
		}
	}

	impl<'a> ContainerGiving<'a, Option<&'a TestField>> for TestContainer {
		fn as_item(&'a self) -> Option<&'a TestField> {
			self.field.as_ref()
		}
	}

	impl ContainerHolding<TestField> for TestContainer {
		fn from_data(data: TestField) -> Self {
			Self { num: 0, slice: [0; 10], field: Some(data) }
		}

		fn update_with_data(&mut self, data: TestField) {
			self.field = Some(data);
		}

		fn remove_from_container(&mut self) {
			// set the field to none
			self.field = None;
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

	#[test]
	fn test_container_holding() {
		let mut container = TestContainer::default();
		container.update_with_data(1);
		assert_eq!(container.num, 1);

		container.update_with_data([1; 10]);
		assert_eq!(container.slice, [1; 10]);
	}

	#[test]
	fn test_container_holding_optional() {
		let mut container = TestContainer::default();
		container.update_with_data(TestField(1));
		assert_eq!(container.field, Some(TestField(1)));

		container.remove_this::<TestField>();
		assert_eq!(container.field, None);
	}
}
