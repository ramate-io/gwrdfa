use crate::aegeri::{AegeriHart, AegeriHartError};
#[cfg(test)]
use crate::aegeri_message::Index as AegeriIndex;
use crate::aegeri_message::{AegeriSubcommittee, Availability, PublicKey};
use crate::gossamer::local_cluster::{LocalClusterConfig, LocalClusterError};
use crate::gossamer::Multiaddr;
use gwrdfa_container::ContainerEntity;
use ml_dsa::{MlDsa44, SigningKey, B32};

#[derive(thiserror::Error, Debug)]
pub enum AegeriLocalClusterError {
	#[error("error building Aegeri hart: {0}")]
	BuildError(#[from] AegeriHartError),
	#[error("error building Gossamer local cluster: {0}")]
	GossamerLocalCluster(#[from] LocalClusterError),
}

#[derive(Debug, Clone)]
pub struct AegeriLocalClusterConfig {
	/// The number of Aegeri harts to start.
	pub count: usize,
	/// The topic to use for all gossamer nodes.
	pub topic: String,
}

pub struct AegeriLocalCluster {
	pub harts: Vec<AegeriHart>,
	pub listen_addrs: Vec<Multiaddr>,
	pub genesis_subcommittee: AegeriSubcommittee,
}

impl AegeriLocalClusterConfig {
	pub fn with_count(mut self, count: usize) -> Self {
		self.count = count;
		self
	}

	pub fn with_topic(mut self, topic: String) -> Self {
		self.topic = topic;
		self
	}
}

impl Default for AegeriLocalClusterConfig {
	fn default() -> Self {
		Self { count: 3, topic: "aegeri".to_string() }
	}
}

impl AegeriLocalClusterConfig {
	pub async fn build(self) -> Result<AegeriLocalCluster, AegeriLocalClusterError> {
		let count = self.count;
		let gossamers = LocalClusterConfig::default()
			.with_count(count)
			.with_topic(self.topic)
			.build::<ContainerEntity>()
			.await?;

		let signers = (0..count)
			.map(|i| SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![(i as u8) + 1; 32])))
			.collect::<Vec<_>>();
		let members = signers.iter().map(PublicKey::new).collect::<Vec<_>>();
		let genesis_subcommittee = AegeriSubcommittee::genesis().with_members(members.into_iter());
		let availability = Availability::genesis();

		let mut harts = Vec::with_capacity(count);
		let mut listen_addrs = Vec::with_capacity(count);
		for (signer, (gossamer, listen_addr)) in signers.into_iter().zip(gossamers.into_iter()) {
			let hart = AegeriHart::from_gossamer(gossamer)?
				.with_signer(signer)
				.with_genesis(genesis_subcommittee.clone(), availability.clone());
			harts.push(hart);
			listen_addrs.push(listen_addr);
		}

		Ok(AegeriLocalCluster { harts, listen_addrs, genesis_subcommittee })
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::aegeri_message::IndexValue;
	use anyhow::Result;
	use gwrdfa_resample::agreement::std::NextRound;
	use std::sync::Once;
	use tokio::time::Duration;

	static LOG_INIT: Once = Once::new();

	fn init_test_logger() {
		LOG_INIT.call_once(|| {
			let _ = env_logger::Builder::from_env(
				env_logger::Env::default()
					.default_filter_or("gossamer=debug,aegeri_process=debug,aegeri_message=debug"),
			)
			.is_test(true)
			.try_init();
		});
	}

	fn hart_has_index(hart: &AegeriHart, index: AegeriIndex) -> bool {
		hart.certificate_set().into_iter().any(|(candidate, _)| candidate == index)
	}

	fn hart_has_index_agreement(hart: &AegeriHart, index: AegeriIndex) -> bool {
		hart.index_subcommittee_agreement_set()
			.into_iter()
			.any(|(candidate, _)| candidate == index)
	}

	fn hart_current_index_agreement(hart: &AegeriHart) -> Option<AegeriIndex> {
		hart.index_subcommittee_agreement_set()
			.into_iter()
			.map(|(index, _)| index)
			.max()
	}

	async fn tick_active(harts: &mut [AegeriHart], active: &[usize], steps: usize) {
		for _ in 0..steps {
			for &i in active {
				harts[i].tick();
			}
			tokio::time::sleep(Duration::from_millis(200)).await;
		}
	}

	async fn drive_until_all_active_have_index(
		harts: &mut [AegeriHart],
		active: &[usize],
		index: AegeriIndex,
		max_steps: usize,
	) -> Result<()> {
		for step in 0..max_steps {
			if active.iter().all(|&i| hart_has_index(&harts[i], index)) {
				log::debug!("local-cluster: reached target index {index:?} at step {step}");
				return Ok(());
			}

			if step % 10 == 0 {
				let snapshot = active
					.iter()
					.map(|&i| {
						let known = harts[i]
							.certificate_set()
							.into_iter()
							.map(|(idx, _)| idx)
							.collect::<std::collections::BTreeSet<_>>();
						format!("{i}:{known:?}")
					})
					.collect::<Vec<_>>()
					.join(" | ");
				log::debug!(
					"local-cluster: waiting for {index:?}, step={step}, active snapshot={snapshot}"
				);
			}

			tick_active(harts, active, 1).await;
		}

		let snapshots = active
			.iter()
			.map(|&i| {
				let known = harts[i]
					.certificate_set()
					.into_iter()
					.map(|(idx, _)| idx)
					.collect::<std::collections::BTreeSet<_>>();
				format!("hart {i}: {known:?}")
			})
			.collect::<Vec<_>>()
			.join("; ");
		anyhow::bail!("did not reach expected index {index:?} within max steps; {snapshots}")
	}

	async fn drive_until_all_active_have_index_agreement(
		harts: &mut [AegeriHart],
		active: &[usize],
		index: AegeriIndex,
		max_steps: usize,
	) -> Result<()> {
		for _ in 0..max_steps {
			if active.iter().all(|&i| hart_has_index_agreement(&harts[i], index)) {
				return Ok(());
			}
			tick_active(harts, active, 1).await;
		}

		let snapshots = active
			.iter()
			.map(|&i| {
				let known = harts[i]
					.index_subcommittee_agreement_set()
					.into_iter()
					.map(|(idx, _)| idx)
					.collect::<std::collections::BTreeSet<_>>();
				format!("hart {i}: {known:?}")
			})
			.collect::<Vec<_>>()
			.join("; ");
		anyhow::bail!(
			"did not reach expected index agreement {index:?} within max steps; {snapshots}"
		)
	}

	async fn drive_until_all_active_share_same_index_agreement(
		harts: &mut [AegeriHart],
		active: &[usize],
		max_steps: usize,
	) -> Result<AegeriIndex> {
		for _ in 0..max_steps {
			let current = active
				.iter()
				.map(|&i| hart_current_index_agreement(&harts[i]))
				.collect::<Vec<_>>();
			if current.iter().all(Option::is_some) {
				let first = current[0].expect("checked is_some");
				if current.iter().all(|candidate| *candidate == Some(first)) {
					return Ok(first);
				}
			}
			tick_active(harts, active, 1).await;
		}

		let snapshots = active
			.iter()
			.map(|&i| format!("hart {i}: {:?}", hart_current_index_agreement(&harts[i])))
			.collect::<Vec<_>>()
			.join("; ");
		anyhow::bail!("active harts did not converge on same index agreement; {snapshots}")
	}

	async fn find_stalled_shared_index_under_active_set(
		harts: &mut [AegeriHart],
		active: &[usize],
		max_index_hops: usize,
		stall_ticks: usize,
	) -> Result<AegeriIndex> {
		let mut shared =
			drive_until_all_active_share_same_index_agreement(harts, active, 120).await?;
		for _ in 0..max_index_hops {
			let next = shared.next().ok_or_else(|| {
				anyhow::anyhow!("shared index {shared:?} did not have a next round")
			})?;

			let mut advanced = false;
			for _ in 0..stall_ticks {
				tick_active(harts, active, 1).await;
				if active.iter().all(|&i| hart_has_index_agreement(&harts[i], next)) {
					advanced = true;
					shared = next;
					break;
				}
			}

			if !advanced {
				return Ok(shared);
			}
		}

		anyhow::bail!("active set kept advancing without stalling under reduced participation")
	}

	#[tokio::test]
	#[ignore = "Acquires ephemeral ports; run with --ignored to opt in."]
	async fn test_local_cluster_multi_hart_consensus_varying_participation() -> Result<()> {
		init_test_logger();
		let active7 = vec![0, 1, 2, 3, 4, 5, 6];
		let active5 = vec![0, 1, 2, 3, 4];
		let active4 = vec![0, 1, 2, 3];

		// One local cluster for the whole scenario progression.
		let mut cluster = AegeriLocalClusterConfig::default().with_count(7).build().await?;
		assert_eq!(cluster.genesis_subcommittee.size(), 7);
		assert_eq!(cluster.listen_addrs.len(), 7);
		tokio::time::sleep(Duration::from_secs(15)).await;

		// Scenario 1: first round, all seven active -> transition consensus.
		drive_until_all_active_have_index(
			&mut cluster.harts,
			&active7,
			AegeriIndex::Transition(IndexValue::new(0)),
			120,
		)
		.await?;

		// Scenario 2: second round, five active -> consensus reached.
		drive_until_all_active_have_index(
			&mut cluster.harts,
			&active5,
			AegeriIndex::Confirmation(IndexValue::new(1)),
			120,
		)
		.await?;

		// Scenario 3: third round, four active first -> no consensus, then fifth joins.
		let stalled_index =
			find_stalled_shared_index_under_active_set(&mut cluster.harts, &active4, 24, 8).await?;
		let next_index = stalled_index.next().ok_or_else(|| {
			anyhow::anyhow!("shared index {stalled_index:?} did not have a next round")
		})?;
		let only_the_following_harts_should_send = format!(
			"Only the following harts should send: {}",
			active4.iter().fold(String::new(), |acc, &i| acc
				+ &format!("hart {i}: {}", cluster.harts[i].signer_public_key().to_string())
				+ ", ")
		);
		for &i in &active4 {
			let signer_public_key = cluster.harts[i].signer_public_key();
			assert!(
				!hart_has_index_agreement(&cluster.harts[i], next_index),
				"hart {i}: {signer_public_key} unexpectedly reached next index agreement {next_index:?} after active4 stalled at {stalled_index:?}\n{only_the_following_harts_should_send}"
			);
		}

		// Let the fifth hart catch up to the same stalled index first.
		drive_until_all_active_have_index_agreement(
			&mut cluster.harts,
			&active5,
			stalled_index,
			120,
		)
		.await?;

		drive_until_all_active_have_index_agreement(&mut cluster.harts, &active5, next_index, 120)
			.await?;

		Ok(())
	}
}
