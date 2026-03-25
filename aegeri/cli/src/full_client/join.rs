use aegeri_full_client::FullClient;
use aegeri_message::{Message, Nonce, Transaction};
use clap::Parser;
use orfile::Orfile;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::common::{
	bootstrap_peers_from_peer_list, gossamer_config_for_bootstrap, resolve_signer, GossamerCliConfig,
	PeerList,
};
/// Sends a transaction to join the cluster and waits for consensus on the transaction.
#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
#[clap(help_expected = true)]
pub struct Join {
	#[clap(flatten)]
	gossamer: GossamerCliConfig,
	/// The private key hex string to use for the signer.
	///
	/// Currently interpreted as a 32-byte hex seed for ML-DSA key derivation.
	#[clap(long)]
	private_key: Option<String>,
	/// The seed to use for the signer if no private key is provided.
	#[clap(long, default_value = "42")]
	seed: u64,
	/// The peer list to join the cluster.
	#[orfile(config)]
	#[clap(flatten)]
	peer_list: PeerList,
	/// The number of peers to require during bootstrap.
	#[clap(long, default_value = "3")]
	peer_count_required: usize,
	/// Timeout in seconds to wait for transition confirmation.
	#[clap(long, default_value = "60")]
	timeout_seconds: u64,
}

impl Join {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let signer = resolve_signer(self.private_key.as_deref(), self.seed)?;
		let bootstrap_peers = bootstrap_peers_from_peer_list(&self.peer_list)?;
		let bootstrap_count = self.peer_count_required.min(bootstrap_peers.len());

		log::info!(
			"bootstrapping with count={} and peers={:?}",
			bootstrap_count,
			bootstrap_peers
				.iter()
				.map(|(addr, pk)| format!("{}:{}", addr, pk))
				.collect::<Vec<_>>()
		);
		let gossamer_config = gossamer_config_for_bootstrap(self.gossamer.clone(), &bootstrap_peers);
		let (mut client, listen_addr) = FullClient::bootstrap_non_participant(
			gossamer_config,
			bootstrap_count,
			bootstrap_peers,
		)
		.await?;
		log::info!("bootstrapped with client using listen address {}", listen_addr);

		let timeout = Duration::from_secs(self.timeout_seconds);
		let nonce = Nonce::new(
			std::time::SystemTime::now()
				.duration_since(std::time::UNIX_EPOCH)?
				.as_nanos()
				.to_le_bytes()
				.to_vec(),
		);
		let transaction = Message::<Transaction>::try_new(&signer, Transaction::Join, nonce)?;
		let id = client.send_transaction(transaction)?;
		let transition_index = client.wait_for_transition(id, timeout).await?;

		println!("client_listen_addr: {listen_addr}");
		println!("transaction_id: {id}");
		println!("transaction included in transition: {transition_index:?}");
		Ok(())
	}
}

impl or_file::Join {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let resolved = self.clone().resolve().await?;
		resolved.execute().await
	}
}
