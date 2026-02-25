use crate::{Broadcast, GossamerMessage, GossamerMessageError, In, InFlight, Out};
use gwrdfa_container::{Component, ContainerAccepting, ContainerGiving};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GossamerContainer<T: GossamerMessage> {
	pub message: Component<T>,
	pub message_in: Component<In>,
	pub message_out: Component<Out>,
	pub message_in_flight: Component<InFlight>,
	pub message_broadcast: Component<Broadcast>,
	pub message_error: Component<GossamerMessageError>,
}

impl<T: GossamerMessage> ContainerAccepting<T> for GossamerContainer<T> {
	fn from_data(data: T) -> Self {
		Self {
			message: Component::Present(data),
			message_in: Component::Absent,
			message_out: Component::Absent,
			message_in_flight: Component::Absent,
			message_broadcast: Component::Absent,
			message_error: Component::Absent,
		}
	}

	fn update_with_data(&mut self, data: T) {
		self.message = Component::Present(data);
	}

	fn remove_from_container(&mut self) {
		self.message = Component::Absent;
	}
}

impl<T: GossamerMessage> ContainerGiving<T> for GossamerContainer<T> {
	fn as_component(&self) -> Component<&T> {
		self.message.as_ref()
	}
}

impl<T: GossamerMessage> ContainerAccepting<In> for GossamerContainer<T> {
	fn from_data(data: In) -> Self {
		Self {
			message: Component::Absent,
			message_in: Component::Present(data),
			message_out: Component::Absent,
			message_in_flight: Component::Absent,
			message_broadcast: Component::Absent,
			message_error: Component::Absent,
		}
	}

	fn update_with_data(&mut self, data: In) {
		self.message_in = Component::Present(data);
	}

	fn remove_from_container(&mut self) {
		self.message_in = Component::Absent;
	}
}

impl<T: GossamerMessage> ContainerGiving<In> for GossamerContainer<T> {
	fn as_component(&self) -> Component<&In> {
		self.message_in.as_ref()
	}
}

impl<T: GossamerMessage> ContainerAccepting<Out> for GossamerContainer<T> {
	fn from_data(data: Out) -> Self {
		Self {
			message: Component::Absent,
			message_in: Component::Absent,
			message_out: Component::Present(data),
			message_in_flight: Component::Absent,
			message_broadcast: Component::Absent,
			message_error: Component::Absent,
		}
	}

	fn update_with_data(&mut self, data: Out) {
		self.message_out = Component::Present(data);
	}

	fn remove_from_container(&mut self) {
		self.message_out = Component::Absent;
	}
}

impl<T: GossamerMessage> ContainerGiving<Out> for GossamerContainer<T> {
	fn as_component(&self) -> Component<&Out> {
		self.message_out.as_ref()
	}
}

impl<T: GossamerMessage> ContainerAccepting<InFlight> for GossamerContainer<T> {
	fn from_data(data: InFlight) -> Self {
		Self {
			message: Component::Absent,
			message_in: Component::Absent,
			message_out: Component::Absent,
			message_in_flight: Component::Present(data),
			message_broadcast: Component::Absent,
			message_error: Component::Absent,
		}
	}

	fn update_with_data(&mut self, data: InFlight) {
		self.message_in_flight = Component::Present(data);
	}

	fn remove_from_container(&mut self) {
		self.message_in_flight = Component::Absent;
	}
}

impl<T: GossamerMessage> ContainerGiving<InFlight> for GossamerContainer<T> {
	fn as_component(&self) -> Component<&InFlight> {
		self.message_in_flight.as_ref()
	}
}

impl<T: GossamerMessage> ContainerAccepting<Broadcast> for GossamerContainer<T> {
	fn from_data(data: Broadcast) -> Self {
		Self {
			message: Component::Absent,
			message_in: Component::Absent,
			message_out: Component::Absent,
			message_in_flight: Component::Absent,
			message_broadcast: Component::Present(data),
			message_error: Component::Absent,
		}
	}

	fn update_with_data(&mut self, data: Broadcast) {
		self.message_broadcast = Component::Present(data);
	}

	fn remove_from_container(&mut self) {
		self.message_broadcast = Component::Absent;
	}
}

impl<T: GossamerMessage> ContainerGiving<Broadcast> for GossamerContainer<T> {
	fn as_component(&self) -> Component<&Broadcast> {
		self.message_broadcast.as_ref()
	}
}

impl<T: GossamerMessage> ContainerAccepting<GossamerMessageError> for GossamerContainer<T> {
	fn from_data(data: GossamerMessageError) -> Self {
		Self {
			message: Component::Absent,
			message_in: Component::Absent,
			message_out: Component::Absent,
			message_in_flight: Component::Absent,
			message_broadcast: Component::Absent,
			message_error: Component::Present(data),
		}
	}

	fn update_with_data(&mut self, data: GossamerMessageError) {
		self.message_error = Component::Present(data);
	}

	fn remove_from_container(&mut self) {
		self.message_error = Component::Absent;
	}
}

impl<T: GossamerMessage> ContainerGiving<GossamerMessageError> for GossamerContainer<T> {
	fn as_component(&self) -> Component<&GossamerMessageError> {
		self.message_error.as_ref()
	}
}
