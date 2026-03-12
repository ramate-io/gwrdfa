pub mod data;

use aegeri_message::{AegeriSubcommittee, Index, Proposal};
use gwrdfa_resample::agreement::{
	std::{ConstantCommittee, MemoryAgreementData},
	ResampleAgreement,
};

pub struct Aegeri {
	/// This is where the parbyzantine data will go.
	data: (),
	///
	reample_agreement: ResampleAgreement<
		MemoryAgreementData<Index, Proposal, AegeriSubcommittee, ConstantCommittee>,
	>,
}
