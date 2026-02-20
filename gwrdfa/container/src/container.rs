pub mod tuple;

#[derive(Debug)]
pub enum Component<T: Sized> {
	Present(T),
	Absent,
}

impl<T: Sized> Default for Component<T> {
	fn default() -> Self {
		Self::Absent
	}
}

impl<T: Sized> Component<T> {
	pub fn new(data: T) -> Self {
		Self::Present(data)
	}

	pub fn as_ref(&self) -> Component<&T> {
		match self {
			Self::Present(data) => Component::Present(data),
			Self::Absent => Component::Absent,
		}
	}
}

/// A trait representing valid containers for bundles.
pub trait ContainerAccepting<Data: Sized> {
	/// Creates a new container from a bundle.
	fn from_data(data: Data) -> Self;

	/// Updates a container with a bundle.
	fn update_with_data(&mut self, data: Data);

	/// Removes the value from the container.
	fn remove_from_container(&mut self);
}

/// A trait the localizes container holding operations to the call site.
pub trait ContainerHoldingOps: Sized {
	fn from_this_data<Data: Sized>(data: Data) -> Self
	where
		Self: ContainerAccepting<Data>,
	{
		Self::from_data(data)
	}

	fn update_this_with_data<Data: Sized>(&mut self, data: Data)
	where
		Self: ContainerAccepting<Data>,
	{
		self.update_with_data(data)
	}

	fn remove_this<Data: Sized>(&mut self)
	where
		Self: ContainerAccepting<Data>,
	{
		self.remove_from_container()
	}
}

impl<T: Sized> ContainerHoldingOps for T {}

pub trait ContainerGiving<'a, Data: Sized> {
	/// Gets a the bundle from a reference to the container
	fn as_component(&'a self) -> Component<&'a Data>;
}

#[cfg(test)]
pub mod test {
	use super::*;

	#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
	pub struct TestField(pub i32);

	#[derive(Debug, Clone, Default, PartialEq, Eq)]
	pub struct TestContainer {
		num: i32,
		slice: [i32; 10],
		field: Component<TestField>,
	}

	impl TestContainer {
		pub fn new(num: i32, slice: [i32; 10], field: Component<TestField>) -> Self {
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

		pub fn with_field(mut self, field: Component<TestField>) -> Self {
			self.field = field;
			self
		}
	}

	impl ContainerGiving<'_, i32> for TestContainer {
		fn as_component(&'_ self) -> Component<&'_ i32> {
			Component::Present(&self.num)
		}
	}

	impl ContainerAccepting<i32> for TestContainer {
		fn from_data(data: i32) -> Self {
			Self { num: data, slice: [0; 10], field: Component::Absent }
		}

		fn update_with_data(&mut self, data: i32) {
			self.num = data;
		}

		fn remove_from_container(&mut self) {
			// not possible to remove, data remains unchanged
		}
	}

	impl ContainerGiving<'_, [i32; 10]> for TestContainer {
		fn as_component(&'_ self) -> Component<&'_ [i32; 10]> {
			Component::Present(&self.slice)
		}
	}

	impl ContainerAccepting<[i32; 10]> for TestContainer {
		fn from_data(data: [i32; 10]) -> Self {
			Self { num: 0, slice: data, field: Component::Absent }
		}

		fn update_with_data(&mut self, data: [i32; 10]) {
			self.slice = data;
		}

		fn remove_from_container(&mut self) {
			// not possible to remove, data remains unchanged
		}
	}

	impl<'a> ContainerGiving<'a, TestField> for TestContainer {
		fn as_component(&'a self) -> Component<&'a TestField> {
			self.field.as_ref()
		}
	}

	impl ContainerAccepting<TestField> for TestContainer {
		fn from_data(data: TestField) -> Self {
			Self { num: 0, slice: [0; 10], field: Component::Present(data) }
		}

		fn update_with_data(&mut self, data: TestField) {
			self.field = Component::Present(data);
		}

		fn remove_from_container(&mut self) {
			// set the field to none
			self.field = Component::Absent;
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
		container.field = Component::Present(TestField(1));

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
