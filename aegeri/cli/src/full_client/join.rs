use aegeri_message::PublicKey;
use clap::Parser;
use gossamer::Multiaddr;
use orfile::Orfile;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Sends a transaction to join the cluster and waits for consensus on the transaction.
#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
#[clap(help_expected = true)]
pub struct Join {
	/// The private key hex string to use for the signer.
	#[clap(long)]
	private_key: Option<String>,
	/// The seed to use for the signer if no private key is provided.
	#[clap(long, default_value = "1")]
	seed: u64,
	/// The peer list of public keys to join the cluster.
	#[clap(long)]
	peers: Vec<PublicKey>,
	/// The multiaddress to join the cluster on.
	#[clap(long)]
	multiaddr: Multiaddr,
	/// The timeout to use for the join operation.
	#[clap(long, default_value = "10s")]
	timeout: Duration,
}

impl Join {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		Ok(())
	}
}

impl or_file::Join {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let resolved = self.clone().resolve().await?;
		resolved.execute().await
	}
}
