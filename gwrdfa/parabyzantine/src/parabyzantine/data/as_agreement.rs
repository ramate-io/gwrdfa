use crate::parabyzantine::agreement::{ParabyzantineAgreementData, ParabyzantineAgreementSpec};
use crate::parabyzantine::data::{ParabyzantineData, ParabyzantineSpec};

/// Blanket implementation for the agreement spec.
///
/// Downcasting the world to an agreement world.
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

/// Blanket implementation for the agreement Data.
impl<Spec: ParabyzantineSpec<Data>, Data: ParabyzantineData<Spec>> ParabyzantineAgreementData<Spec>
	for Data
{
}
