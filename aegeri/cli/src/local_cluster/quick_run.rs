use clap::Parser;
use orfile::Orfile;
use serde::{Deserialize, Serialize};

/// Runs a local cluster and logs out the topic, public keys, and addresses of the nodes.
///
/// Note: this does not allow setting the public keys or addresses of the nodes.
#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
#[clap(help_expected = true)]
pub struct QuickRun {
	/// The number of nodes to start
	#[clap(long, default_value = "7")]
	count: usize,
	/// The topic to use for the nodes
	#[clap(long, default_value = "aegeri-local-cluster-quick-run")]
	topic: String,
}

impl QuickRun {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		todo!()
	}
}

impl or_file::QuickRun {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let resolved = self.clone().resolve().await?;
		resolved.execute().await
	}
}
