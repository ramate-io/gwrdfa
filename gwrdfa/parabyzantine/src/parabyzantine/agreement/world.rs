use super::ParabyzantineAgreementSpec;
use crate::buffer::{facts::Facts, inferences::Inferences};

/// The world of the agreement step of a parabyzantine agreement system.
pub struct AgreementWorld<'a, Spec: ParabyzantineAgreementSpec> {
	pub certificate_facts: Facts<'a, Spec::CertificateEntity, Spec::CertificateBuffer>,
	pub certificate_inferences:
		Inferences<Spec::CertificateEntity, Spec::CertificateBuffer, Spec::CertificateDraftBuffer>,
	pub agreement_facts: Facts<'a, Spec::AgreementEntity, Spec::AgreementBuffer>,
	pub agreement_inferences:
		Inferences<Spec::AgreementEntity, Spec::AgreementBuffer, Spec::AgreementDraftBuffer>,
}
