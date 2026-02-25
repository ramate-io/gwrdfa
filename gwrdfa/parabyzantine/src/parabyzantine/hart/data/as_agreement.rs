use crate::agreement::{
	AgreementWorld, ParabyzantineAgreementData, ParabyzantineAgreementDataSpec,
};
use crate::hart::{ParabyzantineData, ParabyzantineDataSpec, ParabyzantineWorld};

/// Blanket implementation for the agreement spec.
///
/// Downcasting the world to an agreement world.
///
/// Note that because of blanket implementations on the Data,
/// we don't also have blanket implementations here.
impl<Spec: ParabyzantineDataSpec> ParabyzantineAgreementDataSpec for Spec {
	type CertificateEntity = Spec::CertificateEntity;
	type CertificateBuffer = Spec::CertificateBuffer;
	type AgreementEntity = Spec::AgreementEntity;
	type AgreementBuffer = Spec::AgreementBuffer;
}

/// Blanket implementation for the agreement data.
impl<Spec: ParabyzantineDataSpec, Data: ParabyzantineData<Spec>> ParabyzantineAgreementData<Spec>
	for Data
where
	// Spec needs to be static to support the 'a lifetime in the AgreementWorld.
	Spec: 'static,
{
	fn parabyzantine_agreement_world<'a>(&'a mut self) -> AgreementWorld<'a, Spec> {
		let ParabyzantineWorld { certificate_facts, agreement_facts, .. } =
			self.parabyzantine_world();

		AgreementWorld { certificate_facts, agreement_facts }
	}
}
