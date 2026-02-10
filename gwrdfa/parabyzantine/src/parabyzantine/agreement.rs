use crate::buffer::{facts::Facts, inferences::Inferences, Bufferlike, DraftBufferlike};

/// The schedule for the prepare step of the parabyzantine agreement.
#[derive(Debug, Clone, Copy)]
pub struct PreParabyzantineAgreement;

/// The schedule for the compute step of the parabyzantine agreement.
#[derive(Debug, Clone, Copy)]
pub struct UpdateParabyzantineAgreement;

/// The schedule for the commit step of the parabyzantine agreement.
#[derive(Debug, Clone, Copy)]
pub struct PostParabyzantineAgreement;

/// Specifies the entities and buffers for a parabyzantine agreement Data.
///
/// A Parabyzantine agreement Data is concerned with deriving agreements from certificates.
pub trait ParabyzantineAgreementSpec: Sized {
	/// The entity type for the certificate.
	type CertificateEntity: Sized;
	/// The buffer type for the certificate.
	type CertificateBuffer: Bufferlike<Self::CertificateEntity>;
	/// The draft buffer type for the certificate.
	type CertificateDraftBuffer: DraftBufferlike<Self::CertificateEntity, Self::CertificateBuffer>;

	/// The entity type for the agreement.
	type AgreementEntity: Sized;
	/// The buffer type for the agreement.
	type AgreementBuffer: Bufferlike<Self::AgreementEntity>;
	/// The draft buffer type for the agreement.
	type AgreementDraftBuffer: DraftBufferlike<Self::AgreementEntity, Self::AgreementBuffer>;
}

pub trait ParabyzantineAgreementData<Spec: ParabyzantineAgreementSpec>: Sized {
	/// The buffer for the certificate.
	fn parabyzantine_agreement_certificate_buffer(&self) -> &Spec::CertificateBuffer;
	/// The draft buffer for the certificate.
	fn parabyzantine_agreement_certificate_draft_buffer(&self) -> Spec::CertificateDraftBuffer;
	/// The buffer for the agreement.
	fn parabyzantine_agreement_agreement_buffer(&self) -> &Spec::AgreementBuffer;
	/// The draft buffer for the agreement.
	fn parabyzantine_agreement_agreement_draft_buffer(&self) -> Spec::AgreementDraftBuffer;

	/// The world of the agreement.
	fn parabyzantine_agreement_world(&self) -> AgreementWorld<Spec> {
		AgreementWorld {
			certificate_facts: self.parabyzantine_agreement_certificate_buffer().into(),
			certificate_inferences: self.parabyzantine_agreement_certificate_draft_buffer().into(),
			agreement_facts: self.parabyzantine_agreement_agreement_buffer().into(),
			agreement_inferences: self.parabyzantine_agreement_agreement_draft_buffer().into(),
		}
	}
}

pub trait ParabyzantineAgreementBinding {
	type Spec: ParabyzantineAgreementSpec;
	type Data: ParabyzantineAgreementData<Self::Spec>;
}

/// The world of the agreement step of a parabyzantine agreement Data.
pub struct AgreementWorld<'a, Spec: ParabyzantineAgreementSpec> {
	pub certificate_facts: Facts<'a, Spec::CertificateEntity, Spec::CertificateBuffer>,
	pub certificate_inferences:
		Inferences<Spec::CertificateEntity, Spec::CertificateBuffer, Spec::CertificateDraftBuffer>,
	pub agreement_facts: Facts<'a, Spec::AgreementEntity, Spec::AgreementBuffer>,
	pub agreement_inferences:
		Inferences<Spec::AgreementEntity, Spec::AgreementBuffer, Spec::AgreementDraftBuffer>,
}

pub trait ParabyzantineAgreement<Spec: ParabyzantineAgreementSpec>: Sized {
	/// Compute the parabyzantine agreement.
	fn update_parabyzantine_agreement(&mut self, data: &mut AgreementWorld<Spec>);
}
