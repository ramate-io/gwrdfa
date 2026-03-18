pub use aegeri_message::{Id, Index, Message, PublicKey, Transaction};
use aegeri_process::aegeri::AegeriHart;
use gossamer::{Gossamer, GossamerConfig, Multiaddr};
pub use ml_dsa;
use std::collections::VecDeque;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio::time::{Duration, Instant};

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
	pub async fn bootstrap_non_participant(
		bootstrap_count: usize,
		bootstrap_peers: impl IntoIterator<Item = (Multiaddr, PublicKey)>,
	) -> Result<(Self, Multiaddr), anyhow::Error> {
		// Collect all the bootstrap peers into a vector.
		let bootstrap_peers = bootstrap_peers.into_iter().collect::<Vec<_>>();

		let gossamer_config = GossamerConfig::default().with_bootstrap_peers(
			bootstrap_peers.iter().map(|(addr, _)| addr).cloned().collect::<Vec<_>>(),
		);
		let (gossamer, listen_addr) = Gossamer::spawn_tokio(gossamer_config).await?;

		let hart = AegeriHart::from_gossamer(gossamer)?
			.with_bootstrap_peer_count_required(bootstrap_count)
			.with_bootstrap_peers(bootstrap_peers.into_iter().map(|(_, pk)| pk).collect::<Vec<_>>())
			.with_is_participant(false);

		Ok((Self::spawn(hart).await?, listen_addr))
	}

	pub async fn spawn(hart: AegeriHart) -> Result<Self, anyhow::Error> {
		let (transaction_sender, transaction_receiver) = unbounded_channel();
		let (status_sender, status_receiver) = unbounded_channel();
		let (shutdown_sender, mut shutdown_receiver) = oneshot::channel();

		let mut hart = hart
			.with_broadcast_transaction_receiver(transaction_receiver)
			.with_transaction_status_sender(status_sender);

		let tick_handle = tokio::spawn(async move {
			loop {
				if shutdown_receiver.try_recv().is_ok() {
					break;
				}
				hart.tick();
				tokio::time::sleep(Duration::from_millis(20)).await;
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
			if let Some(position) = self.buffered_statuses.iter().position(|(index, tx_id)| {
				log::debug!("index: {:?} for id: {:?}", index, id);
				*tx_id == id && matcher(*index)
			}) {
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
			log::debug!("client: received status for id: {:?} index: {:?}", tx_id, index);
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

#[cfg(test)]
mod tests {
	use super::*;
	use aegeri_message::{
		AegeriSubcommittee, Availability, IndexValue, Nonce, PublicKey, TransactionSet,
	};
	use ml_dsa::{MlDsa44, SigningKey, B32};
	use std::sync::Once;

	fn tx_message(seed: u8, nonce: &[u8]) -> Result<Message<Transaction>, anyhow::Error> {
		let signer = SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![seed; 32]));
		Ok(Message::<Transaction>::try_new(&signer, Transaction::Leave, Nonce::new(nonce))?)
	}

	static LOG_INIT: Once = Once::new();

	fn init_test_logger() {
		LOG_INIT.call_once(|| {
			let _ = env_logger::Builder::from_env(
				env_logger::Env::default()
					.default_filter_or("aegeri_full_client=debug,aegeri_process=debug/client:*"),
			)
			.is_test(true)
			.try_init();

			//gossamer=debug,aegeri_process=debug,aegeri_message=debug
		});
	}

	#[tokio::test]
	async fn test_mocked_hart_wait_for_availability_status() -> Result<(), anyhow::Error> {
		let tx = tx_message(7, b"availability")?;
		let tx_id = *tx.id();

		let signer = SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![1; 32]));
		let signer_public_key = PublicKey::new(&signer);
		let genesis_subcommittee =
			AegeriSubcommittee::genesis().with_members(vec![signer_public_key].into_iter());
		let mut txs = TransactionSet::new();
		txs.add_id(tx_id);
		let availability = Availability::from_transactions(txs);

		let (hart, _channels) = AegeriHart::mock()?;
		let hart = hart
			.with_signer(signer)
			.with_genesis(genesis_subcommittee, availability)
			.with_pings(false)
			.with_loopback(true);

		let mut client = FullClient::spawn(hart).await?;

		let sent_id = client.send_transaction(tx)?;
		assert_eq!(sent_id, tx_id);

		let index = client
			.wait_for_availability(tx_id, Duration::from_secs(2))
			.await
			.map_err(|e| anyhow::anyhow!("failed to wait for availability: {}", e))?;
		assert!(index.is_availability(), "index is not availability");
		assert!(index.value() == IndexValue::genesis(), "index value is not genesis");
		Ok(())
	}

	#[tokio::test]
	async fn test_mocked_hart_buffers_non_matching_statuses() -> Result<(), anyhow::Error> {
		init_test_logger();

		let tx_a = tx_message(8, b"tx-a")?;
		let tx_b = tx_message(9, b"tx-b")?;
		let tx_a_id = *tx_a.id();
		let tx_b_id = *tx_b.id();

		let signer = SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![1; 32]));
		let signer_public_key = PublicKey::new(&signer);
		let genesis_subcommittee =
			AegeriSubcommittee::genesis().with_members(vec![signer_public_key].into_iter());

		let (hart, _channels) = AegeriHart::mock()?;
		let hart = hart
			.with_signer(signer)
			.with_genesis(genesis_subcommittee, Availability::genesis())
			.with_pings(false)
			.with_loopback(true);

		let mut client = FullClient::spawn(hart).await?;

		// Send in reverse and then await in original order to force buffering.
		client.send_transaction(tx_b)?;
		client.send_transaction(tx_a)?;

		let index_a = client
			.wait_for_availability(tx_a_id, Duration::from_secs(2))
			.await
			.map_err(|e| anyhow::anyhow!("failed to wait for availability for tx_a: {}", e))?;
		let index_b = client
			.wait_for_availability(tx_b_id, Duration::from_secs(2))
			.await
			.map_err(|e| anyhow::anyhow!("failed to wait for availability for tx_b: {}", e))?;
		assert!(index_a.is_availability(), "index_a is not availability");
		assert!(index_b.is_availability(), "index_b is not availability");

		// Wait for confirmation
		let confirmation_index = client
			.wait_for_confirmation(tx_a_id, Duration::from_secs(2))
			.await
			.map_err(|e| anyhow::anyhow!("failed to wait for confirmation for tx_a: {}", e))?;
		assert!(confirmation_index.is_confirmation(), "confirmation_index is not confirmation");
		assert!(
			confirmation_index.value() >= index_a.value(),
			"confirmation_index is not greater than index_a"
		);

		// Wait for block
		let block_index = client
			.wait_for_block(tx_a_id, Duration::from_secs(2))
			.await
			.map_err(|e| anyhow::anyhow!("failed to wait for block for tx_a: {}", e))?;
		assert!(block_index.is_block(), "block_index is not block");
		assert!(
			block_index.value() >= confirmation_index.value(),
			"block_index is not greater than confirmation_index"
		);

		// Wait for transition
		let transition_index = client
			.wait_for_transition(tx_a_id, Duration::from_secs(2))
			.await
			.map_err(|e| anyhow::anyhow!("failed to wait for transition for tx_a: {}", e))?;
		assert!(transition_index.is_transition(), "transition_index is not transition");
		assert!(
			transition_index.value() >= block_index.value(),
			"transition_index is not greater than block_index"
		);
		Ok(())
	}
}
