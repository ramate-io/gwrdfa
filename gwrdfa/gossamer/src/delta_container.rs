use crate::{
	container::GossamerContainer, Broadcast, GossamerMessage, GossamerMessageError, In, InFlight,
	Out,
};
use gwrdfa_container::{ContainerAccepting, Delta, DeltaContainer};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GossamerDeltaContainer<T: GossamerMessage> {
	pub message: Delta<T>,
	pub message_in: Delta<In>,
	pub message_out: Delta<Out>,
	pub message_in_flight: Delta<InFlight>,
	pub message_broadcast: Delta<Broadcast>,
	pub message_error: Delta<GossamerMessageError>,
}

impl<T: GossamerMessage> DeltaContainer<GossamerContainer<T>> for GossamerDeltaContainer<T> {
	fn apply_deltas(self, container: &mut GossamerContainer<T>) {
		self.message.apply(&mut container.message);
		self.message_in.apply(&mut container.message_in);
		self.message_out.apply(&mut container.message_out);
		self.message_in_flight.apply(&mut container.message_in_flight);
		self.message_broadcast.apply(&mut container.message_broadcast);
		self.message_error.apply(&mut container.message_error);
	}

	fn into_container(self) -> GossamerContainer<T> {
		GossamerContainer {
			message: self.message.into_component(),
			message_in: self.message_in.into_component(),
			message_out: self.message_out.into_component(),
			message_in_flight: self.message_in_flight.into_component(),
			message_broadcast: self.message_broadcast.into_component(),
			message_error: self.message_error.into_component(),
		}
	}
}

impl<T: GossamerMessage> ContainerAccepting<T> for GossamerDeltaContainer<T> {
	fn from_data(data: T) -> Self {
		Self {
			message: Delta::Modified(data),
			message_in: Delta::Unchanged,
			message_out: Delta::Unchanged,
			message_in_flight: Delta::Unchanged,
			message_broadcast: Delta::Unchanged,
			message_error: Delta::Unchanged,
		}
	}

	fn from_removed_data() -> Self {
		Self {
			message: Delta::Removed,
			message_in: Delta::Unchanged,
			message_out: Delta::Unchanged,
			message_in_flight: Delta::Unchanged,
			message_broadcast: Delta::Unchanged,
			message_error: Delta::Unchanged,
		}
	}

	fn update_with_data(&mut self, data: T) {
		self.message = Delta::Modified(data);
	}

	fn remove_from_container(&mut self) {
		self.message = Delta::Removed;
	}
}

impl<T: GossamerMessage> ContainerAccepting<In> for GossamerDeltaContainer<T> {
	fn from_data(data: In) -> Self {
		Self {
			message: Delta::Unchanged,
			message_in: Delta::Modified(data),
			message_out: Delta::Unchanged,
			message_in_flight: Delta::Unchanged,
			message_broadcast: Delta::Unchanged,
			message_error: Delta::Unchanged,
		}
	}

	fn from_removed_data() -> Self {
		Self {
			message: Delta::Unchanged,
			message_in: Delta::Removed,
			message_out: Delta::Unchanged,
			message_in_flight: Delta::Unchanged,
			message_broadcast: Delta::Unchanged,
			message_error: Delta::Unchanged,
		}
	}

	fn update_with_data(&mut self, data: In) {
		self.message_in = Delta::Modified(data);
	}

	fn remove_from_container(&mut self) {
		self.message_in = Delta::Removed;
	}
}

impl<T: GossamerMessage> ContainerAccepting<Out> for GossamerDeltaContainer<T> {
	fn from_data(data: Out) -> Self {
		Self {
			message: Delta::Unchanged,
			message_in: Delta::Unchanged,
			message_out: Delta::Modified(data),
			message_in_flight: Delta::Unchanged,
			message_broadcast: Delta::Unchanged,
			message_error: Delta::Unchanged,
		}
	}

	fn from_removed_data() -> Self {
		Self {
			message: Delta::Unchanged,
			message_in: Delta::Unchanged,
			message_out: Delta::Removed,
			message_in_flight: Delta::Unchanged,
			message_broadcast: Delta::Unchanged,
			message_error: Delta::Unchanged,
		}
	}

	fn update_with_data(&mut self, data: Out) {
		self.message_out = Delta::Modified(data);
	}

	fn remove_from_container(&mut self) {
		println!("Removing out from {:?}", std::any::type_name::<T>());
		self.message_out = Delta::Removed;
	}
}

impl<T: GossamerMessage> ContainerAccepting<InFlight> for GossamerDeltaContainer<T> {
	fn from_data(data: InFlight) -> Self {
		Self {
			message: Delta::Unchanged,
			message_in: Delta::Unchanged,
			message_out: Delta::Unchanged,
			message_in_flight: Delta::Modified(data),
			message_broadcast: Delta::Unchanged,
			message_error: Delta::Unchanged,
		}
	}

	fn from_removed_data() -> Self {
		Self {
			message: Delta::Unchanged,
			message_in: Delta::Unchanged,
			message_out: Delta::Unchanged,
			message_in_flight: Delta::Removed,
			message_broadcast: Delta::Unchanged,
			message_error: Delta::Unchanged,
		}
	}

	fn update_with_data(&mut self, data: InFlight) {
		self.message_in_flight = Delta::Modified(data);
	}

	fn remove_from_container(&mut self) {
		self.message_in_flight = Delta::Removed;
	}
}

impl<T: GossamerMessage> ContainerAccepting<Broadcast> for GossamerDeltaContainer<T> {
	fn from_data(data: Broadcast) -> Self {
		Self {
			message: Delta::Unchanged,
			message_in: Delta::Unchanged,
			message_out: Delta::Unchanged,
			message_in_flight: Delta::Unchanged,
			message_broadcast: Delta::Modified(data),
			message_error: Delta::Unchanged,
		}
	}

	fn from_removed_data() -> Self {
		Self {
			message: Delta::Unchanged,
			message_in: Delta::Unchanged,
			message_out: Delta::Unchanged,
			message_in_flight: Delta::Unchanged,
			message_broadcast: Delta::Removed,
			message_error: Delta::Unchanged,
		}
	}

	fn update_with_data(&mut self, data: Broadcast) {
		self.message_broadcast = Delta::Modified(data);
	}

	fn remove_from_container(&mut self) {
		self.message_broadcast = Delta::Removed;
	}
}

impl<T: GossamerMessage> ContainerAccepting<GossamerMessageError> for GossamerDeltaContainer<T> {
	fn from_data(data: GossamerMessageError) -> Self {
		Self {
			message: Delta::Unchanged,
			message_in: Delta::Unchanged,
			message_out: Delta::Unchanged,
			message_in_flight: Delta::Unchanged,
			message_broadcast: Delta::Unchanged,
			message_error: Delta::Modified(data),
		}
	}

	fn from_removed_data() -> Self {
		Self {
			message: Delta::Unchanged,
			message_in: Delta::Unchanged,
			message_out: Delta::Unchanged,
			message_in_flight: Delta::Unchanged,
			message_broadcast: Delta::Unchanged,
			message_error: Delta::Removed,
		}
	}

	fn update_with_data(&mut self, data: GossamerMessageError) {
		self.message_error = Delta::Modified(data);
	}

	fn remove_from_container(&mut self) {
		self.message_error = Delta::Removed;
	}
}
