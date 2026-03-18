use crate::{FullClient, Index, Message, Transaction};
use aegeri_process::local_cluster::AegeriLocalClusterConfig;
use ml_dsa::{B32, MlDsa44, SigningKey};
use std::sync::Once;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

static LOG_INIT: Once = Once::new();

fn init_test_logger() {
	LOG_INIT.call_once(|| {
		let _ = env_logger::Builder::from_env(
			env_logger::Env::default()
				.default_filter_or("aegeri_full_client=debug,aegeri_process=debug,gossamer=info"),
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

	let topic = format!(
		"aegeri-full-client-resource-{}",
		SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos()
	);
	let cluster = AegeriLocalClusterConfig::default()
		.with_count(7)
		.with_topic(topic)
		.build()
		.await?;

	// Use one live peer for bootstrap.
	let bootstrap_peers = vec![(cluster.listen_addrs[0].clone(), cluster.harts[0].signer_public_key())];

	// Keep consensus progressing in the cluster while the client watches as a non-participant.
	let mut harts = cluster.harts;
	let hart_ticker = tokio::spawn(async move {
		loop {
			for hart in harts.iter_mut() {
				hart.tick();
			}
			tokio::time::sleep(Duration::from_millis(150)).await;
		}
	});

	// Give peers a brief moment to connect before bootstrap.
	tokio::time::sleep(Duration::from_secs(2)).await;

	let test_result = async {
		let (mut client, _listen_addr) = FullClient::bootstrap_non_participant(1, bootstrap_peers).await?;
		let leave = leave_message(99, b"resource-acquiring-leave")?;
		let index = client
			.send_and_wait_for_transition(leave, Duration::from_secs(90))
			.await?;
		assert!(matches!(index, Index::Transition(_)));
		Ok(()) as Result<(), anyhow::Error>
	}
	.await;

	hart_ticker.abort();
	test_result
}
