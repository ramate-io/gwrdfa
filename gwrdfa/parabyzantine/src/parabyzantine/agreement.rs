use crate::buffer::{facts::Facts, inferences::Inferences, Bufferlike, DraftBufferlike};
use crate::{Container, Factory, Member, Product, View};

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
pub trait ParabyzantineAgreementSpec<Binding: ParabyzantineAgreementBinding>: Sized {
	/// The entity type for the certificate.
	type CertificateEntity: Sized;
	/// The buffer type for the certificate.
	type CertificateBuffer: Bufferlike<Self::CertificateEntity> + Member<Binding::Data>;
	/// The draft buffer type for the certificate.
	type CertificateDraftBuffer: DraftBufferlike<Self::CertificateEntity, Self::CertificateBuffer>
		+ Product<Binding::Data>;

	/// The entity type for the agreement.
	type AgreementEntity: Sized;
	/// The buffer type for the agreement.
	type AgreementBuffer: Bufferlike<Self::AgreementEntity> + Member<Binding::Data>;
	/// The draft buffer type for the agreement.
	type AgreementDraftBuffer: DraftBufferlike<Self::AgreementEntity, Self::AgreementBuffer>
		+ Product<Binding::Data>;
}

pub trait ParabyzantineAgreementData<Binding: ParabyzantineAgreementBinding>: Sized {
	fn parabyzantine_agreement_world(&self) -> AgreementWorld<Binding>;
}

/// Represent a valid binding between a parabyzantine agreement spec and data.
///
/// In categorical language, if this diagram commutes, you can use the Data to satisfy the Spec,
/// and thus have a valid parabyzantine agreement.
pub trait ParabyzantineAgreementBinding: Sized {
	/// The spec for the parabyzantine agreement.
	type Spec: ParabyzantineAgreementSpec<Self>;

	/// The data for the parabyzantine agreement.
	type Data: ParabyzantineAgreementData<Self>;
}

/// Blanket implementation for the agreement Data when members and products are available.
///
/// Currently, we're forcing this pattern by requiring the Spec to bound these fields as members and products of the Data.
/// This is more of a developer awareness thing than a technical requirement.
/// It's easier to debug where you're coming up short on members and products
/// if it points to a particular field in the spec, as opposed to simply showing
/// a failed trait bound.
impl<'a, Binding: ParabyzantineAgreementBinding> ParabyzantineAgreementData<Binding>
	for Binding::Data
where
	<Binding::Spec as ParabyzantineAgreementSpec<Binding>>::CertificateBuffer:
		Member<Binding::Data>,
	<Binding::Spec as ParabyzantineAgreementSpec<Binding>>::CertificateDraftBuffer:
		Product<Binding::Data>,
	<Binding::Spec as ParabyzantineAgreementSpec<Binding>>::AgreementBuffer: Member<Binding::Data>,
	<Binding::Spec as ParabyzantineAgreementSpec<Binding>>::AgreementDraftBuffer:
		Product<Binding::Data>,
{
	fn parabyzantine_agreement_world(&self) -> AgreementWorld<Binding> {
		AgreementWorld {
			certificate_facts: self
				.member::<<Binding::Spec as ParabyzantineAgreementSpec<Binding>>::CertificateBuffer>(
				)
				.into(),
			certificate_inferences: self
				.produce::<<Binding::Spec as ParabyzantineAgreementSpec<Binding>>::CertificateDraftBuffer>(
				)
				.into(),
			agreement_facts: self
				.member::<<Binding::Spec as ParabyzantineAgreementSpec<Binding>>::AgreementBuffer>()
				.into(),
			agreement_inferences: self
				.produce::<<Binding::Spec as ParabyzantineAgreementSpec<Binding>>::AgreementDraftBuffer>(
				)
				.into(),
		}
	}
}

/// The world of the agreement step of a parabyzantine agreement Data.
pub struct AgreementWorld<'a, Binding: ParabyzantineAgreementBinding> {
	pub certificate_facts: Facts<
		'a,
		<Binding::Spec as ParabyzantineAgreementSpec<Binding>>::CertificateEntity,
		<Binding::Spec as ParabyzantineAgreementSpec<Binding>>::CertificateBuffer,
	>,
	pub certificate_inferences: Inferences<
		<Binding::Spec as ParabyzantineAgreementSpec<Binding>>::CertificateEntity,
		<Binding::Spec as ParabyzantineAgreementSpec<Binding>>::CertificateBuffer,
		<Binding::Spec as ParabyzantineAgreementSpec<Binding>>::CertificateDraftBuffer,
	>,
	pub agreement_facts: Facts<
		'a,
		<Binding::Spec as ParabyzantineAgreementSpec<Binding>>::AgreementEntity,
		<Binding::Spec as ParabyzantineAgreementSpec<Binding>>::AgreementBuffer,
	>,
	pub agreement_inferences: Inferences<
		<Binding::Spec as ParabyzantineAgreementSpec<Binding>>::AgreementEntity,
		<Binding::Spec as ParabyzantineAgreementSpec<Binding>>::AgreementBuffer,
		<Binding::Spec as ParabyzantineAgreementSpec<Binding>>::AgreementDraftBuffer,
	>,
}

/// View the world of a parabyzantine agreement Data.
///
/// This is implemented for ergonomics so that the user can write in the same style if they so choose.
impl<'a, Binding: ParabyzantineAgreementBinding> View<'a, Binding::Data>
	for AgreementWorld<'a, Binding>
{
	fn view(from: &'a Binding::Data) -> Self {
		from.parabyzantine_agreement_world()
	}
}

pub trait ParabyzantineAgreement<Binding: ParabyzantineAgreementBinding>: Sized {
	/// Compute the parabyzantine agreement.
	fn update_parabyzantine_agreement(&mut self, data: &mut AgreementWorld<Binding>);
}
