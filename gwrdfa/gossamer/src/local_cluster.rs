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
