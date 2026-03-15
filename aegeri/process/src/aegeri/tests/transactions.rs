use super::super::test_utils::{
	advance_consensus_step, assert_certificate_set, availability_from_ids, send_transaction,
	send_transaction_with_public_key, setup_trivial_consensus_harness,
};
use aegeri_message::{
	Availability, BlockHeader, Confirmation, Index as AegeriIndex, IndexValue,
	Proposal as AegeriProposal, Transaction, Transition,
};
use std::collections::BTreeSet;
use std::time::Duration;

#[tokio::test]
async fn test_transactions_appear_in_next_round_availability_after_slot_wait(
) -> Result<(), anyhow::Error> {
	let harness = setup_trivial_consensus_harness(1)?;
	let mut hart = harness.hart;
	let mut gossamer_channels = harness.gossamer_channels;

	let tx_a = send_transaction(&mut gossamer_channels, 7, Transaction::Join, [1; 32])?;
	let tx_b = send_transaction(&mut gossamer_channels, 8, Transaction::Leave, [2; 32])?;
	let tx_c = send_transaction(&mut gossamer_channels, 9, Transaction::Join, [3; 32])?;

	// Transactions are inserted on the next tick; then wait until they are eligible for
	// selection by mempool slot policy (< current_slot - 1).
	advance_consensus_step(&mut hart, &mut gossamer_channels)?;
	advance_consensus_step(&mut hart, &mut gossamer_channels)?;
	let wait_ms = hart.mempool_slot_width_ms() * 2 + 20;
	tokio::time::sleep(Duration::from_millis(wait_ms)).await;

	advance_consensus_step(&mut hart, &mut gossamer_channels)?;
	advance_consensus_step(&mut hart, &mut gossamer_channels)?;

	let next_round_availability = availability_from_ids([tx_a, tx_b, tx_c]);
	assert_certificate_set(
		&hart,
		[
			(
				AegeriIndex::Availability(IndexValue::genesis()),
				AegeriProposal::Availability(Availability::genesis()),
			),
			(
				AegeriIndex::Confirmation(IndexValue::genesis()),
				AegeriProposal::Confirmation(Confirmation::genesis()),
			),
			(
				AegeriIndex::Block(IndexValue::genesis()),
				AegeriProposal::BlockHeader(BlockHeader::genesis()),
			),
			(
				AegeriIndex::Transition(IndexValue::genesis()),
				AegeriProposal::Transition(Transition::genesis()),
			),
			(
				AegeriIndex::Availability(IndexValue::new(1)),
				AegeriProposal::Availability(next_round_availability),
			),
		],
	);

	Ok(())
}

#[tokio::test]
async fn test_transactions_progress_round_and_apply_joiners_leavers() -> Result<(), anyhow::Error> {
	let harness = setup_trivial_consensus_harness(1)?;
	let mut hart = harness.hart;
	let mut gossamer_channels = harness.gossamer_channels;

	let joiner_a =
		send_transaction_with_public_key(&mut gossamer_channels, 17, Transaction::Join, [11; 32])?;
	let leaver =
		send_transaction_with_public_key(&mut gossamer_channels, 18, Transaction::Leave, [12; 32])?;
	let joiner_b =
		send_transaction_with_public_key(&mut gossamer_channels, 19, Transaction::Join, [13; 32])?;

	let expected_round_one_availability = availability_from_ids([joiner_a.id, leaver.id, joiner_b.id]);
	let wait_ms = hart.mempool_slot_width_ms() * 2 + 20;

	// Drive round 0 to completion and wait for slot eligibility.
	advance_consensus_step(&mut hart, &mut gossamer_channels)?;
	advance_consensus_step(&mut hart, &mut gossamer_channels)?;
	tokio::time::sleep(Duration::from_millis(wait_ms)).await;
	advance_consensus_step(&mut hart, &mut gossamer_channels)?;
	advance_consensus_step(&mut hart, &mut gossamer_channels)?;

	// Round 1 availability must include all injected transactions.
	assert!(hart.certificate_set().contains(&(
		AegeriIndex::Availability(IndexValue::new(1)),
		AegeriProposal::Availability(expected_round_one_availability.clone()),
	)));

	// Round 1 confirmation must preserve the selected transaction set.
	advance_consensus_step(&mut hart, &mut gossamer_channels)?;
	assert!(hart.certificate_set().contains(&(
		AegeriIndex::Confirmation(IndexValue::new(1)),
		AegeriProposal::Confirmation(Confirmation::from_transactions(
			expected_round_one_availability.transactions().clone(),
		)),
	)));

	// Round 1 block header must preserve the same transaction set.
	advance_consensus_step(&mut hart, &mut gossamer_channels)?;
	assert!(hart.certificate_set().contains(&(
		AegeriIndex::Block(IndexValue::new(1)),
		AegeriProposal::BlockHeader(BlockHeader::from_transactions(
			expected_round_one_availability.transactions().clone(),
		)),
	)));

	// Round 1 transition must apply joiners and leavers from executed transactions.
	advance_consensus_step(&mut hart, &mut gossamer_channels)?;
	let transition = hart
		.certificate_set()
		.into_iter()
		.find_map(|(index, proposal)| match (index, proposal) {
			(AegeriIndex::Transition(IndexValue(1)), AegeriProposal::Transition(transition)) => {
				Some(transition)
			}
			_ => None,
		})
		.expect("missing transition certificate for round 1");

	assert_eq!(
		transition.block().ids(),
		expected_round_one_availability.transactions().ids()
	);
	assert_eq!(transition.state_root().as_bytes(), &[0; 0]);

	let expected_joiners =
		BTreeSet::from([joiner_a.public_key.clone(), joiner_b.public_key.clone()]);
	let expected_leavers = BTreeSet::from([leaver.public_key.clone()]);
	let actual_joiners = transition.join_set().joiners().iter().cloned().collect::<BTreeSet<_>>();
	let actual_leavers = transition.join_set().leavers().iter().cloned().collect::<BTreeSet<_>>();
	assert_eq!(actual_joiners, expected_joiners);
	assert_eq!(actual_leavers, expected_leavers);

	// Keep existing round-state expectation behavior intact.
	assert_certificate_set(
		&hart,
		[
			(
				AegeriIndex::Availability(IndexValue::genesis()),
				AegeriProposal::Availability(Availability::genesis()),
			),
			(
				AegeriIndex::Confirmation(IndexValue::genesis()),
				AegeriProposal::Confirmation(Confirmation::genesis()),
			),
			(
				AegeriIndex::Block(IndexValue::genesis()),
				AegeriProposal::BlockHeader(BlockHeader::genesis()),
			),
			(
				AegeriIndex::Transition(IndexValue::genesis()),
				AegeriProposal::Transition(Transition::genesis()),
			),
			(
				AegeriIndex::Availability(IndexValue::new(1)),
				AegeriProposal::Availability(expected_round_one_availability),
			),
			(
				AegeriIndex::Confirmation(IndexValue::new(1)),
				AegeriProposal::Confirmation(Confirmation::from_transactions(
					transition.block().clone(),
				)),
			),
			(
				AegeriIndex::Block(IndexValue::new(1)),
				AegeriProposal::BlockHeader(BlockHeader::from_transactions(
					transition.block().clone(),
				)),
			),
			(
				AegeriIndex::Transition(IndexValue::new(1)),
				AegeriProposal::Transition(transition),
			),
		],
	);

	Ok(())
}
