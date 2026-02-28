use crate::{Gossamer, GossamerConfig, GossamerConfigError};
use libp2p::Multiaddr;

#[derive(thiserror::Error, Debug)]
pub enum LocalClusterError {
	#[error("Error building Gossamer: {0}")]
	BuildError(#[from] GossamerConfigError),
}

#[derive(Debug, Clone)]
pub struct LocalClusterConfig {
	pub count: usize,
	pub topic: String,
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

	pub fn into_base_config(self) -> GossamerConfig {
		GossamerConfig::default().with_topic(self.topic)
	}
}

impl Default for LocalClusterConfig {
	fn default() -> Self {
		Self { count: 3, topic: "gossamer".to_string() }
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
			let config = base_config.clone().with_bootstrap_peers(peers.clone());
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

	#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
	pub struct TestMessage(u32);

	impl GossamerMessage for TestMessage {
		fn to_gossamer_bytes(&self) -> Result<Vec<u8>, GossamerMessageError> {
			Ok(self.0.to_le_bytes().to_vec())
		}
		fn from_gossamer_bytes(bytes: Vec<u8>) -> Result<Self, GossamerMessageError> {
			Ok(TestMessage(u32::from_le_bytes(bytes.try_into().unwrap())))
		}
	}

	#[tokio::test]
	#[ignore = "This acquires empheral ports. Run with --ignored if you want to opt in."]
	async fn test_local_cluster_starts() -> Result<(), LocalClusterError> {
		let config = LocalClusterConfig::default();
		let gossamers = config.build::<u32>().await?;
		assert!(gossamers.len() == 3);
		Ok(())
	}

	#[tokio::test]
	async fn test_local_cluster_sends_and_receives_message() -> Result<(), anyhow::Error> {
		let config = LocalClusterConfig::default();
		let mut gossamers = config.build::<u32>().await?;
		let message = TestMessage(1);
		gossamers[0].0.send_message(0, &message)?;
		let received_message = gossamers[1].0.recv_message::<TestMessage>().await?;
		assert_eq!(received_message, Some(message));
		Ok(())
	}
}
