use crate::{Broadcast, GossamerMessage, GossamerMessageError, In, InFlight, Out};
use gwrdfa_container::{Component, ContainerGiving};

/// Canonical message container used by the Gossamer Hart integration.
///
/// The container keeps the message payload plus lifecycle/error markers as
/// components so Parabyzantine queries can reason over transport state.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GossamerContainer<T: GossamerMessage> {
	/// Message payload component.
	pub message: Component<T>,
	/// Marker indicating this message was received from the network.
	pub message_in: Component<In>,
	/// Marker indicating this message is queued to be sent.
	pub message_out: Component<Out>,
	/// Marker indicating this message has been handed off to Gossamer for publish.
	pub message_in_flight: Component<InFlight>,
	/// Marker indicating Gossamer confirmed publish handling for this message.
	pub message_broadcast: Component<Broadcast>,
	/// Message-level transport/serialization error marker.
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
