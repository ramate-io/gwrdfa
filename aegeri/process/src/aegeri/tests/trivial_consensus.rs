use super::super::test_utils::{
	advance_consensus_step, assert_certificate_set, assert_index_subcommittee_agreement_set,
	assert_index_value_agreement_set, setup_trivial_consensus_harness,
};
use aegeri_message::{
	BlockHeader, Confirmation, Index as AegeriIndex, IndexValue, Proposal as AegeriProposal,
	Transition, Availability,
};

#[tokio::test]
async fn test_aegeri_hart_trivial_consensus() -> Result<(), anyhow::Error> {
	let harness = setup_trivial_consensus_harness(1)?;
	let mut hart = harness.hart;
	let mut gossamer_channels = harness.gossamer_channels;
	let genesis_subcommittee = harness.genesis_subcommittee;

	hart = advance_consensus_step(hart, &mut gossamer_channels)?;
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
		],
	);
	assert_index_subcommittee_agreement_set(
		&hart,
		[(
			AegeriIndex::Block(IndexValue::genesis()),
			genesis_subcommittee
				.clone()
				.with_index(AegeriIndex::Block(IndexValue::genesis())),
		)],
	);
	assert_index_value_agreement_set(
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
		],
	);

	hart = advance_consensus_step(hart, &mut gossamer_channels)?;
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
		],
	);
	assert_index_subcommittee_agreement_set(
		&hart,
		[(
			AegeriIndex::Transition(IndexValue::genesis()),
			genesis_subcommittee
				.clone()
				.with_index(AegeriIndex::Transition(IndexValue::genesis())),
		)],
	);
	assert_index_value_agreement_set(
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
		],
	);

	hart = advance_consensus_step(hart, &mut gossamer_channels)?;
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
		],
	);
	assert_index_subcommittee_agreement_set(
		&hart,
		[(
			AegeriIndex::Availability(IndexValue::new(1)),
			genesis_subcommittee
				.clone()
				.with_index(AegeriIndex::Availability(IndexValue::new(1))),
		)],
	);
	assert_index_value_agreement_set(
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
		],
	);

	hart = advance_consensus_step(hart, &mut gossamer_channels)?;
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
				AegeriProposal::Availability(Availability::genesis()),
			),
		],
	);
	assert_index_subcommittee_agreement_set(
		&hart,
		[(
			AegeriIndex::Confirmation(IndexValue::new(1)),
			genesis_subcommittee
				.clone()
				.with_index(AegeriIndex::Confirmation(IndexValue::new(1))),
		)],
	);
	assert_index_value_agreement_set(
		&hart,
		[
			(
				AegeriIndex::Availability(IndexValue::new(0)),
				AegeriProposal::Availability(Availability::genesis()),
			),
			(
				AegeriIndex::Availability(IndexValue::new(1)),
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
		],
	);

	Ok(())
}
