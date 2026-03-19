use aegeri_full_client::FullClient;
use aegeri_message::{ElfScript, Message, Nonce, Transaction};
use clap::Parser;
use orfile::Orfile;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use crate::common::{bootstrap_peers_from_peer_list, resolve_signer, PeerList};

/// Sends an ELF transaction and waits for transition consensus.
#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
#[clap(help_expected = true)]
pub struct SendElf {
	/// Topic to use for gossamer networking.
	#[clap(long, default_value = "aegeri-local-cluster-quick-run")]
	topic: String,
	/// The private key hex string to use for the signer.
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
	/// ELF path or workspace binary name.
	#[clap(long)]
	elf: String,
}

#[derive(Debug, Deserialize)]
struct CargoMetadata {
	workspace_root: String,
	target_directory: String,
}

impl SendElf {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let signer = resolve_signer(self.private_key.as_deref(), self.seed)?;
		let bootstrap_peers = bootstrap_peers_from_peer_list(&self.peer_list)?;
		let bootstrap_count = self.peer_count_required.min(bootstrap_peers.len());

		let (mut client, listen_addr) = FullClient::bootstrap_non_participant(
			self.topic.clone(),
			bootstrap_count,
			bootstrap_peers,
		)
		.await?;

		let elf_bytes = self.load_elf_bytes()?;
		let timeout = Duration::from_secs(self.timeout_seconds);
		let nonce = Nonce::new(
			std::time::SystemTime::now()
				.duration_since(std::time::UNIX_EPOCH)?
				.as_nanos()
				.to_le_bytes()
				.to_vec(),
		);

		let transaction = Message::<Transaction>::try_new(
			&signer,
			Transaction::ElfScript(ElfScript::new(elf_bytes)),
			nonce,
		)?;
		let id = client.send_transaction(transaction)?;
		let transition_index = client.wait_for_transition(id, timeout).await?;

		println!("client_listen_addr: {listen_addr}");
		println!("transaction_id: {id}");
		println!("transaction included in transition: {transition_index:?}");
		Ok(())
	}

	fn load_elf_bytes(&self) -> Result<Vec<u8>, anyhow::Error> {
		if let Some(path) = self.resolve_workspace_binary_path()? {
			log::info!("resolved workspace binary '{}' to {}", self.elf, path.display());
			return Ok(std::fs::read(path)?);
		}

		let direct_path = PathBuf::from(&self.elf);
		if direct_path.exists() {
			log::info!("resolved direct ELF path {}", direct_path.display());
			return Ok(std::fs::read(direct_path)?);
		}

		anyhow::bail!(
			"could not resolve ELF '{}': checked workspace binary name first, then direct file path",
			self.elf
		);
	}

	fn resolve_workspace_binary_path(&self) -> Result<Option<PathBuf>, anyhow::Error> {
		let output = Command::new("cargo")
			.args(["metadata", "--format-version", "1", "--no-deps"])
			.output()?;
		if !output.status.success() {
			return Ok(None);
		}

		let metadata: CargoMetadata = serde_json::from_slice(&output.stdout)?;
		let target_directory = Path::new(&metadata.target_directory);
		let workspace_root = Path::new(&metadata.workspace_root);

		let mut candidate_names = vec![self.elf.clone()];
		if cfg!(windows) && !self.elf.ends_with(".exe") {
			candidate_names.push(format!("{}.exe", self.elf));
		}

		for name in candidate_names {
			let target_debug = target_directory.join("debug").join(&name);
			if target_debug.exists() {
				return Ok(Some(target_debug));
			}
			let target_release = target_directory.join("release").join(&name);
			if target_release.exists() {
				return Ok(Some(target_release));
			}
			// Fallback: workspace-local target directory in case metadata target differs.
			let workspace_debug = workspace_root.join("target").join("debug").join(&name);
			if workspace_debug.exists() {
				return Ok(Some(workspace_debug));
			}
			let workspace_release = workspace_root.join("target").join("release").join(&name);
			if workspace_release.exists() {
				return Ok(Some(workspace_release));
			}
		}

		Ok(None)
	}
}

impl or_file::SendElf {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let resolved = self.clone().resolve().await?;
		resolved.execute().await
	}
}
