use super::{IndexSubcommitteeAgreement, Subcommittee};
use parabyzantine::{
	agreement::{ParabyzantineAgreementDataBinding, ParabyzantineAgreementDataSpec},
	buffer::Inferences,
	NoOp,
};

pub trait Sampler<
	Index: Eq,
	Value: Eq + 'static,
	Sub: Subcommittee<Value>,
	SubAgree: IndexSubcommitteeAgreement<Index, Sub, Value>,
	Binding: ParabyzantineAgreementDataBinding,
>: Sized
{
	/// Given a value and the subcommittee agreeement which gave that value,
	/// the sampler has the option to insert agreements into the buffer.
	///
	/// Note, that this is an offline rule set,
	/// so it does not depend on a consensus value itself.
	/// If you want to write a protocol which changes the rule set based on a consensus value,
	/// there are two major patterns:
	/// 1. Write your [Sampler] so that values themselves can effectively update the sampler to reflect the new rule set.
	/// 2. Write your [Sampler] so that it inserts agreements which defer the actual subcommittee election to a later stage, e.g., ParabyzantineTask.
	fn elect_subcommittees_from_consensus_value(
		&mut self,
		value: &Value,
		agreement: &SubAgree,
		agreeement_inferences: &mut Inferences<
			<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementEntity,
			<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementBuffer,
			<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementDraftBuffer,
		>,
	);

	/// Given a hung subcommittee agreement, the sampler has the option to insert agreements into the buffer.
	fn elect_subcommittees_from_hung_value(
		&mut self,
		agreement: &SubAgree,
		agreement_inferences: &mut Inferences<
			<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementEntity,
			<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementBuffer,
			<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementDraftBuffer,
		>,
	);
}

impl<
		Index: Eq,
		Value: Eq + 'static,
		Sub: Subcommittee<Value>,
		SubAgree: IndexSubcommitteeAgreement<Index, Sub, Value>,
		Binding: ParabyzantineAgreementDataBinding,
	> Sampler<Index, Value, Sub, SubAgree, Binding> for NoOp
{
	fn elect_subcommittees_from_consensus_value(
		&mut self,
		_value: &Value,
		_agreement: &SubAgree,
		_agreement_inferences: &mut Inferences<
			<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementEntity,
			<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementBuffer,
			<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementDraftBuffer,
		>,
	) {
	}
	fn elect_subcommittees_from_hung_value(
		&mut self,
		_agreement: &SubAgree,
		_agreement_inferences: &mut Inferences<
			<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementEntity,
			<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementBuffer,
			<Binding::Spec as ParabyzantineAgreementDataSpec>::AgreementDraftBuffer,
		>,
	) {
	}
}

#[cfg(test)]
pub mod test {
	use std::collections::HashSet;

	pub struct TestSampler {}
}
