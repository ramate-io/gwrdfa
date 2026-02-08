use crate::buffer::{facts::Facts, inferences::Inferences, Bufferlike, DraftBufferlike};
use crate::{Container, Factory, Member, Product, View};

/// Specifies the entities and buffers for a parabyzantine broadcast in Data.
///
/// A Parabyzantine broadcast in Data is concerned with deriving transactions and certificates from broadcasts.
pub trait ParabyzantineBroadcastInSpec<Data: ParabyzantineBroadcastInData<Self>>: Sized {
	/// The entity type for the broadcast.
	type BroadcastEntity: Sized;
	/// The buffer type for the broadcast.
	type BroadcastBuffer: Bufferlike<Self::BroadcastEntity> + Member<Data>;
	/// The draft buffer type for the broadcast.
	type BroadcastDraftBuffer: DraftBufferlike<Self::BroadcastEntity, Self::BroadcastBuffer>
		+ Product<Data>;

	/// The entity type for the transaction.
	type TransactionEntity: Sized;
	/// The buffer type for the transaction.
	type TransactionBuffer: Bufferlike<Self::TransactionEntity> + Member<Data>;
	/// The draft buffer type for the transaction.
	type TransactionDraftBuffer: DraftBufferlike<Self::TransactionEntity, Self::TransactionBuffer>
		+ Product<Data>;

	/// The entity type for the certificate.
	type CertificateEntity: Sized;
	/// The buffer type for the certificate.
	type CertificateBuffer: Bufferlike<Self::CertificateEntity> + Member<Data>;
	/// The draft buffer type for the certificate.
	type CertificateDraftBuffer: DraftBufferlike<Self::CertificateEntity, Self::CertificateBuffer>
		+ Product<Data>;
}

pub trait ParabyzantineBroadcastInData<Spec: ParabyzantineBroadcastInSpec<Self>>: Sized {
	fn parabyzantine_broadcast_in_world(&self) -> BroadcastInWorld<Spec, Self>;
}

/// Blanket implementation for the broadcast in Data when members are available.
impl<Spec: ParabyzantineBroadcastInSpec<Data>, Data> ParabyzantineBroadcastInData<Spec> for Data
where
	Data: Sized,
	Spec::BroadcastBuffer: Member<Data>,
	Spec::BroadcastDraftBuffer: Product<Data>,
	Spec::TransactionBuffer: Member<Data>,
	Spec::TransactionDraftBuffer: Product<Data>,
	Spec::CertificateBuffer: Member<Data>,
	Spec::CertificateDraftBuffer: Product<Data>,
{
	fn parabyzantine_broadcast_in_world(&self) -> BroadcastInWorld<Spec, Self> {
		BroadcastInWorld {
			broadcast_facts: self.member::<Spec::BroadcastBuffer>().into(),
			broadcast_inferences: self.produce::<Spec::BroadcastDraftBuffer>().into(),
			transaction_facts: self.member::<Spec::TransactionBuffer>().into(),
			transaction_inferences: self.produce::<Spec::TransactionDraftBuffer>().into(),
			certificate_facts: self.member::<Spec::CertificateBuffer>().into(),
			certificate_inferences: self.produce::<Spec::CertificateDraftBuffer>().into(),
		}
	}
}

/// The world of the broadcast in step of a parabyzantine broadcast in Data.
pub struct BroadcastInWorld<
	'a,
	Spec: ParabyzantineBroadcastInSpec<Data>,
	Data: ParabyzantineBroadcastInData<Spec>,
> {
	pub broadcast_facts: Facts<'a, Spec::BroadcastEntity, Spec::BroadcastBuffer>,
	pub broadcast_inferences:
		Inferences<Spec::BroadcastEntity, Spec::BroadcastBuffer, Spec::BroadcastDraftBuffer>,
	pub transaction_facts: Facts<'a, Spec::TransactionEntity, Spec::TransactionBuffer>,
	pub transaction_inferences:
		Inferences<Spec::TransactionEntity, Spec::TransactionBuffer, Spec::TransactionDraftBuffer>,
	pub certificate_facts: Facts<'a, Spec::CertificateEntity, Spec::CertificateBuffer>,
	pub certificate_inferences:
		Inferences<Spec::CertificateEntity, Spec::CertificateBuffer, Spec::CertificateDraftBuffer>,
}

/// View the world of a parabyzantine broadcast in Data.
///
/// This is implemented for ergonomics so that the user can write in the same style if they so choose.
impl<'a, Spec: ParabyzantineBroadcastInSpec<Data>, Data: ParabyzantineBroadcastInData<Spec>>
	View<'a, Data> for BroadcastInWorld<'a, Spec, Data>
{
	fn view(from: &'a Data) -> Self {
		from.parabyzantine_broadcast_in_world()
	}
}
