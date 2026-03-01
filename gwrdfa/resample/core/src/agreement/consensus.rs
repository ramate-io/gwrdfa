use parabyzantine::{
	agreement::{ParabyzantineAgreementDataBinding, ParabyzantineAgreementDataSpec},
	buffer::Inferences,
	NoOp,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Condition<Value: Eq> {
	Consensus(Value),
	Hung,
	InProgress,
}

pub trait ResampleAgreementConsensusUpdate<
	Index: Eq,
	Value: Eq,
	Sender: Eq,
	Binding: ParabyzantineAgreementDataBinding,
>: Sized
{
	fn insert_resample_agreement_consensus_agreement(
		&mut self,
		index: &Index,
		value: &Value,
		agreement_inferences: &mut Inferences<
			<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementEntity,
			<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementBuffer,
			<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementDraftBuffer,
		>,
	);
}

impl<Index: Eq, Value: Eq, Sender: Eq, Binding: ParabyzantineAgreementDataBinding>
	ResampleAgreementConsensusUpdate<Index, Value, Sender, Binding> for NoOp
{
	fn insert_resample_agreement_consensus_agreement(
		&mut self,
		_index: &Index,
		_value: &Value,
		_agreement_inferences: &mut Inferences<
			<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementEntity,
			<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementBuffer,
			<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementDraftBuffer,
		>,
	) {
	}
}
