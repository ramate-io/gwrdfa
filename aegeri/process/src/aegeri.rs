pub mod data;
pub use data::AegeriData;

use aegeri_message::{AegeriSubcommittee, Index, Proposal};
use gwrdfa_resample::agreement::{
	std::{ConstantCommittee, MemoryAgreementData},
	ResampleAgreement,
};

pub struct Aegeri {
	/// This is where the parbyzantine data will go.
	data: AegeriData,
	///
	reample_agreement: ResampleAgreement<
		AegeriData,
		MemoryAgreementData<Index, Proposal, AegeriSubcommittee, ConstantCommittee>,
	>,
}
