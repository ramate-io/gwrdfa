use crate::{Gossamer, GossamerConfig, GossamerConfigError};
use libp2p::{identity::Keypair, Multiaddr};

#[derive(thiserror::Error, Debug)]
pub enum LocalClusterError {
	#[error("Error building Gossamer: {0}")]
	BuildError(#[from] GossamerConfigError),
}

#[derive(Debug, Clone)]
pub struct LocalClusterConfig {
	/// The number of Gossamer instances to start.
	pub count: usize,
	/// The topic to use for the Gossamer instances.
	pub topic: String,
	/// Optional override for gossipsub max transmit size (bytes).
	///
	/// If unset, uses `GossamerConfig` defaults.
	pub gossipsub_max_transmit_size: Option<usize>,
}

impl LocalClusterConfig {
	pub fn with_count(mut self, count: usize) -> Self {
		self.count = count;
		self
	}

	pub fn with_topic(mut self, topic: String) -> Self {
		self.topic = topic;
		self
	}

	pub fn with_gossipsub_max_transmit_size(mut self, bytes: usize) -> Self {
		self.gossipsub_max_transmit_size = Some(bytes);
		self
	}

	pub fn into_base_config(self) -> GossamerConfig {
		let mut config = GossamerConfig::default().with_topic(self.topic);
		if let Some(bytes) = self.gossipsub_max_transmit_size {
			config = config.with_gossipsub_max_transmit_size(bytes);
		}
		config
	}
}

impl Default for LocalClusterConfig {
	fn default() -> Self {
		Self { count: 3, topic: "gossamer".to_string(), gossipsub_max_transmit_size: None }
	}
}

impl LocalClusterConfig {
	pub async fn build<Entity: Send + Sync + 'static>(
		self,
	) -> Result<Vec<(Gossamer<Entity>, Multiaddr)>, LocalClusterError> {
		let mut peers = Vec::new();
		let count = self.count;
		let base_config = self.into_base_config();
		let mut gossamers = Vec::new();

		for _ in 0..count {
			let config = base_config
				.clone()
				.with_bootstrap_peers(peers.clone())
				.with_identity(Keypair::generate_ed25519());
			let (gossamer, multiaddr) = Gossamer::spawn_tokio(config).await?;
			peers.push(multiaddr.clone());
			gossamers.push((gossamer, multiaddr));
		}

		Ok(gossamers)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{GossamerMessage, GossamerMessageError};
	use std::env;
	use tokio::time::Duration;

	#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
	pub struct TestMessage(u32);

	impl GossamerMessage for TestMessage {
		fn to_gossamer_bytes(&self) -> Result<Vec<u8>, GossamerMessageError> {
			Ok(self.0.to_le_bytes().to_vec())
		}
		fn from_gossamer_bytes(bytes: Vec<u8>) -> Result<Self, GossamerMessageError> {
			Ok(TestMessage(u32::from_le_bytes(
				bytes.try_into().map_err(|_| {
					GossamerMessageError::InternalError("Invalid bytes".to_string())
				})?,
			)))
		}
	}

	fn local_cluster_config_for_tests() -> LocalClusterConfig {
		let mut config = LocalClusterConfig::default();
		if let Ok(s) = env::var("GOSSAMER_TEST_MAX_TRANSMIT_SIZE") {
			if let Ok(bytes) = s.parse::<usize>() {
				config = config.with_gossipsub_max_transmit_size(bytes);
			}
		}
		config
	}

	#[tokio::test]
	#[ignore = "This acquires empheral ports. Run with --ignored if you want to opt in."]
	async fn test_local_cluster_starts() -> Result<(), LocalClusterError> {
		let config = local_cluster_config_for_tests();
		let gossamers = config.build::<u32>().await?;
		assert!(gossamers.len() == 3);
		Ok(())
	}

	#[tokio::test]
	#[ignore = "This acquires empheral ports. Run with --ignored if you want to opt in."]
	async fn test_local_cluster_sends_and_receives_message() -> Result<(), anyhow::Error> {
		run_local_cluster_send_and_receive_once(32).await
	}

	async fn run_local_cluster_send_and_receive_once(
		max_retries: usize,
	) -> Result<(), anyhow::Error> {
		let config = local_cluster_config_for_tests();
		let mut gossamers = config.build::<u32>().await?;
		let message = TestMessage(1);

		// Keep sending messages from the sender gossamer instance.
		let mut sender = gossamers.pop().ok_or(anyhow::anyhow!("No sender found"))?;

		// Try to send the message a few times while peers converge.
		// Confirmation here only means publish accepted by local task.
		let mut peer0_received = false;
		let mut peer1_received = false;
		let mut last_error = None;
		for i in 0..max_retries {
			if let Err(e) = sender
				.0
				.send_and_confirm_with_timeout(i as u32, &message, Duration::from_secs(2))
				.await
			{
				last_error = Some(e);
			} else {
				if !peer0_received {
					let maybe_message = gossamers[0]
						.0
						.recv_message_with_timeout::<TestMessage>(Duration::from_millis(500))
						.await;
					if let Ok(Some(received_message)) = maybe_message {
						assert_eq!(received_message, message);
						peer0_received = true;
					}
				}
				if !peer1_received {
					let maybe_message = gossamers[1]
						.0
						.recv_message_with_timeout::<TestMessage>(Duration::from_millis(500))
						.await;
					if let Ok(Some(received_message)) = maybe_message {
						assert_eq!(received_message, message);
						peer1_received = true;
					}
				}
			}

			if peer0_received && peer1_received {
				break;
			}
			tokio::time::sleep(Duration::from_millis(250)).await;
		}
		if !(peer0_received && peer1_received) {
			return Err(anyhow::anyhow!(
				"failed to deliver test message to all peers after retries; last publish error: {:?}",
				last_error
			));
		}
		Ok(())
	}

	#[tokio::test]
	#[ignore = "Stress test for https://github.com/ramate-io/gwrdfa/issues/18; run manually with --ignored."]
	async fn test_local_cluster_sends_and_receives_message_stress_issue_18(
	) -> Result<(), anyhow::Error> {
		let iterations = env::var("GOSSAMER_STRESS_ITERS")
			.ok()
			.and_then(|s| s.parse::<usize>().ok())
			.unwrap_or(16);

		for i in 0..iterations {
			run_local_cluster_send_and_receive_once(32).await.map_err(|e| {
				anyhow::anyhow!(
					"stress iteration {i}/{iterations} failed for https://github.com/ramate-io/gwrdfa/issues/18: {e}"
				)
			})?;
		}

		Ok(())
	}
}
