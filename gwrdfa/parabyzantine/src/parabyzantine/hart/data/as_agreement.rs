use crate::agreement::ParabyzantineAgreementData;
use crate::hart::ParabyzantineData;

/// Blanket implementation for the agreement data.
impl<Data: ParabyzantineData> ParabyzantineAgreementData for Data {
	type CertificateEntity = Data::CertificateEntity;
	type CertificateBuffer = Data::CertificateBuffer;
	type CertificateDraftBuffer = Data::CertificateDraftBuffer;
	type AgreementEntity = Data::AgreementEntity;
	type AgreementBuffer = Data::AgreementBuffer;
	type AgreementDraftBuffer = Data::AgreementDraftBuffer;

	fn parabyzantine_agreement_certificate_buffer(&self) -> &Data::CertificateBuffer {
		self.parabyzantine_certificate_buffer()
	}
	fn parabyzantine_agreement_certificate_buffer_mut(&mut self) -> &mut Data::CertificateBuffer {
		self.parabyzantine_certificate_buffer_mut()
	}
	fn parabyzantine_agreement_certificate_draft_buffer(&self) -> Data::CertificateDraftBuffer {
		self.parabyzantine_certificate_draft_buffer()
	}
	fn parabyzantine_agreement_agreement_buffer(&self) -> &Data::AgreementBuffer {
		self.parabyzantine_agreement_buffer()
	}
	fn parabyzantine_agreement_agreement_buffer_mut(&mut self) -> &mut Data::AgreementBuffer {
		self.parabyzantine_agreement_buffer_mut()
	}
	fn parabyzantine_agreement_agreement_draft_buffer(&self) -> Data::AgreementDraftBuffer {
		self.parabyzantine_agreement_draft_buffer()
	}
}
