use super::{
	AgreementContainer, AgreementDelta, CertificateContainer, CertificateDelta,
};
use crate::agreement::Subcommittee;
use gwrdfa_container::{ContainerEntity, ContainerEntityBuffer, ContainerEntityDraftBuffer};
use parabyzantine::agreement::ParabyzantineAgreementData;

pub struct AgreementParabyzantineData<Index: Eq, Value: Eq + 'static, Sub: Subcommittee<Value>> {
	pub certificate_buffer: ContainerEntityBuffer<CertificateContainer<Index, Value, Sub>>,
	pub agreement_buffer: ContainerEntityBuffer<AgreementContainer<Index, Value, Sub>>,
}

impl<Index: Eq, Value: Eq + 'static, Sub: Subcommittee<Value>>
	AgreementParabyzantineData<Index, Value, Sub>
{
	pub fn new() -> Self {
		Self {
			certificate_buffer: ContainerEntityBuffer::new(),
			agreement_buffer: ContainerEntityBuffer::new(),
		}
	}
}

impl<Index: Eq, Value: Eq + 'static, Sub: Subcommittee<Value>> ParabyzantineAgreementData
	for AgreementParabyzantineData<Index, Value, Sub>
{
	type CertificateEntity = ContainerEntity;
	type CertificateBuffer = ContainerEntityBuffer<CertificateContainer<Index, Value, Sub>>;
	type CertificateDraftBuffer = ContainerEntityDraftBuffer<CertificateDelta<Index, Value, Sub>>;
	type AgreementEntity = ContainerEntity;
	type AgreementBuffer = ContainerEntityBuffer<AgreementContainer<Index, Value, Sub>>;
	type AgreementDraftBuffer = ContainerEntityDraftBuffer<AgreementDelta<Index, Value, Sub>>;

	fn parabyzantine_agreement_certificate_buffer(
		&self,
	) -> &ContainerEntityBuffer<CertificateContainer<Index, Value, Sub>> {
		&self.certificate_buffer
	}

	fn parabyzantine_agreement_certificate_buffer_mut(
		&mut self,
	) -> &mut ContainerEntityBuffer<CertificateContainer<Index, Value, Sub>> {
		&mut self.certificate_buffer
	}

	fn parabyzantine_agreement_certificate_draft_buffer(
		&self,
	) -> ContainerEntityDraftBuffer<CertificateDelta<Index, Value, Sub>> {
		ContainerEntityDraftBuffer::new()
	}

	fn parabyzantine_agreement_agreement_buffer(
		&self,
	) -> &ContainerEntityBuffer<AgreementContainer<Index, Value, Sub>> {
		&self.agreement_buffer
	}

	fn parabyzantine_agreement_agreement_buffer_mut(
		&mut self,
	) -> &mut ContainerEntityBuffer<AgreementContainer<Index, Value, Sub>> {
		&mut self.agreement_buffer
	}

	fn parabyzantine_agreement_agreement_draft_buffer(
		&self,
	) -> ContainerEntityDraftBuffer<AgreementDelta<Index, Value, Sub>> {
		ContainerEntityDraftBuffer::new()
	}
}
