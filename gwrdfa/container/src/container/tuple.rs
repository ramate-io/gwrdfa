//! This module will contain tuple generalizations of [ContainerHolding] and [ContainerGiving].
use super::{ContainerHoldingOps, ContainerStores};

impl<Container: ContainerStores<A> + ContainerStores<B>, A, B> ContainerStores<(A, B)>
	for Container
{
	fn from_data(data: (A, B)) -> Self {
		let mut container = Container::from_data(data.0);
		container.update_with_data(data.1);
		container
	}

	fn from_removed_data() -> Self {
		let mut container = <Container as ContainerStores<A>>::from_removed_data();
		container.remove_this::<B>();
		container
	}

	fn update_with_data(&mut self, data: (A, B)) {
		self.update_with_data(data.0);
		self.update_with_data(data.1);
	}

	fn remove_from_container(&mut self) {
		self.remove_this::<A>();
		self.remove_this::<B>();
	}
}

impl<Container: ContainerStores<A> + ContainerStores<B> + ContainerStores<C>, A, B, C>
	ContainerStores<(A, B, C)> for Container
{
	fn from_data(data: (A, B, C)) -> Self {
		let mut container = Container::from_data(data.0);
		container.update_with_data(data.1);
		container.update_with_data(data.2);
		container
	}

	fn from_removed_data() -> Self {
		let mut container = <Container as ContainerStores<A>>::from_removed_data();
		container.remove_this::<B>();
		container.remove_this::<C>();
		container
	}

	fn update_with_data(&mut self, data: (A, B, C)) {
		self.update_with_data(data.0);
		self.update_with_data(data.1);
		self.update_with_data(data.2);
	}

	fn remove_from_container(&mut self) {
		self.remove_this::<A>();
		self.remove_this::<B>();
		self.remove_this::<C>();
	}
}

impl<
		Container: ContainerStores<A> + ContainerStores<B> + ContainerStores<C> + ContainerStores<D>,
		A,
		B,
		C,
		D,
	> ContainerStores<(A, B, C, D)> for Container
{
	fn from_data(data: (A, B, C, D)) -> Self {
		let mut container = Container::from_data(data.0);
		container.update_with_data(data.1);
		container.update_with_data(data.2);
		container.update_with_data(data.3);
		container
	}

	fn from_removed_data() -> Self {
		let mut container = <Container as ContainerStores<A>>::from_removed_data();
		container.remove_this::<B>();
		container.remove_this::<C>();
		container.remove_this::<D>();
		container
	}

	fn update_with_data(&mut self, data: (A, B, C, D)) {
		self.update_with_data(data.0);
		self.update_with_data(data.1);
		self.update_with_data(data.2);
		self.update_with_data(data.3);
	}

	fn remove_from_container(&mut self) {
		self.remove_this::<A>();
		self.remove_this::<B>();
		self.remove_this::<C>();
		self.remove_this::<D>();
	}
}
