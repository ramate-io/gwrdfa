use crate::{FullClient, Index, Message, Transaction};
use aegeri_process::local_cluster::AegeriLocalClusterConfig;
use ml_dsa::{MlDsa44, SigningKey, B32};
use std::sync::Once;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

static LOG_INIT: Once = Once::new();

fn init_test_logger() {
	LOG_INIT.call_once(|| {
		let _ = env_logger::Builder::from_env(
			env_logger::Env::default()
				.default_filter_or("aegeri_full_client=debug,aegeri_process=debug/client:*"),
		)
		.is_test(true)
		.try_init();
	});
}

fn leave_message(seed: u8, nonce: &[u8]) -> Result<Message<Transaction>, anyhow::Error> {
	let signer = SigningKey::<MlDsa44>::from_seed(&B32::from_iter(vec![seed; 32]));
	Ok(Message::<Transaction>::try_new(
		&signer,
		Transaction::Leave,
		aegeri_message::Nonce::new(nonce),
	)?)
}

#[tokio::test]
#[ignore = "Acquires ports/network resources; run with --ignored to opt in."]
async fn test_bootstrap_non_participant_leave_reaches_transition() -> Result<(), anyhow::Error> {
	init_test_logger();

	log::debug!("starting test_bootstrap_non_participant_leave_reaches_transition");

	let topic = format!(
		"aegeri-full-client-resource-{}",
		SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos()
	);
	let cluster = AegeriLocalClusterConfig::default()
		.with_count(7)
		.with_topic(topic.clone())
		.build()
		.await?;

	// Use one live peer for bootstrap.
	let bootstrap_peers =
		vec![(cluster.listen_addrs[0].clone(), cluster.harts[0].signer_public_key())];

	// Keep consensus progressing in the cluster while the client watches as a non-participant.
	let mut harts = cluster.harts;
	let hart_ticker = tokio::spawn(async move {
		loop {
			for hart in harts.iter_mut() {
				log::debug!("\n\n===\nclient: hart: tick: {:?}\n====", hart.signer_public_key());
				hart.tick();
			}
			tokio::time::sleep(Duration::from_millis(150)).await;
		}
	});

	let test_result = async {
		// Waiting for 7 peers instead of 5 or 1 should make this wait deeper into the
		// network startup before attempting to send transactions.
		let (mut client, _listen_addr) =
			FullClient::bootstrap_non_participant(topic, 6, bootstrap_peers).await?;
		let leave = leave_message(99, b"resource-acquiring-leave")?;
		let index = client
			.send_and_wait_for_transition(leave, Duration::from_secs(120))
			.await
			.map_err(|e| {
				log::debug!("client: failed to send and wait for transition: {}", e);
				anyhow::anyhow!("failed to send and wait for transition: {}", e)
			})?;
		assert!(matches!(index, Index::Transition(_)));
		Ok(()) as Result<(), anyhow::Error>
	}
	.await;

	hart_ticker.abort();
	test_result
}
