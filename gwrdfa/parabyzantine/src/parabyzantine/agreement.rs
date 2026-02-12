use crate::buffer::{facts::Facts, inferences::Inferences, Bufferlike, DraftBufferlike};

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
	fn parabyzantine_agreement_certificate_buffer_mut(&mut self) -> &mut Spec::CertificateBuffer;

	/// The draft buffer for the certificate.
	fn parabyzantine_agreement_certificate_draft_buffer(&self) -> Spec::CertificateDraftBuffer;
	/// The buffer for the agreement.
	fn parabyzantine_agreement_agreement_buffer(&self) -> &Spec::AgreementBuffer;

	/// The draft buffer for the agreement.
	fn parabyzantine_agreement_agreement_buffer_mut(&mut self) -> &mut Spec::AgreementBuffer;

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

	/// Commit the agreement world to the data.
	fn commit_parabyzantine_agreement(&mut self, agreement_inferences: AgreementInferences<Spec>) {
		self.parabyzantine_agreement_certificate_buffer_mut()
			.commit_inferences(agreement_inferences.certificate_inferences);
		self.parabyzantine_agreement_agreement_buffer_mut()
			.commit_inferences(agreement_inferences.agreement_inferences);
	}
}

/// A [ParabyzantineAgreementBinding] is a binding for the [ParabyzantineAgreement] protocol.
///
/// It binds between the [ParabyzantineAgreementSpec] and the [ParabyzantineAgreementData].
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

/// The inferences for the agreement step of a parabyzantine agreement Data.
pub struct AgreementInferences<Spec: ParabyzantineAgreementSpec> {
	pub certificate_inferences:
		Inferences<Spec::CertificateEntity, Spec::CertificateBuffer, Spec::CertificateDraftBuffer>,
	pub agreement_inferences:
		Inferences<Spec::AgreementEntity, Spec::AgreementBuffer, Spec::AgreementDraftBuffer>,
}

impl<'a, Spec: ParabyzantineAgreementSpec> From<AgreementWorld<'a, Spec>>
	for AgreementInferences<Spec>
{
	fn from(world: AgreementWorld<'a, Spec>) -> Self {
		AgreementInferences {
			certificate_inferences: world.certificate_inferences,
			agreement_inferences: world.agreement_inferences,
		}
	}
}

pub trait ParabyzantineAgreement<
	Spec: ParabyzantineAgreementSpec,
	Data: ParabyzantineAgreementData<Spec>,
>: Sized
{
	/// Prepare the parabyzantine agreement.
	///
	/// This is a good place to add setup steps for tha agreement.
	fn pre_parabyzantine_agreement<'a>(&mut self, data: &'a mut Data) -> AgreementWorld<'a, Spec> {
		data.parabyzantine_agreement_world()
	}

	/// Compute the parabyzantine agreement.
	fn update_parabyzantine_agreement(&mut self, agreement_world: &mut AgreementWorld<Spec>);

	/// Run after the parabyzantine agreement update.
	///
	/// By default, this is where we commit the draft buffer to the main buffer.
	fn post_parabyzantine_agreement(
		&mut self,
		data: &mut Data,
		agreement_inferences: AgreementInferences<Spec>,
	) {
		data.commit_parabyzantine_agreement(agreement_inferences);
	}

	/// Runs the full parabyzantine agreement phase: pre, update, post.
	///
	/// Generally speaking, you'll use composition APIs to overwrite this.
	/// For example, in cases where you want to extend atomicity beyond
	/// just the agreement phase, you can update the [ParabyzantineAgreement::pre_parabyzantine_agreement]
	/// to store the or merge the inferences in a continuation buffer that will persist to the next phase.
	/// Then you can force this behavior on lower level implementations by overwriting this API.
	fn run_parabyzantine_agreement(&mut self, data: &mut Data) {
		let mut agreement_world = self.pre_parabyzantine_agreement(data);
		self.update_parabyzantine_agreement(&mut agreement_world);
		let agreement_inferences = agreement_world.into();
		self.post_parabyzantine_agreement(data, agreement_inferences);
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Agreement;
