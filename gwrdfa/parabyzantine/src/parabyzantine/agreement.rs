use crate::buffer::{facts::Facts, inferences::Inferences, Bufferlike, DraftBufferlike};
use crate::{Container, Factory, Member, Product, View};

/// Specifies the entities and buffers for a parabyzantine agreement Data.
///
/// A Parabyzantine agreement Data is concerned with deriving agreements from certificates.
pub trait ParabyzantineAgreementSpec<Data: ParabyzantineAgreementData<Self>>: Sized {
	/// The entity type for the certificate.
	type CertificateEntity: Sized;
	/// The buffer type for the certificate.
	type CertificateBuffer: Bufferlike<Self::CertificateEntity> + Member<Data>;
	/// The draft buffer type for the certificate.
	type CertificateDraftBuffer: DraftBufferlike<Self::CertificateEntity, Self::CertificateBuffer>
		+ Product<Data>;

	/// The entity type for the agreement.
	type AgreementEntity: Sized;
	/// The buffer type for the agreement.
	type AgreementBuffer: Bufferlike<Self::AgreementEntity> + Member<Data>;
	/// The draft buffer type for the agreement.
	type AgreementDraftBuffer: DraftBufferlike<Self::AgreementEntity, Self::AgreementBuffer>
		+ Product<Data>;
}

pub trait ParabyzantineAgreementData<Spec: ParabyzantineAgreementSpec<Self>>: Sized {
	fn parabyzantine_agreement_world(&self) -> AgreementWorld<Spec, Self> {
		AgreementWorld {
			certificate_facts: self.member::<Spec::CertificateBuffer>().into(),
			certificate_inferences: self.produce::<Spec::CertificateDraftBuffer>().into(),
			agreement_facts: self.member::<Spec::AgreementBuffer>().into(),
			agreement_inferences: self.produce::<Spec::AgreementDraftBuffer>().into(),
		}
	}
}

/// The world of the agreement step of a parabyzantine agreement Data.
pub struct AgreementWorld<
	'a,
	Spec: ParabyzantineAgreementSpec<Data>,
	Data: ParabyzantineAgreementData<Spec>,
> {
	pub certificate_facts: Facts<'a, Spec::CertificateEntity, Spec::CertificateBuffer>,
	pub certificate_inferences:
		Inferences<Spec::CertificateEntity, Spec::CertificateBuffer, Spec::CertificateDraftBuffer>,
	pub agreement_facts: Facts<'a, Spec::AgreementEntity, Spec::AgreementBuffer>,
	pub agreement_inferences:
		Inferences<Spec::AgreementEntity, Spec::AgreementBuffer, Spec::AgreementDraftBuffer>,
}

/// View the world of a parabyzantine agreement Data.
///
/// This is implemented for ergonomics so that the user can write in the same style if they so choose.
impl<'a, Spec: ParabyzantineAgreementSpec<Data>, Data: ParabyzantineAgreementData<Spec>>
	View<'a, Data> for AgreementWorld<'a, Spec, Data>
{
	fn view(from: &'a Data) -> Self {
		from.parabyzantine_agreement_world()
	}
}
