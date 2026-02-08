use crate::buffer::{facts::Facts, inferences::Inferences, Bufferlike, DraftBufferlike};

/// Specifies the entities and buffers for a parabyzantine agreement system.
///
/// A Parabyzantine agreement system is concerned with deriving agreements from certificates.
pub trait ParabyzantineAgreementSpec {
	type CertificateEntity: Sized;
	type CertificateBuffer: Bufferlike<Self::CertificateEntity>;
	type CertificateDraftBuffer: DraftBufferlike<Self::CertificateEntity, Self::CertificateBuffer>;
	type AgreementEntity: Sized;
	type AgreementBuffer: Bufferlike<Self::AgreementEntity>;
	type AgreementDraftBuffer: DraftBufferlike<Self::AgreementEntity, Self::AgreementBuffer>;
}

pub trait ParabyzantineAgreementWorld<Spec: ParabyzantineAgreementSpec> {
	/// Gets the certificate buffer.
	fn certificate_buffer(&self) -> &Spec::CertificateBuffer;

	/// Gets the certificate facts.
	fn certificate_facts(&self) -> Facts<Spec::CertificateEntity, Spec::CertificateBuffer> {
		Facts::new(self.certificate_buffer())
	}

	/// Gets the certificate draft buffer.
	fn certificate_draft_buffer(&self) -> Spec::CertificateDraftBuffer;

	/// Gets the certificate inferences.
	fn certificate_inferences(
		&self,
	) -> Inferences<Spec::CertificateEntity, Spec::CertificateBuffer, Spec::CertificateDraftBuffer>
	{
		Inferences::new(self.certificate_draft_buffer())
	}

	/// Gets the agreement buffer.
	fn agreement_buffer(&self) -> &Spec::AgreementBuffer;

	/// Gets the agreement facts.
	fn agreement_facts(&self) -> Facts<Spec::AgreementEntity, Spec::AgreementBuffer> {
		Facts::new(self.agreement_buffer())
	}

	/// Gets the agreement draft buffer.
	fn agreement_draft_buffer(&self) -> Spec::AgreementDraftBuffer;

	/// Gets the agreement inferences.
	fn agreement_inferences(
		&self,
	) -> Inferences<Spec::AgreementEntity, Spec::AgreementBuffer, Spec::AgreementDraftBuffer> {
		Inferences::new(self.agreement_draft_buffer())
	}
}

/// Parabyzantine agreement systems are concerned with deriving agreements from certificates.
pub trait ParabyzantineAgreement<
	Spec: ParabyzantineAgreementSpec,
	AgreementWorld: ParabyzantineAgreementWorld<Spec>,
>
{
	fn compute_parabyzantine_agreement(&mut self, agreement_world: &mut AgreementWorld);
}
