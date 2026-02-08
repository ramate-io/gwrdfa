use crate::parabyzantine::agreement::ParabyzantineAgreementSpec;
use crate::parabyzantine::data::{ParabyzantineData, ParabyzantineSpec};

/// Blanket implementation for the agreement spec.
///
/// Downcasting the world to an agreement world.
///
/// Note that because of blanket implementations on the Data,
/// we don't also have blanket implementations here.
impl<Spec: ParabyzantineSpec<Data>, Data: ParabyzantineData<Spec>> ParabyzantineAgreementSpec<Data>
	for Spec
{
	type CertificateEntity = Spec::CertificateEntity;
	type CertificateBuffer = Spec::CertificateBuffer;
	type CertificateDraftBuffer = Spec::CertificateDraftBuffer;
	type AgreementEntity = Spec::AgreementEntity;
	type AgreementBuffer = Spec::AgreementBuffer;
	type AgreementDraftBuffer = Spec::AgreementDraftBuffer;
}
