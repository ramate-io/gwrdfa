use aegeri_message::PublicKey;
use clap::Parser;
use gossamer::Multiaddr;
use ml_dsa::{ExpandedSigningKey, MlDsa44, SigningKey, B32};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
pub struct PeerList {
	/// The list of public keys to join the cluster.
	#[clap(long)]
	pub peers: Vec<PublicKey>,
	/// The multiaddress to join the cluster on.
	#[clap(long)]
	pub multiaddr: Vec<Multiaddr>,
}

impl PeerList {
	pub fn new() -> Self {
		Self { peers: Vec::new(), multiaddr: Vec::new() }
	}

	pub fn add_peer(&mut self, peer: PublicKey) {
		self.peers.push(peer);
	}

	pub fn add_multiaddr(&mut self, multiaddr: Multiaddr) {
		self.multiaddr.push(multiaddr);
	}
}

pub fn bootstrap_peers_from_peer_list(
	peer_list: &PeerList,
) -> Result<Vec<(Multiaddr, PublicKey)>, anyhow::Error> {
	if peer_list.multiaddr.is_empty() {
		anyhow::bail!("at least one --multiaddr is required");
	}
	if peer_list.peers.is_empty() {
		anyhow::bail!("at least one --peers is required");
	}

	// Build address/public-key tuples even when counts differ by cycling the shorter list.
	// This lets callers provide e.g. a single bootstrap multiaddr with many allowed peers.
	let tuple_count = peer_list.multiaddr.len().max(peer_list.peers.len());
	let bootstrap_peers = (0..tuple_count)
		.map(|i| {
			(
				peer_list.multiaddr[i % peer_list.multiaddr.len()].clone(),
				peer_list.peers[i % peer_list.peers.len()].clone(),
			)
		})
		.collect::<Vec<_>>();
	Ok(bootstrap_peers)
}

pub fn resolve_signer(
	private_key_hex: Option<&str>,
	seed: u64,
) -> Result<SigningKey<MlDsa44>, anyhow::Error> {
	if let Some(private_key_hex) = private_key_hex {
		let bytes = hex::decode(private_key_hex.strip_prefix("0x").unwrap_or(private_key_hex))?;
		if bytes.len() == 32 {
			return Ok(SigningKey::<MlDsa44>::from_seed(&B32::from_iter(bytes)));
		}

		let expanded = ExpandedSigningKey::<MlDsa44>::try_from(bytes.as_slice()).map_err(|_| {
			anyhow::anyhow!(
				"--private-key must be either a 32-byte seed (64 hex chars) or an ML-DSA expanded signing key"
			)
		})?;
		#[allow(deprecated)]
		return Ok(SigningKey::<MlDsa44>::from_expanded(&expanded));
	}

	let seed_byte: u8 =
		u8::try_from(seed).map_err(|_| anyhow::anyhow!("--seed must be in range 0..=255"))?;
	Ok(SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![seed_byte; 32])))
}
