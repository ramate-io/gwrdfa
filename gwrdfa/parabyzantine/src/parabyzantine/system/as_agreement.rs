use crate::parabyzantine::agreement::{
	system::ParabyzantineAgreementSystem, ParabyzantineAgreementSpec,
};
use crate::parabyzantine::system::{ParabyzantineSystem, ParabyzantineSystemSpec};

/// Blanket implementation for the agreement spec.
///
/// Downcasting the world to an agreement world.
impl<Spec: ParabyzantineSystemSpec> ParabyzantineAgreementSpec for Spec {
	type CertificateEntity = Spec::CertificateEntity;
	type CertificateBuffer = Spec::CertificateBuffer;
	type CertificateDraftBuffer = Spec::CertificateDraftBuffer;
	type AgreementEntity = Spec::AgreementEntity;
	type AgreementBuffer = Spec::AgreementBuffer;
	type AgreementDraftBuffer = Spec::AgreementDraftBuffer;
}

impl<Spec: ParabyzantineSystemSpec, World: ParabyzantineSystem<Spec>>
	ParabyzantineAgreementSystem<Spec> for World
{
	fn certificate_buffer(&self) -> &Spec::CertificateBuffer {
		ParabyzantineSystem::certificate_buffer(self)
	}

	fn certificate_draft_buffer(&self) -> Spec::CertificateDraftBuffer {
		ParabyzantineSystem::certificate_draft_buffer(self)
	}

	fn agreement_buffer(&self) -> &Spec::AgreementBuffer {
		ParabyzantineSystem::agreement_buffer(self)
	}

	fn agreement_draft_buffer(&self) -> Spec::AgreementDraftBuffer {
		ParabyzantineSystem::agreement_draft_buffer(self)
	}
}
