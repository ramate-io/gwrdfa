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
use std::collections::VecDeque;
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
	pub(crate) pending_outbound: PendingOutbound<Entity>,
	pub(crate) topic_hash: TopicHash,
	pub(crate) swarm: Swarm<GossamerBehaviour>,
	pub(crate) listen_addr_sender: Option<Sender<Multiaddr>>,
}

impl<Entity: Send + Sync + 'static> Unpin for GossamerTask<Entity> {}

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
	#[error(
		"Pending outbound queue is full: attempted={attempted_message_bytes} bytes, pending={pending_bytes} bytes, max={max_pending_outbound_bytes} bytes"
	)]
	PendingOutboundFull {
		attempted_message_bytes: usize,
		pending_bytes: usize,
		max_pending_outbound_bytes: usize,
	},
}

#[derive(Debug)]
pub struct PendingOutbound<Entity> {
	queue: VecDeque<(Entity, Vec<u8>)>,
	current_pending_bytes: usize,
	max_pending_bytes: usize,
}

impl<Entity> PendingOutbound<Entity> {
	pub fn new(max_pending_bytes: usize) -> Self {
		Self { queue: VecDeque::new(), current_pending_bytes: 0, max_pending_bytes }
	}

	pub fn push(
		&mut self,
		entity: Entity,
		msg: Vec<u8>,
	) -> Result<(), (Entity, GossamerTaskError)> {
		let attempted_message_bytes = msg.len();
		let Some(new_pending_bytes) = self.current_pending_bytes.checked_add(attempted_message_bytes)
		else {
			return Err((
				entity,
				GossamerTaskError::PendingOutboundFull {
					attempted_message_bytes,
					pending_bytes: self.current_pending_bytes,
					max_pending_outbound_bytes: self.max_pending_bytes,
				},
			));
		};

		if new_pending_bytes > self.max_pending_bytes {
			return Err((
				entity,
				GossamerTaskError::PendingOutboundFull {
					attempted_message_bytes,
					pending_bytes: self.current_pending_bytes,
					max_pending_outbound_bytes: self.max_pending_bytes,
				},
			));
		}

		self.current_pending_bytes = new_pending_bytes;
		self.queue.push_back((entity, msg));
		Ok(())
	}

	pub fn pop(&mut self) -> Option<(Entity, Vec<u8>)> {
		let popped = self.queue.pop_front()?;
		self.current_pending_bytes = self.current_pending_bytes.saturating_sub(popped.1.len());
		Some(popped)
	}
}

impl<Entity: Send + Sync + 'static> Future for GossamerTask<Entity> {
	type Output = Result<(), GossamerTaskError>;

	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		fn try_publish<Entity: Send + Sync + 'static>(
			task: &mut GossamerTask<Entity>,
			entity: Entity,
			msg: Vec<u8>,
		) -> Result<bool, GossamerTaskError> {
			match task
				.swarm
				.behaviour_mut()
				.gossipsub
				.publish(task.topic_hash.clone(), msg.clone())
			{
				Ok(_) => {
					task.entity_into_gossamer_sender
						.send(Ok(entity))
						.map_err(|e| GossamerTaskError::BroadcastResultRelayError(e.to_string()))?;
					Ok(true)
				}
				Err(gossipsub::PublishError::InsufficientPeers) => {
					if let Err((entity, e)) = task.pending_outbound.push(entity, msg) {
						task.entity_into_gossamer_sender
							.send(Err((entity, e)))
							.map_err(|e| {
								GossamerTaskError::BroadcastResultRelayError(e.to_string())
							})?;
						return Ok(true);
					}
					Ok(false)
				}
				Err(e) => {
					gossamer_log!("gossamer: publish failed for entity due to: {e}");
					task.entity_into_gossamer_sender
						.send(Err((entity, GossamerTaskError::BroadcastError(e.to_string()))))
						.map_err(|e| GossamerTaskError::BroadcastResultRelayError(e.to_string()))?;
					Ok(true)
				}
			}
		}

		// Broadcast messages to the swarm.
		// Drain the receiver_from_gossamer while there are messages to broadcast.

		// Ingest messages from the swarm.
		let this = self.get_mut();
		loop {
			let mut progressed = false;

			if let Some((entity, msg)) = this.pending_outbound.pop() {
				let publish_result = try_publish(this, entity, msg)?;
				progressed = progressed || publish_result;
			}

			// 1. Poll outbound channel
			match Pin::new(&mut this.entity_message_from_gossamer_receiver).poll_recv(cx) {
				Poll::Ready(Some((entity, msg))) => {
					let _ = try_publish(this, entity, msg)?;
					// Receiving one outbound message is progress even if publish is deferred.
					progressed = true;
				}
				Poll::Ready(None) => {
					return Poll::Ready(Err(GossamerTaskError::BroadcastReceiverDisconnected));
				}
				Poll::Pending => {}
			}

			// Drain while there are messages to receive.
			match Pin::new(&mut this.swarm).poll_next(cx) {
				Poll::Ready(Some(SwarmEvent::Behaviour(GossamerBehaviourEvent::Gossipsub(
					gossipsub::Event::Message { message, .. },
				)))) => {
					if let Err(e) = this.message_into_gossamer_sender.send(message.data) {
						return Poll::Ready(Err(GossamerTaskError::RelayToGossamerError(e)));
					}
					progressed = true;
				}

				Poll::Ready(Some(SwarmEvent::ConnectionEstablished { peer_id, .. })) => {
					{
						let behaviour = this.swarm.behaviour_mut();
						behaviour.gossipsub.add_explicit_peer(&peer_id);
						if let Err(e) = behaviour.kad.bootstrap() {
							gossamer_log!(
								"gossamer: kademlia bootstrap not started after connection to {peer_id}: {e}"
							);
						}
					}
					progressed = true;
				}

				Poll::Ready(Some(SwarmEvent::ConnectionClosed { peer_id, .. })) => {
					this.swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
					progressed = true;
				}

				Poll::Ready(Some(SwarmEvent::OutgoingConnectionError {
					peer_id, error, ..
				})) => {
					gossamer_log!("gossamer: outgoing connection error to {:?}: {error}", peer_id);
					progressed = true;
				}

				Poll::Ready(Some(SwarmEvent::NewListenAddr { address, .. })) => {
					if let Some(sender) = this.listen_addr_sender.take() {
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
