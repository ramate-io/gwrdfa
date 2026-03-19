use aegeri_full_client::FullClient;
use aegeri_message::{Message, Nonce, Transaction};
use clap::Parser;
use ml_dsa::{ExpandedSigningKey, MlDsa44, SigningKey, B32};
use orfile::Orfile;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::time::Duration;

use crate::common::PeerList;
/// Sends a transaction to join the cluster and waits for consensus on the transaction.
#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
#[clap(help_expected = true)]
pub struct Join {
	/// Topic to use for gossamer networking.
	#[clap(long, default_value = "aegeri-local-cluster-quick-run")]
	topic: String,
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
		if self.peer_list.multiaddr.is_empty() {
			anyhow::bail!("at least one --multiaddr is required");
		}
		if self.peer_list.peers.is_empty() {
			anyhow::bail!("at least one --peers is required");
		}

		let signer = self.resolve_signer()?;
		// Build address/public-key tuples even when counts differ by cycling the shorter list.
		// This lets callers provide e.g. a single bootstrap multiaddr with many allowed peers.
		let tuple_count = self.peer_list.multiaddr.len().max(self.peer_list.peers.len());
		let bootstrap_peers = (0..tuple_count)
			.map(|i| {
				(
					self.peer_list.multiaddr[i % self.peer_list.multiaddr.len()].clone(),
					self.peer_list.peers[i % self.peer_list.peers.len()].clone(),
				)
			})
			.collect::<Vec<_>>();
		let bootstrap_count = self.peer_count_required.min(bootstrap_peers.len());

		log::info!(
			"bootstrapping with count={} and peers={:?}",
			bootstrap_count,
			bootstrap_peers
				.iter()
				.map(|(addr, pk)| format!("{}:{}", addr, pk))
				.collect::<Vec<_>>()
		);
		let (mut client, listen_addr) = FullClient::bootstrap_non_participant(
			self.topic.clone(),
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

	fn resolve_signer(&self) -> Result<SigningKey<MlDsa44>, anyhow::Error> {
		if let Some(private_key_hex) = &self.private_key {
			let bytes = hex::decode(
				private_key_hex.strip_prefix("0x").unwrap_or(private_key_hex.as_str()),
			)?;
			if bytes.len() == 32 {
				return Ok(SigningKey::<MlDsa44>::from_seed(&B32::from_iter(bytes)));
			}

			let expanded =
				ExpandedSigningKey::<MlDsa44>::try_from(bytes.as_slice()).map_err(|_| {
					anyhow::anyhow!(
					"--private-key must be either a 32-byte seed (64 hex chars) or an ML-DSA expanded signing key"
				)
				})?;
			#[allow(deprecated)]
			return Ok(SigningKey::<MlDsa44>::from_expanded(&expanded));
		}
		let seed_byte: u8 = u8::try_from(self.seed)
			.map_err(|_| anyhow::anyhow!("--seed must be in range 0..=255"))?;
		Ok(SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![seed_byte; 32])))
	}
}

impl or_file::Join {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let resolved = self.clone().resolve().await?;
		resolved.execute().await
	}
}
