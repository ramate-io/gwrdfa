use super::super::test_utils::{
	advance_consensus_step, assert_certificate_set, availability_from_ids, send_transaction,
	setup_trivial_consensus_harness,
};
use aegeri_message::{
	Availability, BlockHeader, Confirmation, Index as AegeriIndex, IndexValue,
	Proposal as AegeriProposal, Transaction, Transition,
};
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
