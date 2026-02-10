pub mod as_agreement;
pub mod as_broadcast_in;
pub mod as_broadcast_out;
pub mod as_task;

use crate::buffer::{facts::Facts, inferences::Inferences, Bufferlike, DraftBufferlike};

pub trait ParabyzantineSpec: Sized {
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

	/// The entity type for the transaction.
	type TransactionEntity: Sized;
	/// The buffer type for the transaction.
	type TransactionBuffer: Bufferlike<Self::TransactionEntity>;
	/// The draft buffer type for the transaction.
	type TransactionDraftBuffer: DraftBufferlike<Self::TransactionEntity, Self::TransactionBuffer>;

	/// The entity type for the task.
	type TaskEntity: Sized;
	/// The buffer type for the task.
	type TaskBuffer: Bufferlike<Self::TaskEntity>;
	/// The draft buffer type for the task.
	type TaskDraftBuffer: DraftBufferlike<Self::TaskEntity, Self::TaskBuffer>;

	/// The entity type for the broadcast.
	type BroadcastEntity: Sized;
	/// The buffer type for the broadcast.
	type BroadcastBuffer: Bufferlike<Self::BroadcastEntity>;
	/// The draft buffer type for the broadcast.
	type BroadcastDraftBuffer: DraftBufferlike<Self::BroadcastEntity, Self::BroadcastBuffer>;
}

pub trait ParabyzantineData<Spec: ParabyzantineSpec>: Sized {
	/// The buffer for the certificate.
	fn parabyzantine_certificate_buffer(&self) -> &Spec::CertificateBuffer;
	/// The draft buffer for the certificate.
	fn parabyzantine_certificate_draft_buffer(&self) -> Spec::CertificateDraftBuffer;
	/// The buffer for the agreement.
	fn parabyzantine_agreement_buffer(&self) -> &Spec::AgreementBuffer;
	/// The draft buffer for the agreement.
	fn parabyzantine_agreement_draft_buffer(&self) -> Spec::AgreementDraftBuffer;

	/// The buffer for the transaction.
	fn parabyzantine_transaction_buffer(&self) -> &Spec::TransactionBuffer;
	/// The draft buffer for the transaction.
	fn parabyzantine_transaction_draft_buffer(&self) -> Spec::TransactionDraftBuffer;
	/// The buffer for the task.
	fn parabyzantine_task_buffer(&self) -> &Spec::TaskBuffer;
	/// The draft buffer for the task.
	fn parabyzantine_task_draft_buffer(&self) -> Spec::TaskDraftBuffer;

	/// The buffer for the broadcast.
	fn parabyzantine_broadcast_buffer(&self) -> &Spec::BroadcastBuffer;
	/// The draft buffer for the broadcast.
	fn parabyzantine_broadcast_draft_buffer(&self) -> Spec::BroadcastDraftBuffer;

	/// The world of the parabyzantine.
	fn parabyzantine_world(&self) -> ParabyzantineWorld<Spec> {
		ParabyzantineWorld {
			certificate_facts: self.parabyzantine_certificate_buffer().into(),
			certificate_inferences: self.parabyzantine_certificate_draft_buffer().into(),
			agreement_facts: self.parabyzantine_agreement_buffer().into(),
			agreement_inferences: self.parabyzantine_agreement_draft_buffer().into(),
			transaction_facts: self.parabyzantine_transaction_buffer().into(),
			transaction_inferences: self.parabyzantine_transaction_draft_buffer().into(),
			task_facts: self.parabyzantine_task_buffer().into(),
			task_inferences: self.parabyzantine_task_draft_buffer().into(),
			broadcast_facts: self.parabyzantine_broadcast_buffer().into(),
			broadcast_inferences: self.parabyzantine_broadcast_draft_buffer().into(),
		}
	}
}

pub struct ParabyzantineWorld<'a, Spec: ParabyzantineSpec> {
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
