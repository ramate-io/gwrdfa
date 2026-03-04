use crate::act::Act;
use crate::buffer::{facts::Facts, inferences::Inferences, Bufferlike, DraftBufferlike};
use crate::{NoOp, NoOpData};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Agreement;

/// Specifies the entities and buffers for a parabyzantine agreement Data.
///
/// A Parabyzantine agreement Data is concerned with deriving agreements from certificates.
pub trait ParabyzantineAgreementData: Sized {
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
	/// The buffer for the certificate.
	fn parabyzantine_agreement_certificate_buffer(&self) -> &Self::CertificateBuffer;

	/// The draft buffer for the certificate.
	fn parabyzantine_agreement_certificate_buffer_mut(&mut self) -> &mut Self::CertificateBuffer;

	/// The draft buffer for the certificate.
	fn parabyzantine_agreement_certificate_draft_buffer(&self) -> Self::CertificateDraftBuffer;
	/// The buffer for the agreement.
	fn parabyzantine_agreement_agreement_buffer(&self) -> &Self::AgreementBuffer;

	/// The draft buffer for the agreement.
	fn parabyzantine_agreement_agreement_buffer_mut(&mut self) -> &mut Self::AgreementBuffer;

	/// The draft buffer for the agreement.
	fn parabyzantine_agreement_agreement_draft_buffer(&self) -> Self::AgreementDraftBuffer;

	/// The world of the agreement.
	fn parabyzantine_agreement_world<'a>(&'a self) -> AgreementWorld<'a, Self> {
		AgreementWorld {
			certificate_facts: self.parabyzantine_agreement_certificate_buffer().into(),
			certificate_inferences: self.parabyzantine_agreement_certificate_draft_buffer().into(),
			agreement_facts: self.parabyzantine_agreement_agreement_buffer().into(),
			agreement_inferences: self.parabyzantine_agreement_agreement_draft_buffer().into(),
		}
	}

	/// Commit the agreement world to the data.
	fn commit_parabyzantine_agreement(&mut self, agreement_inferences: AgreementInferences<Self>) {
		self.parabyzantine_agreement_certificate_buffer_mut()
			.commit_inferences(agreement_inferences.certificate_inferences);
		self.parabyzantine_agreement_agreement_buffer_mut()
			.commit_inferences(agreement_inferences.agreement_inferences);
	}
}

/// A [ParabyzantineAgreementDataBinding] is a binding for the [ParabyzantineAgreement] protocol.
///
/// It binds to [ParabyzantineAgreementData].
pub trait ParabyzantineAgreementDataBinding {
	type Data: ParabyzantineAgreementData;
}

/// The world of the agreement step of a parabyzantine agreement Data.
pub struct AgreementWorld<'a, Data: ParabyzantineAgreementData> {
	pub certificate_facts: Facts<'a, Data::CertificateEntity, Data::CertificateBuffer>,
	pub certificate_inferences:
		Inferences<Data::CertificateEntity, Data::CertificateBuffer, Data::CertificateDraftBuffer>,
	pub agreement_facts: Facts<'a, Data::AgreementEntity, Data::AgreementBuffer>,
	pub agreement_inferences:
		Inferences<Data::AgreementEntity, Data::AgreementBuffer, Data::AgreementDraftBuffer>,
}

/// The inferences for the agreement step of a parabyzantine agreement Data.
pub struct AgreementInferences<Data: ParabyzantineAgreementData> {
	pub certificate_inferences:
		Inferences<Data::CertificateEntity, Data::CertificateBuffer, Data::CertificateDraftBuffer>,
	pub agreement_inferences:
		Inferences<Data::AgreementEntity, Data::AgreementBuffer, Data::AgreementDraftBuffer>,
}

impl<'a, Data: ParabyzantineAgreementData> From<AgreementWorld<'a, Data>>
	for AgreementInferences<Data>
{
	fn from(world: AgreementWorld<'a, Data>) -> Self {
		AgreementInferences {
			certificate_inferences: world.certificate_inferences,
			agreement_inferences: world.agreement_inferences,
		}
	}
}

pub trait ParabyzantineAgreement: Sized {
	type Binding: ParabyzantineAgreementDataBinding;

	/// Gets the [AgreementWorld] for the parabyzantine agreement.
	fn parabyzantine_agreement_world<'a>(
		&mut self,
		data: &'a mut <Self::Binding as ParabyzantineAgreementDataBinding>::Data,
	) -> AgreementWorld<'a, <Self::Binding as ParabyzantineAgreementDataBinding>::Data> {
		data.parabyzantine_agreement_world()
	}

	/// Compute the parabyzantine agreement.
	fn update_parabyzantine_agreement(
		&mut self,
		agreement_world: &mut AgreementWorld<
			<Self::Binding as ParabyzantineAgreementDataBinding>::Data,
		>,
	);

	/// Commits the inferences for the parabyzantine agreement.
	fn commit_parabyzantine_agreement(
		&mut self,
		agreement_inferences: AgreementInferences<
			<Self::Binding as ParabyzantineAgreementDataBinding>::Data,
		>,
		data: &mut <Self::Binding as ParabyzantineAgreementDataBinding>::Data,
	) {
		data.commit_parabyzantine_agreement(agreement_inferences);
	}
}

impl<
		Binding: ParabyzantineAgreementDataBinding,
		AgreementHandler: ParabyzantineAgreement<Binding = Binding>,
	> Act<Agreement, Binding::Data> for AgreementHandler
{
	fn act(&mut self, _action: Agreement, data: &mut Binding::Data) {
		let mut world = self.parabyzantine_agreement_world(data);
		self.update_parabyzantine_agreement(&mut world);
		self.commit_parabyzantine_agreement(world.into(), data);
	}
}

impl ParabyzantineAgreementDataBinding for NoOp {
	type Data = NoOpData;
}
