pub mod as_agreement;
pub mod as_message_in;
pub mod as_message_out;
pub mod as_task;

use crate::buffer::{facts::Facts, inferences::Inferences, Bufferlike, DraftBufferlike};
use crate::{NoOp, NoOpData};

/// The [Hart] marker is used to indicate an act on the entirety of the parabyzantine system.
///
/// You'll note that the canonical naming for systems on the [Hart] is simply to omit
/// the term "Hart". Hennce it is [ParabyzantineData] rather than [ParabyzantineHartData].
#[derive(Debug, Clone, Copy)]
pub struct Hart;

/// A [ParabyzantineDataSpec] is a specification for the parabyzantine protocol.
pub trait ParabyzantineDataSpec: Sized {
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

	/// The entity type for the message.
	type MessageEntity: Sized;
	/// The buffer type for the message.
	type MessageBuffer: Bufferlike<Self::MessageEntity>;
	/// The draft buffer type for the message.
	type MessageDraftBuffer: DraftBufferlike<Self::MessageEntity, Self::MessageBuffer>;
}

pub trait ParabyzantineData<Spec: ParabyzantineDataSpec>: Sized {
	/// The buffer for the certificate.
	fn parabyzantine_certificate_buffer(&self) -> &Spec::CertificateBuffer;
	/// The draft buffer for the certificate.
	fn parabyzantine_certificate_buffer_mut(&mut self) -> &mut Spec::CertificateBuffer;
	/// The draft buffer for the certificate.
	fn parabyzantine_certificate_draft_buffer(&self) -> Spec::CertificateDraftBuffer;
	/// The buffer for the agreement.
	fn parabyzantine_agreement_buffer(&self) -> &Spec::AgreementBuffer;
	/// The draft buffer for the agreement.
	fn parabyzantine_agreement_buffer_mut(&mut self) -> &mut Spec::AgreementBuffer;
	/// The draft buffer for the agreement.
	fn parabyzantine_agreement_draft_buffer(&self) -> Spec::AgreementDraftBuffer;

	/// The buffer for the transaction.
	fn parabyzantine_transaction_buffer(&self) -> &Spec::TransactionBuffer;

	/// The draft buffer for the transaction.
	fn parabyzantine_transaction_buffer_mut(&mut self) -> &mut Spec::TransactionBuffer;

	/// The draft buffer for the transaction.
	fn parabyzantine_transaction_draft_buffer(&self) -> Spec::TransactionDraftBuffer;

	/// The buffer for the task.
	fn parabyzantine_task_buffer(&self) -> &Spec::TaskBuffer;

	/// The draft buffer for the task.
	fn parabyzantine_task_buffer_mut(&mut self) -> &mut Spec::TaskBuffer;

	/// The draft buffer for the task.
	fn parabyzantine_task_draft_buffer(&self) -> Spec::TaskDraftBuffer;

	/// The buffer for the message.
	fn parabyzantine_message_buffer(&self) -> &Spec::MessageBuffer;

	/// The draft buffer for the message.
	fn parabyzantine_message_buffer_mut(&mut self) -> &mut Spec::MessageBuffer;

	/// The draft buffer for the message.
	fn parabyzantine_message_draft_buffer(&self) -> Spec::MessageDraftBuffer;

	/// The world of the parabyzantine.
	fn parabyzantine_world<'a>(&'a self) -> ParabyzantineWorld<'a, Spec> {
		ParabyzantineWorld {
			certificate_facts: self.parabyzantine_certificate_buffer().into(),
			certificate_inferences: self.parabyzantine_certificate_draft_buffer().into(),
			agreement_facts: self.parabyzantine_agreement_buffer().into(),
			agreement_inferences: self.parabyzantine_agreement_draft_buffer().into(),
			transaction_facts: self.parabyzantine_transaction_buffer().into(),
			transaction_inferences: self.parabyzantine_transaction_draft_buffer().into(),
			task_facts: self.parabyzantine_task_buffer().into(),
			task_inferences: self.parabyzantine_task_draft_buffer().into(),
			message_facts: self.parabyzantine_message_buffer().into(),
			message_inferences: self.parabyzantine_message_draft_buffer().into(),
		}
	}
}

pub struct ParabyzantineWorld<'a, Spec: ParabyzantineDataSpec> {
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

	/// The facts for the message.
	pub message_facts: Facts<'a, Spec::MessageEntity, Spec::MessageBuffer>,
	/// The inferences for the message.
	pub message_inferences:
		Inferences<Spec::MessageEntity, Spec::MessageBuffer, Spec::MessageDraftBuffer>,
}

/// A [ParabyzantineHart] trait describes operations on the parabyzantine hart.
pub trait ParabyzantineHart: Sized {
	type Binding: ParabyzantineDataBinding;

	/// Compute the parabyzantine hart.
	fn update_parabyzantine_hart(
		&mut self,
		data: &mut ParabyzantineWorld<<Self::Binding as ParabyzantineDataBinding>::Spec>,
	);
}

/// A [ParabyzantineDataBinding] is a binding for the [Parabyzantine] protocol.
///
/// It binds between the [ParabyzantineDataSpec] and the [ParabyzantineData].
pub trait ParabyzantineDataBinding {
	type Spec: ParabyzantineDataSpec;
	type Data: ParabyzantineData<Self::Spec>;
}

/// A [ParabyzantineDataSpec] for the [NoOp] struct.
impl ParabyzantineDataSpec for NoOp {
	type CertificateEntity = NoOp;
	type CertificateBuffer = NoOp;
	type CertificateDraftBuffer = NoOp;
	type AgreementEntity = NoOp;
	type AgreementBuffer = NoOp;
	type AgreementDraftBuffer = NoOp;
	type TransactionEntity = NoOp;
	type TransactionBuffer = NoOp;
	type TransactionDraftBuffer = NoOp;
	type TaskEntity = NoOp;
	type TaskBuffer = NoOp;
	type TaskDraftBuffer = NoOp;
	type MessageEntity = NoOp;
	type MessageBuffer = NoOp;
	type MessageDraftBuffer = NoOp;
}

/// A [ParabyzantineData] for the [NoOpData] struct.
impl ParabyzantineData<NoOp> for NoOpData {
	fn parabyzantine_certificate_buffer(&self) -> &NoOp {
		&self.no_op
	}
	fn parabyzantine_certificate_buffer_mut(&mut self) -> &mut NoOp {
		&mut self.no_op
	}
	fn parabyzantine_certificate_draft_buffer(&self) -> NoOp {
		NoOp
	}

	fn parabyzantine_agreement_buffer(&self) -> &NoOp {
		&self.no_op
	}
	fn parabyzantine_agreement_buffer_mut(&mut self) -> &mut NoOp {
		&mut self.no_op
	}
	fn parabyzantine_agreement_draft_buffer(&self) -> NoOp {
		NoOp
	}

	fn parabyzantine_transaction_buffer(&self) -> &NoOp {
		&self.no_op
	}
	fn parabyzantine_transaction_draft_buffer(&self) -> NoOp {
		NoOp
	}

	fn parabyzantine_transaction_buffer_mut(&mut self) -> &mut NoOp {
		&mut self.no_op
	}

	fn parabyzantine_task_buffer(&self) -> &NoOp {
		&self.no_op
	}
	fn parabyzantine_task_draft_buffer(&self) -> NoOp {
		NoOp
	}

	fn parabyzantine_task_buffer_mut(&mut self) -> &mut NoOp {
		&mut self.no_op
	}

	fn parabyzantine_message_buffer(&self) -> &NoOp {
		&self.no_op
	}
	fn parabyzantine_message_draft_buffer(&self) -> NoOp {
		NoOp
	}

	fn parabyzantine_message_buffer_mut(&mut self) -> &mut NoOp {
		&mut self.no_op
	}
}

impl ParabyzantineDataBinding for NoOp {
	type Spec = NoOp;
	type Data = NoOpData;
}
