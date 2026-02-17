//! This module will contain tuple generalizations of [ContainerHolding] and [ContainerGiving].
use super::{ContainerGiving, ContainerHolding, ContainerHoldingOps};

impl<Container: ContainerHolding<A> + ContainerHolding<B>, A, B> ContainerHolding<(A, B)>
	for Container
{
	fn from_data(data: (A, B)) -> Self {
		let mut container = Container::from_data(data.0);
		container.update_with_data(data.1);
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

impl<'a, Container: ContainerGiving<'a, A> + ContainerGiving<'a, B>, A, B>
	ContainerGiving<'a, (A, B)> for Container
{
	fn as_item(&'a self) -> (A, B) {
		(self.as_item(), self.as_item())
	}
}
