use crate::{Broadcast, GossamerMessage, GossamerMessageError, In, InFlight, Out};
use gwrdfa_container::{Component, ContainerGiving};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GossamerContainer<T: GossamerMessage> {
	pub message: Component<T>,
	pub message_in: Component<In>,
	pub message_out: Component<Out>,
	pub message_in_flight: Component<InFlight>,
	pub message_broadcast: Component<Broadcast>,
	pub message_error: Component<GossamerMessageError>,
}

impl<T: GossamerMessage> ContainerGiving<T> for GossamerContainer<T> {
	fn as_component(&self) -> Component<&T> {
		self.message.as_ref()
	}
}

impl<T: GossamerMessage> ContainerGiving<In> for GossamerContainer<T> {
	fn as_component(&self) -> Component<&In> {
		self.message_in.as_ref()
	}
}

impl<T: GossamerMessage> ContainerGiving<Out> for GossamerContainer<T> {
	fn as_component(&self) -> Component<&Out> {
		self.message_out.as_ref()
	}
}

impl<T: GossamerMessage> ContainerGiving<InFlight> for GossamerContainer<T> {
	fn as_component(&self) -> Component<&InFlight> {
		self.message_in_flight.as_ref()
	}
}

impl<T: GossamerMessage> ContainerGiving<Broadcast> for GossamerContainer<T> {
	fn as_component(&self) -> Component<&Broadcast> {
		self.message_broadcast.as_ref()
	}
}

impl<T: GossamerMessage> ContainerGiving<GossamerMessageError> for GossamerContainer<T> {
	fn as_component(&self) -> Component<&GossamerMessageError> {
		self.message_error.as_ref()
	}
}
