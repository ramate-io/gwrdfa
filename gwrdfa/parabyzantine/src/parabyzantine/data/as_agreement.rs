use crate::parabyzantine::agreement::{ParabyzantineAgreementData, ParabyzantineAgreementSpec};
use crate::parabyzantine::data::{ParabyzantineData, ParabyzantineSpec};

/// Blanket implementation for the agreement spec.
///
/// Downcasting the world to an agreement world.
///
/// Note that because of blanket implementations on the Data,
/// we don't also have blanket implementations here.
impl<Spec: ParabyzantineSpec> ParabyzantineAgreementSpec for Spec {
	type CertificateEntity = Spec::CertificateEntity;
	type CertificateBuffer = Spec::CertificateBuffer;
	type CertificateDraftBuffer = Spec::CertificateDraftBuffer;
	type AgreementEntity = Spec::AgreementEntity;
	type AgreementBuffer = Spec::AgreementBuffer;
	type AgreementDraftBuffer = Spec::AgreementDraftBuffer;
}

/// Blanket implementation for the agreement data.
impl<Spec: ParabyzantineSpec, Data: ParabyzantineData<Spec>> ParabyzantineAgreementData<Spec>
	for Data
{
	fn parabyzantine_agreement_certificate_buffer(&self) -> &Spec::CertificateBuffer {
		self.parabyzantine_certificate_buffer()
	}
	fn parabyzantine_agreement_certificate_draft_buffer(&self) -> Spec::CertificateDraftBuffer {
		self.parabyzantine_certificate_draft_buffer()
	}
	fn parabyzantine_agreement_agreement_buffer(&self) -> &Spec::AgreementBuffer {
		self.parabyzantine_agreement_buffer()
	}
	fn parabyzantine_agreement_agreement_draft_buffer(&self) -> Spec::AgreementDraftBuffer {
		self.parabyzantine_agreement_draft_buffer()
	}
}
