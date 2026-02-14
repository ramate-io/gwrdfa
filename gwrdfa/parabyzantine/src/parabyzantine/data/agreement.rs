use crate::act::Act;
use crate::buffer::{facts::Facts, inferences::Inferences, Bufferlike, DraftBufferlike};
use crate::{NoOp, NoOpData};

#[derive(Debug, Clone, Copy)]
pub struct Agreement;

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

pub trait ParabyzantineAgreement: Sized {
	type Binding: ParabyzantineAgreementBinding;

	/// Gets the [AgreementWorld] for the parabyzantine agreement.
	fn parabyzantine_agreement_world<'a>(
		&mut self,
		data: &'a mut <Self::Binding as ParabyzantineAgreementBinding>::Data,
	) -> AgreementWorld<'a, <Self::Binding as ParabyzantineAgreementBinding>::Spec> {
		data.parabyzantine_agreement_world()
	}

	/// Compute the parabyzantine agreement.
	fn update_parabyzantine_agreement(
		&mut self,
		agreement_world: &mut AgreementWorld<
			<Self::Binding as ParabyzantineAgreementBinding>::Spec,
		>,
	);

	/// Commits the inferences for the parabyzantine agreement.
	fn commit_parabyzantine_agreement(
		&mut self,
		agreement_inferences: AgreementInferences<
			<Self::Binding as ParabyzantineAgreementBinding>::Spec,
		>,
		data: &mut <Self::Binding as ParabyzantineAgreementBinding>::Data,
	) {
		data.commit_parabyzantine_agreement(agreement_inferences);
	}
}

impl<
		Binding: ParabyzantineAgreementBinding,
		AgreementHandler: ParabyzantineAgreement<Binding = Binding>,
	> Act<Agreement, Binding::Data> for AgreementHandler
{
	fn act(&mut self, _action: Agreement, data: &mut Binding::Data) {
		let mut world = self.parabyzantine_agreement_world(data);
		self.update_parabyzantine_agreement(&mut world);
		self.commit_parabyzantine_agreement(world.into(), data);
	}
}

impl ParabyzantineAgreementBinding for NoOp {
	type Spec = NoOp;
	type Data = NoOpData;
}
