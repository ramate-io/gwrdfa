use crate::config::{GossamerConfig, GossamerConfigError};
use libp2p::Multiaddr;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

pub struct Gossamer<Entity: Send + Sync> {
	pub(crate) message_into_gossamer_receiver: UnboundedReceiver<Vec<u8>>,
	pub(crate) entity_message_from_gossamer_sender: UnboundedSender<(Entity, Vec<u8>)>,
	pub(crate) entity_into_gossamer_receiver: UnboundedReceiver<Entity>,
}

#[derive(Debug, Clone, thiserror::Error, PartialEq, Eq)]
pub enum GossamerMessageError {
	#[error("Error serializing message: {0:?}")]
	SerializeError((String, Vec<u8>)),
	#[error("Error deserializing message: {0:?}")]
	DeserializeError((String, Vec<u8>)),
	#[error("Error sending message from Gossamer to the swarm")]
	RelayToSwarm,
	#[error("Error receiving message from the swarm: {0}")]
	ReceiveFromSwarmError(#[from] tokio::sync::mpsc::error::TryRecvError),
}

pub trait GossamerMessage: Sized {
	fn to_gossamer_bytes(&self) -> Result<Vec<u8>, GossamerMessageError>;
	fn from_gossamer_bytes(bytes: Vec<u8>) -> Result<Self, GossamerMessageError>;
}

impl<Entity: Send + Sync + 'static> Gossamer<Entity> {
	/// Spawns a Gossamer task in a tokio runtime.
	pub async fn spawn_tokio(
		config: GossamerConfig,
	) -> Result<(Gossamer<Entity>, Multiaddr), GossamerConfigError> {
		let (gossamer_task, listen_addr_receiver, gossamer) = config.build().await?;
		tokio::spawn(gossamer_task);
		let listen_addr = listen_addr_receiver.await?;
		Ok((gossamer, listen_addr))
	}

	/// Produces a mock instance, mostly used for testing purposes.
	pub fn mock() -> (
		Self,
		UnboundedSender<Vec<u8>>,
		UnboundedReceiver<(Entity, Vec<u8>)>,
		UnboundedSender<Entity>,
	) {
		let (message_into_gossamer_sender, message_into_gossamer_receiver) = unbounded_channel();
		let (entity_message_from_gossamer_sender, entity_message_from_gossamer_receiver) =
			unbounded_channel();
		let (entity_into_gossamer_sender, entity_into_gossamer_receiver) = unbounded_channel();

		(
			Self {
				message_into_gossamer_receiver,
				entity_message_from_gossamer_sender,
				entity_into_gossamer_receiver,
			},
			message_into_gossamer_sender,
			entity_message_from_gossamer_receiver,
			entity_into_gossamer_sender,
		)
	}

	pub fn try_recv_message<M: GossamerMessage>(
		&mut self,
	) -> Result<Option<M>, GossamerMessageError> {
		match self.message_into_gossamer_receiver.try_recv() {
			Ok(bytes) => {
				let message = GossamerMessage::from_gossamer_bytes(bytes)?;
				Ok(Some(message))
			}
			Err(TryRecvError::Empty) => Ok(None),
			Err(TryRecvError::Disconnected) => {
				Err(GossamerMessageError::ReceiveFromSwarmError(TryRecvError::Disconnected))
			}
		}
	}

	pub fn send_message<M: GossamerMessage>(
		&mut self,
		entity: Entity,
		message: &M,
	) -> Result<(), GossamerMessageError> {
		let bytes = message.to_gossamer_bytes()?;
		self.entity_message_from_gossamer_sender
			.send((entity, bytes))
			.map_err(|_| GossamerMessageError::RelayToSwarm)?;
		Ok(())
	}

	pub fn try_recv_confirmation(&mut self) -> Result<Option<Entity>, GossamerMessageError> {
		match self.entity_into_gossamer_receiver.try_recv() {
			Ok(entity) => Ok(Some(entity)),
			Err(TryRecvError::Empty) => Ok(None),
			Err(TryRecvError::Disconnected) => {
				Err(GossamerMessageError::ReceiveFromSwarmError(TryRecvError::Disconnected))
			}
		}
	}
}

/// Marks that an entity has flowed in through Gossamer In
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct In;

/// Marks that an entity has been dispatched for Gossamer out
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Out;

/// Marks an entity that has been pushed in flight for Gossamer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InFlight;

/// Marks that an entity has flowed through Gossamer broadcast.
///
/// Often these are entities that are ready to be removed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Broadcast;

#[cfg(test)]
mod tests {
	use super::*;

	#[derive(Debug, Clone, PartialEq, Eq, Hash)]
	pub struct TestMessage(Vec<u8>);

	impl TestMessage {
		pub fn new(data: Vec<u8>) -> Self {
			Self(data)
		}
	}

	impl GossamerMessage for TestMessage {
		fn to_gossamer_bytes(&self) -> Result<Vec<u8>, GossamerMessageError> {
			Ok(self.0.clone())
		}
		fn from_gossamer_bytes(bytes: Vec<u8>) -> Result<Self, GossamerMessageError> {
			Ok(TestMessage(bytes))
		}
	}

	#[tokio::test]
	async fn test_mock_flow() -> Result<(), anyhow::Error> {
		// Build the mock Gossamer instance.
		let (
			mut gossamer,
			message_into_gossamer_sender,
			mut entity_message_from_gossamer_receiver,
			entity_into_gossamer_sender,
		) = Gossamer::<u32>::mock();

		// Send a message into the Gossamer instance.
		let message1 = TestMessage::new(vec![1, 2, 3]);
		let message1_bytes = message1.to_gossamer_bytes()?;
		message_into_gossamer_sender.send(message1_bytes)?;

		// Receive the message from the Gossamer instance.
		let message = gossamer.try_recv_message::<TestMessage>()?;
		assert_eq!(message, Some(TestMessage(vec![1, 2, 3])));

		// Send an out message from the Gossamer instance.
		let entity1 = 1;
		let message2 = TestMessage::new(vec![4, 5, 6]);
		let message2_bytes = message2.to_gossamer_bytes()?;
		gossamer.send_message(entity1, &message2)?;
		let (entity, message) = entity_message_from_gossamer_receiver
			.recv()
			.await
			.ok_or(anyhow::anyhow!("Failed to receive message"))?;
		assert_eq!(entity, entity1);
		assert_eq!(message, message2_bytes);

		// Send a confirmation back into the Gossamer instance.
		let entity1 = 1;
		entity_into_gossamer_sender.send(entity1)?;
		let confirmation = gossamer.try_recv_confirmation()?;
		assert_eq!(confirmation, Some(entity1));

		Ok(())
	}
}
