use aegeri_message::{AegeriSubcommittee, Index, Proposal};
use gwrdfa_resample::agreement::std::{ConstantCommittee, MemoryAgreementData};

pub struct Aegeri {
	/// This is where the parbyzantine data will go.
	data: (),
	///
	reample_agreement: MemoryAgreementData<Index, Proposal, AegeriSubcommittee, ConstantCommittee>,
}
