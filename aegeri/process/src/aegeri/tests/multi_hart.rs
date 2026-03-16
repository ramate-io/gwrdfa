use super::super::test_utils::{
	active_harts_all_have_certificate_index,
	advance_multi_hart_consensus_step, assert_active_harts_have_certificate_index,
	assert_active_harts_lack_certificate_index, setup_multi_hart_harness,
};
use aegeri_message::{Index as AegeriIndex, IndexValue};

fn advance_steps(
	harts: &mut [super::super::AegeriHart],
	gossamer_channels: &mut [gossamer::GossamerChannels<gwrdfa_container::ContainerEntity>],
	active: &[usize],
	steps: usize,
) -> Result<(), anyhow::Error> {
	for _ in 0..steps {
		advance_multi_hart_consensus_step(harts, gossamer_channels, active)?;
	}
	Ok(())
}

fn drive_until_all_active_have_index(
	harts: &mut [super::super::AegeriHart],
	gossamer_channels: &mut [gossamer::GossamerChannels<gwrdfa_container::ContainerEntity>],
	active: &[usize],
	index: AegeriIndex,
	max_steps: usize,
) -> Result<(), anyhow::Error> {
	for _ in 0..max_steps {
		if active_harts_all_have_certificate_index(harts, active, index) {
			return Ok(());
		}
		advance_multi_hart_consensus_step(harts, gossamer_channels, active)?;
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
	anyhow::bail!(
		"did not reach expected certificate index within max steps for {index:?}; {snapshots}"
	)
}

#[tokio::test]
async fn test_multi_hart_consensus_across_varying_participation() -> Result<(), anyhow::Error> {
	let mut harness = setup_multi_hart_harness([1, 2, 3, 4, 5, 6, 7])?;
	assert_eq!(harness.genesis_subcommittee.size(), 7);

	let active7 = vec![0, 1, 2, 3, 4, 5, 6];
	let active5 = vec![0, 1, 2, 3, 4];
	let active4 = vec![0, 1, 2, 3];

	// Round 0 with all seven active should reach transition consensus.
	drive_until_all_active_have_index(
		&mut harness.harts,
		&mut harness.gossamer_channels,
		&active7,
		AegeriIndex::Transition(IndexValue::new(0)),
		24,
	)?;
	assert_active_harts_have_certificate_index(
		&harness.harts,
		&active7,
		AegeriIndex::Transition(IndexValue::new(0)),
	);

	// Round 1 with five active (quorum for committee of seven) should also reach consensus.
	drive_until_all_active_have_index(
		&mut harness.harts,
		&mut harness.gossamer_channels,
		&active5,
		AegeriIndex::Transition(IndexValue::new(1)),
		24,
	)?;
	assert_active_harts_have_certificate_index(
		&harness.harts,
		&active5,
		AegeriIndex::Transition(IndexValue::new(1)),
	);

	// Round 2 starts with only four active senders: below quorum, no confirmation consensus yet.
	advance_steps(&mut harness.harts, &mut harness.gossamer_channels, &active4, 3)?;
	assert_active_harts_lack_certificate_index(
		&harness.harts,
		&active4,
		AegeriIndex::Confirmation(IndexValue::new(2)),
	);
	assert_active_harts_lack_certificate_index(
		&harness.harts,
		&active4,
		AegeriIndex::Transition(IndexValue::new(2)),
	);

	// Once the fifth hart participates, consensus progresses and reaches transition.
	drive_until_all_active_have_index(
		&mut harness.harts,
		&mut harness.gossamer_channels,
		&active5,
		AegeriIndex::Confirmation(IndexValue::new(2)),
		24,
	)?;
	assert_active_harts_have_certificate_index(
		&harness.harts,
		&active5,
		AegeriIndex::Confirmation(IndexValue::new(2)),
	);

	Ok(())
}
