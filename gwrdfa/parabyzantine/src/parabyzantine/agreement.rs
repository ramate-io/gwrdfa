pub mod system;
pub mod world;

use crate::buffer::{Bufferlike, DraftBufferlike};
use world::AgreementWorld;

/// Specifies the entities and buffers for a parabyzantine agreement system.
///
/// A Parabyzantine agreement system is concerned with deriving agreements from certificates.
pub trait ParabyzantineAgreementSpec {
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

/// Parabyzantine agreement systems are concerned with deriving agreements from certificates.
pub trait ParabyzantineAgreement<Spec: ParabyzantineAgreementSpec> {
	fn compute_parabyzantine_agreement(&mut self, agreement_world: &mut AgreementWorld<Spec>);
}
