use super::{world::AgreementWorld, ParabyzantineAgreementSpec};

pub trait ParabyzantineAgreementSystem<Spec: ParabyzantineAgreementSpec> {
	/// Gets the certificate buffer.
	fn certificate_buffer(&self) -> &Spec::CertificateBuffer;

	/// Gets the certificate draft buffer.
	fn certificate_draft_buffer(&self) -> Spec::CertificateDraftBuffer;

	/// Gets the agreement buffer.
	fn agreement_buffer(&self) -> &Spec::AgreementBuffer;

	/// Gets the agreement draft buffer.
	fn agreement_draft_buffer(&self) -> Spec::AgreementDraftBuffer;

	/// Gets the agreement world.
	fn agreement_world(&self) -> AgreementWorld<Spec> {
		AgreementWorld {
			certificate_facts: self.certificate_buffer().into(),
			certificate_inferences: self.certificate_draft_buffer().into(),
			agreement_facts: self.agreement_buffer().into(),
			agreement_inferences: self.agreement_draft_buffer().into(),
		}
	}
}
