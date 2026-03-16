use super::super::{AegeriHart, AegeriHartError};
use aegeri_message::{AegeriSubcommittee, Availability};

#[test]
fn test_aegeri_hart_with_genesis_subcommittee() -> Result<(), AegeriHartError> {
	let (hart, _gossamer_channels) = AegeriHart::mock()?;
	let subcommittee = AegeriSubcommittee::genesis();
	let availability = Availability::genesis();
	let hart = hart.with_genesis(subcommittee, availability);
	assert_eq!(hart.index_subcommittee_agreements().count(), 1);
	Ok(())
}
