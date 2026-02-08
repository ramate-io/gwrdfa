use crate::parabyzantine::agreement::{ParabyzantineAgreementSpec, ParabyzantineAgreementWorld};
use crate::parabyzantine::world::{ParabyzantineWorld, ParabyzantineWorldSpec};

/// Blanket implementation for the agreement spec.
///
/// Downcasting the world to an agreement world.
impl<Spec: ParabyzantineWorldSpec> ParabyzantineAgreementSpec for Spec {
	type CertificateEntity = Spec::CertificateEntity;
	type CertificateBuffer = Spec::CertificateBuffer;
	type CertificateDraftBuffer = Spec::CertificateDraftBuffer;
	type AgreementEntity = Spec::AgreementEntity;
	type AgreementBuffer = Spec::AgreementBuffer;
	type AgreementDraftBuffer = Spec::AgreementDraftBuffer;
}

impl<Spec: ParabyzantineWorldSpec, World: ParabyzantineWorld<Spec>>
	ParabyzantineAgreementWorld<Spec> for World
{
	fn certificate_buffer(&self) -> &Spec::CertificateBuffer {
		ParabyzantineWorld::certificate_buffer(self)
	}

	fn certificate_draft_buffer(&self) -> Spec::CertificateDraftBuffer {
		ParabyzantineWorld::certificate_draft_buffer(self)
	}

	fn agreement_buffer(&self) -> &Spec::AgreementBuffer {
		ParabyzantineWorld::agreement_buffer(self)
	}

	fn agreement_draft_buffer(&self) -> Spec::AgreementDraftBuffer {
		ParabyzantineWorld::agreement_draft_buffer(self)
	}
}
