use crate::parabyzantine::agreement::{ParabyzantineAgreementSpec, ParabyzantineAgreementSystem};
use crate::parabyzantine::system::{ParabyzantineSpec, ParabyzantineSystem};

/// Blanket implementation for the agreement spec.
///
/// Downcasting the world to an agreement world.
impl<Spec: ParabyzantineSpec<System>, System: ParabyzantineSystem<Spec>>
	ParabyzantineAgreementSpec<System> for Spec
{
	type CertificateEntity = Spec::CertificateEntity;
	type CertificateBuffer = Spec::CertificateBuffer;
	type CertificateDraftBuffer = Spec::CertificateDraftBuffer;
	type AgreementEntity = Spec::AgreementEntity;
	type AgreementBuffer = Spec::AgreementBuffer;
	type AgreementDraftBuffer = Spec::AgreementDraftBuffer;
}

/// Blanket implementation for the agreement system.
impl<Spec: ParabyzantineSpec<System>, System: ParabyzantineSystem<Spec>>
	ParabyzantineAgreementSystem<Spec> for System
{
}
