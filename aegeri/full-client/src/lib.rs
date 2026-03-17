pub use aegeri_message::{Id, Index, Message, Transaction};
use aegeri_process::aegeri::AegeriHart;
use aegeri_process::gossamer::{GossamerChannels, GossamerConfig, Multiaddr};
use gwrdfa_container::ContainerEntity;
pub use ml_dsa;
use std::collections::VecDeque;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct FullClientConfig {
	pub mocked: bool,
	pub bootstrap_peers: Vec<Multiaddr>,
	pub is_participant: bool,
	pub tick_interval: Duration,
}

impl Default for FullClientConfig {
	fn default() -> Self {
		Self {
			mocked: true,
			bootstrap_peers: Vec::new(),
			is_participant: true,
			tick_interval: Duration::from_millis(20),
		}
	}
}

impl FullClientConfig {
	pub fn with_mocked(mut self, mocked: bool) -> Self {
		self.mocked = mocked;
		self
	}

	pub fn with_bootstrap_peers(mut self, bootstrap_peers: Vec<Multiaddr>) -> Self {
		self.bootstrap_peers = bootstrap_peers;
		self
	}

	pub fn with_is_participant(mut self, is_participant: bool) -> Self {
		self.is_participant = is_participant;
		self
	}

	pub fn with_tick_interval(mut self, tick_interval: Duration) -> Self {
		self.tick_interval = tick_interval;
		self
	}
}

pub struct FullClient {
	transaction_sender: UnboundedSender<Message<Transaction>>,
	status_receiver: UnboundedReceiver<(Index, Id)>,
	buffered_statuses: VecDeque<(Index, Id)>,
	shutdown_sender: Option<oneshot::Sender<()>>,
	tick_handle: JoinHandle<()>,
}

impl Drop for FullClient {
	fn drop(&mut self) {
		if let Some(shutdown_sender) = self.shutdown_sender.take() {
			let _ = shutdown_sender.send(());
		}
		self.tick_handle.abort();
	}
}

impl FullClient {
	pub async fn spawn(config: FullClientConfig) -> Result<Self, anyhow::Error> {
		let (transaction_sender, transaction_receiver) = unbounded_channel();
		let (status_sender, status_receiver) = unbounded_channel();
		let (shutdown_sender, mut shutdown_receiver) = oneshot::channel();

		let (mut hart, mock_channels): (AegeriHart, Option<GossamerChannels<ContainerEntity>>) =
			if config.mocked {
				let (hart, channels) = AegeriHart::mock()?;
				(hart, Some(channels))
			} else {
				let gossamer_config =
					GossamerConfig::default().with_bootstrap_peers(config.bootstrap_peers.clone());
				let (hart, _listen_addr) = AegeriHart::spawn_tokio(gossamer_config).await?;
				(hart, None)
			};

		hart = hart
			.with_is_participant(config.is_participant)
			.with_broadcast_transaction_receiver(transaction_receiver)
			.with_transaction_status_sender(status_sender)
			.with_report_transaction_channel_errors(false)
			.with_loopback(config.mocked);

		let tick_interval = config.tick_interval;
		let tick_handle = tokio::spawn(async move {
			let mut mock_channels = mock_channels;
			loop {
				if shutdown_receiver.try_recv().is_ok() {
					break;
				}

				hart.tick();
				if let Some(channels) = mock_channels.as_mut() {
					// In mocked mode, emulate gossamer publish confirmations and network loopback.
					while let Ok((entity, bytes)) =
						channels.entity_message_from_gossamer_receiver.try_recv()
					{
						let _ = channels.entity_into_gossamer_sender.send(Ok(entity));
						let _ = channels.message_into_gossamer_sender.send(bytes);
					}
				}
				tokio::time::sleep(tick_interval).await;
			}
		});

		Ok(Self {
			transaction_sender,
			status_receiver,
			buffered_statuses: VecDeque::new(),
			shutdown_sender: Some(shutdown_sender),
			tick_handle,
		})
	}

	pub fn send_transaction(&self, transaction: Message<Transaction>) -> Result<Id, anyhow::Error> {
		let id = *transaction.id();
		self.transaction_sender.send(transaction)?;
		Ok(id)
	}

	pub async fn wait_for_status(
		&mut self,
		id: Id,
		timeout: Duration,
		matcher: impl Fn(Index) -> bool,
	) -> Result<Index, anyhow::Error> {
		let deadline = Instant::now() + timeout;
		loop {
			if let Some(position) = self
				.buffered_statuses
				.iter()
				.position(|(index, tx_id)| *tx_id == id && matcher(*index))
			{
				let (index, _id) =
					self.buffered_statuses.remove(position).expect("checked position");
				return Ok(index);
			}

			let now = Instant::now();
			if now >= deadline {
				anyhow::bail!("timed out waiting for transaction status");
			}
			let remaining = deadline - now;

			let received = tokio::time::timeout(remaining, self.status_receiver.recv()).await?;
			let Some((index, tx_id)) = received else {
				anyhow::bail!("transaction status channel closed");
			};
			if tx_id == id && matcher(index) {
				return Ok(index);
			}
			self.buffered_statuses.push_back((index, tx_id));
		}
	}

	pub async fn wait_for_availability(
		&mut self,
		id: Id,
		timeout: Duration,
	) -> Result<Index, anyhow::Error> {
		self.wait_for_status(id, timeout, |index| matches!(index, Index::Availability(_)))
			.await
	}

	pub async fn wait_for_confirmation(
		&mut self,
		id: Id,
		timeout: Duration,
	) -> Result<Index, anyhow::Error> {
		self.wait_for_status(id, timeout, |index| matches!(index, Index::Confirmation(_)))
			.await
	}

	pub async fn wait_for_block(
		&mut self,
		id: Id,
		timeout: Duration,
	) -> Result<Index, anyhow::Error> {
		self.wait_for_status(id, timeout, |index| matches!(index, Index::Block(_)))
			.await
	}

	pub async fn wait_for_transition(
		&mut self,
		id: Id,
		timeout: Duration,
	) -> Result<Index, anyhow::Error> {
		self.wait_for_status(id, timeout, |index| matches!(index, Index::Transition(_)))
			.await
	}

	pub async fn send_and_wait_for_transition(
		&mut self,
		transaction: Message<Transaction>,
		timeout: Duration,
	) -> Result<Index, anyhow::Error> {
		let id = self.send_transaction(transaction)?;
		self.wait_for_transition(id, timeout).await
	}
}
