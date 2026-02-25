use crate::act::Act;
use crate::buffer::{facts::Facts, Bufferlike};
use crate::{NoOp, NoOpData};

#[derive(Debug, Clone, Copy)]
pub struct Agreement;

/// Specifies the entities and buffers for a parabyzantine agreement Data.
///
/// A Parabyzantine agreement Data is concerned with deriving agreements from certificates.
pub trait ParabyzantineAgreementDataSpec: Sized {
	/// The entity type for the certificate.
	type CertificateEntity: Sized;
	/// The buffer type for the certificate.
	type CertificateBuffer: Bufferlike<Self::CertificateEntity>;

	/// The entity type for the agreement.
	type AgreementEntity: Sized;
	/// The buffer type for the agreement.
	type AgreementBuffer: Bufferlike<Self::AgreementEntity>;
}

pub trait ParabyzantineAgreementData<Spec: ParabyzantineAgreementDataSpec>: Sized {
	/// The world of the agreement.
	fn parabyzantine_agreement_world<'a>(&'a mut self) -> AgreementWorld<'a, Spec>;
}

/// A [ParabyzantineAgreementDataBinding] is a binding for the [ParabyzantineAgreement] protocol.
///
/// It binds between the [ParabyzantineAgreementDataSpec] and the [ParabyzantineAgreementData].
pub trait ParabyzantineAgreementDataBinding {
	type Spec: ParabyzantineAgreementDataSpec;
	type Data: ParabyzantineAgreementData<Self::Spec>;
}

/// The world of the agreement step of a parabyzantine agreement Data.
pub struct AgreementWorld<'a, Spec: ParabyzantineAgreementDataSpec> {
	pub certificate_facts: Facts<'a, Spec::CertificateEntity, Spec::CertificateBuffer>,
	pub agreement_facts: Facts<'a, Spec::AgreementEntity, Spec::AgreementBuffer>,
}

pub trait ParabyzantineAgreement: Sized {
	type Binding: ParabyzantineAgreementDataBinding;

	/// Gets the [AgreementWorld] for the parabyzantine agreement.
	fn parabyzantine_agreement_world<'a>(
		&mut self,
		data: &'a mut <Self::Binding as ParabyzantineAgreementDataBinding>::Data,
	) -> AgreementWorld<'a, <Self::Binding as ParabyzantineAgreementDataBinding>::Spec> {
		data.parabyzantine_agreement_world()
	}

	/// Compute the parabyzantine agreement.
	fn update_parabyzantine_agreement(
		&mut self,
		agreement_world: &mut AgreementWorld<
			<Self::Binding as ParabyzantineAgreementDataBinding>::Spec,
		>,
	);
}

impl<
		Binding: ParabyzantineAgreementDataBinding,
		AgreementHandler: ParabyzantineAgreement<Binding = Binding>,
	> Act<Agreement, Binding::Data> for AgreementHandler
{
	fn act(&mut self, _action: Agreement, data: &mut Binding::Data) {
		let mut world = self.parabyzantine_agreement_world(data);
		self.update_parabyzantine_agreement(&mut world);
	}
}

impl ParabyzantineAgreementDataBinding for NoOp {
	type Spec = NoOp;
	type Data = NoOpData;
}
