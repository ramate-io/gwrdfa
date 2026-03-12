pub mod data;
pub use data::AegeriData;

use aegeri_message::{
	AegeriSubcommittee, Index as AegeriIndex, Proposal as AegeriProposal, UnifiedMessage,
};
use gossamer::{
	container::GossamerContainer, hart::gossamer_messages::GossamerMessages, hart::GossamerHart,
	Gossamer, GossamerConfig, GossamerConfigError, Multiaddr, Out,
};
use gwrdfa_container::query::matching_tuple::{MatchingTuple, MatchingTupleQuery};
use gwrdfa_resample::agreement::{
	std::{ConstantCommittee, MemoryAgreementData},
	ResampleAgreement,
};

/// A [AegeriHart] is a [Hart] that implements the Aegeri protocol.
pub struct AegeriHart {
	/// This is where the parbyzantine data will go.
	data: AegeriData,

	/// Message protocol is gossamer messages over [UnifiedMessage].
	message: GossamerHart<AegeriData, AegeriGossamerMessages>,

	/// Agreement protocol is resample agreement.
	agreement: ResampleAgreement<
		AegeriData,
		MemoryAgreementData<AegeriIndex, AegeriProposal, AegeriSubcommittee, ConstantCommittee>,
	>,
}

impl AegeriHart {
	pub async fn spawn_tokio(
		config: GossamerConfig,
	) -> Result<(Self, Multiaddr), GossamerConfigError> {
		let (gossamer, listen_addr) = Gossamer::spawn_tokio(config).await?;

		Ok((
			Self {
				data: AegeriData::new(),
				message: GossamerHart::new(gossamer, AegeriGossamerMessages),
				agreement: ResampleAgreement::new(MemoryAgreementData::new()),
			},
			listen_addr,
		))
	}
}

pub struct AegeriGossamerMessages;

impl GossamerMessages<AegeriData> for AegeriGossamerMessages {
	type Message = UnifiedMessage;
	type OutQuery<'a> =
		MatchingTupleQuery<'a, GossamerContainer<UnifiedMessage>, (Out, UnifiedMessage)>;
	type OutQueryPlan = MatchingTuple<(Out, UnifiedMessage)>;

	fn gossamer_messages_out_plan(&mut self) -> MatchingTuple<(Out, UnifiedMessage)> {
		MatchingTuple::new()
	}
}
