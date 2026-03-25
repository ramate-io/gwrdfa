use crate::common::PeerList;
use aegeri_process::local_cluster::AegeriLocalClusterConfig;
use clap::Parser;
use orfile::Orfile;
use serde::{Deserialize, Serialize};
use std::{fs::File, time::Duration};
use crate::common::GossamerCliConfig;

/// Runs a local cluster and logs out the topic, public keys, and addresses of the nodes.
///
/// Note: this does not allow setting the public keys or addresses of the nodes.
#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
#[clap(help_expected = true)]
pub struct QuickRun {
	/// The number of nodes to start
	#[clap(long, default_value = "4")]
	count: usize,
	#[clap(flatten)]
	gossamer: GossamerCliConfig,
	/// The file to write the peer list to.
	#[clap(long, default_value = "aegeri.peer-list.json")]
	output_file: String,
}

impl QuickRun {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		log::info!(
			"building cluster with count={} and topic={}",
			self.count,
			self.gossamer.topic
		);
		let mut cluster = AegeriLocalClusterConfig::default()
			.with_count(self.count)
			.with_topic(self.gossamer.topic.clone())
			.with_gossipsub_max_transmit_size(self.gossamer.gossipsub_max_transmit_size)
			.build()
			.await?;

		println!("topic: {}", self.gossamer.topic);
		println!("count: {}", cluster.harts.len());

		// Build and log the peer list
		let mut peer_list = PeerList::new();
		for (i, (hart, addr)) in cluster.harts.iter().zip(cluster.listen_addrs.iter()).enumerate() {
			println!(
				"node[{i}] public_key={} address={}",
				hart.signer_public_key().to_hex_string(),
				addr
			);
			peer_list.add_peer(hart.signer_public_key());
			peer_list.add_multiaddr(addr.clone());
		}

		// Write the peer list to the file
		// Creating the file if it doesn't exist or clearing out the old file and overwriting it if it does.
		if std::fs::exists(&self.output_file)? {
			std::fs::remove_file(&self.output_file)?;
		}

		serde_json::to_writer_pretty(
			File::options().create(true).write(true).open(&self.output_file)?,
			&peer_list,
		)?;
		println!("wrote peer list to {}", self.output_file);

		println!("cluster running; ticking {} harts (Ctrl-C to stop)", cluster.harts.len());

		loop {
			tokio::select! {
				_ = tokio::signal::ctrl_c() => {
					println!("received Ctrl-C, shutting down local cluster");
					break;
				}
				_ = tokio::time::sleep(Duration::from_millis(150)) => {
					for hart in cluster.harts.iter_mut() {
						log::info!("hart {} updated to consensus indices {:?}", hart.signer_public_key(), hart.index_subcommittee_agreement_set().iter().map(|(index, _)| index).collect::<Vec<_>>());
						hart.tick();
					}
				}
			}
		}
		Ok(())
	}
}

impl or_file::QuickRun {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let resolved = self.clone().resolve().await?;
		resolved.execute().await
	}
}
