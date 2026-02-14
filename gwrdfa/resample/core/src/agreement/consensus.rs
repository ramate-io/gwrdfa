use parabyzantine::{
	agreement::{ParabyzantineAgreementBinding, ParabyzantineAgreementSpec},
	buffer::Inferences,
	NoOp,
};

#[derive(Debug, Clone, Copy)]
pub enum Condition<Value: Eq> {
	Consensus(Value),
	Hung,
	InProgress,
}

pub trait ResampleAgreementConsensusUpdate<
	Index: Eq,
	Value: Eq,
	Sender: Eq,
	Binding: ParabyzantineAgreementBinding,
>: Sized
{
	fn insert_resample_agreement_consensus_agreement(
		&mut self,
		index: &Index,
		value: &Value,
		agreement_inferences: &mut Inferences<
			<Binding::Spec as ParabyzantineAgreementSpec>::AgreementEntity,
			<Binding::Spec as ParabyzantineAgreementSpec>::AgreementBuffer,
			<Binding::Spec as ParabyzantineAgreementSpec>::AgreementDraftBuffer,
		>,
	);
}

impl<Index: Eq, Value: Eq, Sender: Eq, Binding: ParabyzantineAgreementBinding>
	ResampleAgreementConsensusUpdate<Index, Value, Sender, Binding> for NoOp
{
	fn insert_resample_agreement_consensus_agreement(
		&mut self,
		_index: &Index,
		_value: &Value,
		_agreement_inferences: &mut Inferences<
			<Binding::Spec as ParabyzantineAgreementSpec>::AgreementEntity,
			<Binding::Spec as ParabyzantineAgreementSpec>::AgreementBuffer,
			<Binding::Spec as ParabyzantineAgreementSpec>::AgreementDraftBuffer,
		>,
	) {
	}
}
