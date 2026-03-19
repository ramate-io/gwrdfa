use aegeri_process::local_cluster::AegeriLocalClusterConfig;
use clap::Parser;
use orfile::Orfile;
use serde::{Deserialize, Serialize};
use std::time::Duration;

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
		log::info!("building cluster with count={} and topic={}", self.count, self.topic);
		let mut cluster = AegeriLocalClusterConfig::default()
			.with_count(self.count)
			.with_topic(self.topic.clone())
			.build()
			.await?;

		println!("topic: {}", self.topic);
		println!("count: {}", cluster.harts.len());
		for (i, (hart, addr)) in cluster.harts.iter().zip(cluster.listen_addrs.iter()).enumerate() {
			println!(
				"node[{i}] public_key={} address={}",
				hart.signer_public_key().to_hex_string(),
				addr
			);
		}

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
