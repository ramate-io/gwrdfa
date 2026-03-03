use super::TestResampleParabyzantineSpec;
use super::{
	TestResampleAgreementContainer, TestResampleAgreementDelta, TestResampleCertificateContainer,
	TestResampleCertificateDelta,
};
use crate::agreement::Subcommittee;
use gwrdfa_container::{ContainerEntityBuffer, ContainerEntityDraftBuffer};
use parabyzantine::agreement::ParabyzantineAgreementData;

pub struct TestResampleParabyzantineData<Index: Eq, Value: Eq + 'static, Sub: Subcommittee<Value>> {
	pub certificate_buffer:
		ContainerEntityBuffer<TestResampleCertificateContainer<Index, Value, Sub>>,
	pub agreement_buffer: ContainerEntityBuffer<TestResampleAgreementContainer<Index, Value, Sub>>,
}

impl<Index: Eq, Value: Eq + 'static, Sub: Subcommittee<Value>>
	TestResampleParabyzantineData<Index, Value, Sub>
{
	pub fn new() -> Self {
		Self {
			certificate_buffer: ContainerEntityBuffer::new(),
			agreement_buffer: ContainerEntityBuffer::new(),
		}
	}
}

impl<Index: Eq, Value: Eq + 'static, Sub: Subcommittee<Value>>
	ParabyzantineAgreementData<TestResampleParabyzantineSpec<Index, Value, Sub>>
	for TestResampleParabyzantineData<Index, Value, Sub>
{
	fn parabyzantine_agreement_certificate_buffer(
		&self,
	) -> &ContainerEntityBuffer<TestResampleCertificateContainer<Index, Value, Sub>> {
		&self.certificate_buffer
	}
	fn parabyzantine_agreement_certificate_buffer_mut(
		&mut self,
	) -> &mut ContainerEntityBuffer<TestResampleCertificateContainer<Index, Value, Sub>> {
		&mut self.certificate_buffer
	}
	fn parabyzantine_agreement_certificate_draft_buffer(
		&self,
	) -> ContainerEntityDraftBuffer<TestResampleCertificateDelta<Index, Value, Sub>> {
		ContainerEntityDraftBuffer::new()
	}
	fn parabyzantine_agreement_agreement_buffer(
		&self,
	) -> &ContainerEntityBuffer<TestResampleAgreementContainer<Index, Value, Sub>> {
		&self.agreement_buffer
	}
	fn parabyzantine_agreement_agreement_buffer_mut(
		&mut self,
	) -> &mut ContainerEntityBuffer<TestResampleAgreementContainer<Index, Value, Sub>> {
		&mut self.agreement_buffer
	}
	fn parabyzantine_agreement_agreement_draft_buffer(
		&self,
	) -> ContainerEntityDraftBuffer<TestResampleAgreementDelta<Index, Value, Sub>> {
		ContainerEntityDraftBuffer::new()
	}
}
