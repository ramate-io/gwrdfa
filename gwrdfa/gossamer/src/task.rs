use crate::{GossamerBehaviour, GossamerBehaviourEvent};
use futures::{
	task::{Context, Poll},
	Future, Stream,
};
use libp2p::{
	gossipsub::{self, TopicHash},
	swarm::SwarmEvent,
	Multiaddr, Swarm,
};
use std::pin::Pin;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot::Sender;

#[cfg(feature = "gossamer-logging")]
macro_rules! gossamer_log {
	($($arg:tt)*) => {
		eprintln!($($arg)*)
	};
}

#[cfg(not(feature = "gossamer-logging"))]
macro_rules! gossamer_log {
	($($arg:tt)*) => {};
}

pub struct GossamerTask<Entity: Send + Sync + 'static> {
	pub(crate) message_into_gossamer_sender: UnboundedSender<Vec<u8>>,
	pub(crate) entity_message_from_gossamer_receiver: UnboundedReceiver<(Entity, Vec<u8>)>,
	pub(crate) entity_into_gossamer_sender:
		UnboundedSender<Result<Entity, (Entity, GossamerTaskError)>>,
	pub(crate) topic_hash: TopicHash,
	pub(crate) swarm: Swarm<GossamerBehaviour>,
	pub(crate) listen_addr_sender: Option<Sender<Multiaddr>>,
}

#[derive(Debug, thiserror::Error)]
pub enum GossamerTaskError {
	#[error("Error relaying message to Gossamer from swarm: {0}")]
	RelayToGossamerError(#[from] tokio::sync::mpsc::error::SendError<Vec<u8>>),
	#[error("Error broadcasting message: {0}")]
	BroadcastError(String),
	#[error("Error relaying broadcast result to gossamer API: {0}")]
	BroadcastResultRelayError(String),
	#[error("The broadcast receiver is disconnected")]
	BroadcastReceiverDisconnected,
	#[error("The swarm stream is disconnected")]
	SwarmStreamDisconnected,
	#[error("Error sending listen address to the sender: {0}")]
	ListenAddrSenderError(String),
}

impl<Entity: Send + Sync + 'static> Future for GossamerTask<Entity> {
	type Output = Result<(), GossamerTaskError>;

	fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		// Broadcast messages to the swarm.
		// Drain the receiver_from_gossamer while there are messages to broadcast.

		// Ingest messages from the swarm.
		loop {
			let mut progressed = false;
			let topic_hash = self.topic_hash.clone();

			// 1. Poll outbound channel
			match Pin::new(&mut self.entity_message_from_gossamer_receiver).poll_recv(cx) {
				Poll::Ready(Some((entity, msg))) => {
					let res = match self.swarm.behaviour_mut().gossipsub.publish(topic_hash, msg) {
						Ok(_) => Ok(entity),
						Err(e) => {
							gossamer_log!("gossamer: publish failed for entity due to: {e}");
							Err((entity, GossamerTaskError::BroadcastError(e.to_string())))
						}
					};

					self.entity_into_gossamer_sender
						.send(res)
						.map_err(|e| GossamerTaskError::BroadcastResultRelayError(e.to_string()))?;
					progressed = true;
				}
				Poll::Ready(None) => {
					return Poll::Ready(Err(GossamerTaskError::BroadcastReceiverDisconnected));
				}
				Poll::Pending => {}
			}

			// Drain while there are messages to receive.
			match Pin::new(&mut self.swarm).poll_next(cx) {
				Poll::Ready(Some(SwarmEvent::Behaviour(GossamerBehaviourEvent::Gossipsub(
					gossipsub::Event::Message { message, .. },
				)))) => {
					if let Err(e) = self.message_into_gossamer_sender.send(message.data) {
						return Poll::Ready(Err(GossamerTaskError::RelayToGossamerError(e)));
					}
					progressed = true;
				}

				Poll::Ready(Some(SwarmEvent::NewListenAddr { address, .. })) => {
					if let Some(sender) = self.listen_addr_sender.take() {
						let _ = sender
							.send(address)
							.map_err(|e| GossamerTaskError::ListenAddrSenderError(e.to_string()))?;
					}
				}

				Poll::Ready(Some(_)) => {}

				Poll::Ready(None) => {
					return Poll::Ready(Err(GossamerTaskError::SwarmStreamDisconnected));
				}

				Poll::Pending => {}
			}

			if !progressed {
				return Poll::Pending;
			}
		}
	}
}
