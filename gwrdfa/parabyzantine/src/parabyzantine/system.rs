pub mod as_agreement;
//pub mod as_broadcast_in;
//pub mod as_broadcast_out;
//pub mod as_task;

use crate::buffer::{facts::Facts, inferences::Inferences, Bufferlike, DraftBufferlike};
use crate::{Container, Factory, Member, Product, View};

pub trait ParabyzantineSpec<System: ParabyzantineSystem<Self>>: Sized {
	/// The entity type for the certificate.
	type CertificateEntity: Sized;
	/// The buffer type for the certificate.
	type CertificateBuffer: Bufferlike<Self::CertificateEntity> + Member<System>;
	/// The draft buffer type for the certificate.
	type CertificateDraftBuffer: DraftBufferlike<Self::CertificateEntity, Self::CertificateBuffer>
		+ Product<System>;

	/// The entity type for the agreement.
	type AgreementEntity: Sized;
	/// The buffer type for the agreement.
	type AgreementBuffer: Bufferlike<Self::AgreementEntity> + Member<System>;
	/// The draft buffer type for the agreement.
	type AgreementDraftBuffer: DraftBufferlike<Self::AgreementEntity, Self::AgreementBuffer>
		+ Product<System>;

	/// The entity type for the transaction.
	type TransactionEntity: Sized;
	/// The buffer type for the transaction.
	type TransactionBuffer: Bufferlike<Self::TransactionEntity> + Member<System>;
	/// The draft buffer type for the transaction.
	type TransactionDraftBuffer: DraftBufferlike<Self::TransactionEntity, Self::TransactionBuffer>
		+ Product<System>;

	/// The entity type for the task.
	type TaskEntity: Sized;
	/// The buffer type for the task.
	type TaskBuffer: Bufferlike<Self::TaskEntity> + Member<System>;
	/// The draft buffer type for the task.
	type TaskDraftBuffer: DraftBufferlike<Self::TaskEntity, Self::TaskBuffer> + Product<System>;

	/// The entity type for the broadcast.
	type BroadcastEntity: Sized;
	/// The buffer type for the broadcast.
	type BroadcastBuffer: Bufferlike<Self::BroadcastEntity> + Member<System>;
	/// The draft buffer type for the broadcast.
	type BroadcastDraftBuffer: DraftBufferlike<Self::BroadcastEntity, Self::BroadcastBuffer>
		+ Product<System>;
}

pub trait ParabyzantineSystem<Spec: ParabyzantineSpec<Self>>: Sized {
	fn parabyzantine_world(&self) -> ParabyzantineWorld<Spec, Self> {
		ParabyzantineWorld {
			certificate_facts: self.member::<Spec::CertificateBuffer>().into(),
			certificate_inferences: self.produce::<Spec::CertificateDraftBuffer>().into(),
			agreement_facts: self.member::<Spec::AgreementBuffer>().into(),
			agreement_inferences: self.produce::<Spec::AgreementDraftBuffer>().into(),
			transaction_facts: self.member::<Spec::TransactionBuffer>().into(),
			transaction_inferences: self.produce::<Spec::TransactionDraftBuffer>().into(),
			task_facts: self.member::<Spec::TaskBuffer>().into(),
			task_inferences: self.produce::<Spec::TaskDraftBuffer>().into(),
			broadcast_facts: self.member::<Spec::BroadcastBuffer>().into(),
			broadcast_inferences: self.produce::<Spec::BroadcastDraftBuffer>().into(),
		}
	}
}

pub struct ParabyzantineWorld<
	'a,
	Spec: ParabyzantineSpec<System>,
	System: ParabyzantineSystem<Spec>,
> {
	/// The facts for the certificate.
	pub certificate_facts: Facts<'a, Spec::CertificateEntity, Spec::CertificateBuffer>,
	/// The inferences for the certificate.
	pub certificate_inferences:
		Inferences<Spec::CertificateEntity, Spec::CertificateBuffer, Spec::CertificateDraftBuffer>,

	/// The facts for the agreement.
	pub agreement_facts: Facts<'a, Spec::AgreementEntity, Spec::AgreementBuffer>,
	/// The inferences for the agreement.
	pub agreement_inferences:
		Inferences<Spec::AgreementEntity, Spec::AgreementBuffer, Spec::AgreementDraftBuffer>,

	/// The facts for the transaction.
	pub transaction_facts: Facts<'a, Spec::TransactionEntity, Spec::TransactionBuffer>,
	/// The inferences for the transaction.
	pub transaction_inferences:
		Inferences<Spec::TransactionEntity, Spec::TransactionBuffer, Spec::TransactionDraftBuffer>,

	/// The facts for the task.
	pub task_facts: Facts<'a, Spec::TaskEntity, Spec::TaskBuffer>,
	/// The inferences for the task.
	pub task_inferences: Inferences<Spec::TaskEntity, Spec::TaskBuffer, Spec::TaskDraftBuffer>,

	/// The facts for the broadcast.
	pub broadcast_facts: Facts<'a, Spec::BroadcastEntity, Spec::BroadcastBuffer>,
	/// The inferences for the broadcast.
	pub broadcast_inferences:
		Inferences<Spec::BroadcastEntity, Spec::BroadcastBuffer, Spec::BroadcastDraftBuffer>,
}

/// View the world of a parabyzantine system.
///
/// This is implemented for ergonomics so that the user can write in the same style if they so choose.
impl<'a, Spec: ParabyzantineSpec<System>, System: ParabyzantineSystem<Spec>> View<'a, System>
	for ParabyzantineWorld<'a, Spec, System>
{
	fn view(from: &'a System) -> Self {
		from.parabyzantine_world()
	}
}
