pub mod as_agreement;
pub mod as_message_in;
pub mod as_message_out;
pub mod as_task;

use crate::act::Act;
use crate::buffer::{facts::Facts, inferences::Inferences, Bufferlike, DraftBufferlike};
use crate::{NoOp, NoOpData};

/// The [Hart] marker is used to indicate an act on the entirety of the parabyzantine system.
///
/// You'll note that the canonical naming for systems on the [Hart] is simply to omit
/// the term "Hart". Hennce it is [ParabyzantineData] rather than [ParabyzantineHartData].
#[derive(Debug, Clone, Copy)]
pub struct Hart;

/// A [ParabyzantineData] describes the entities and buffers used by the parabyzantine protocol.
pub trait ParabyzantineData: Sized {
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
	/// The buffer for the certificate.
	fn parabyzantine_certificate_buffer(&self) -> &Self::CertificateBuffer;

	/// The draft buffer for the certificate.
	fn parabyzantine_certificate_buffer_mut(&mut self) -> &mut Self::CertificateBuffer;

	/// The draft buffer for the certificate.
	fn parabyzantine_certificate_draft_buffer(&self) -> Self::CertificateDraftBuffer;

	/// The buffer for the agreement.
	fn parabyzantine_agreement_buffer(&self) -> &Self::AgreementBuffer;

	/// The draft buffer for the agreement.
	fn parabyzantine_agreement_buffer_mut(&mut self) -> &mut Self::AgreementBuffer;

	/// The draft buffer for the agreement.
	fn parabyzantine_agreement_draft_buffer(&self) -> Self::AgreementDraftBuffer;

	/// The buffer for the transaction.
	fn parabyzantine_transaction_buffer(&self) -> &Self::TransactionBuffer;

	/// The draft buffer for the transaction.
	fn parabyzantine_transaction_buffer_mut(&mut self) -> &mut Self::TransactionBuffer;

	/// The draft buffer for the transaction.
	fn parabyzantine_transaction_draft_buffer(&self) -> Self::TransactionDraftBuffer;

	/// The buffer for the task.
	fn parabyzantine_task_buffer(&self) -> &Self::TaskBuffer;

	/// The draft buffer for the task.
	fn parabyzantine_task_buffer_mut(&mut self) -> &mut Self::TaskBuffer;

	/// The draft buffer for the task.
	fn parabyzantine_task_draft_buffer(&self) -> Self::TaskDraftBuffer;

	/// The buffer for the message.
	fn parabyzantine_message_buffer(&self) -> &Self::MessageBuffer;

	/// The draft buffer for the message.
	fn parabyzantine_message_buffer_mut(&mut self) -> &mut Self::MessageBuffer;

	/// The draft buffer for the message.
	fn parabyzantine_message_draft_buffer(&self) -> Self::MessageDraftBuffer;

	/// The world of the parabyzantine.
	fn parabyzantine_world<'a>(&'a self) -> ParabyzantineWorld<'a, Self> {
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

	/// Commits the inferences for the parabyzantine world.
	fn commit_parabyzantine_hart(&mut self, inferences: ParabyzantineHartInferences<Self>) {
		self.parabyzantine_certificate_buffer_mut()
			.commit_inferences(inferences.certificate_inferences);
		self.parabyzantine_agreement_buffer_mut()
			.commit_inferences(inferences.agreement_inferences);
		self.parabyzantine_transaction_buffer_mut()
			.commit_inferences(inferences.transaction_inferences);
		self.parabyzantine_task_buffer_mut()
			.commit_inferences(inferences.task_inferences);
		self.parabyzantine_message_buffer_mut()
			.commit_inferences(inferences.message_inferences);
	}
}

pub struct ParabyzantineWorld<'a, Data: ParabyzantineData> {
	/// The facts for the certificate.
	pub certificate_facts: Facts<'a, Data::CertificateEntity, Data::CertificateBuffer>,
	/// The inferences for the certificate.
	pub certificate_inferences:
		Inferences<Data::CertificateEntity, Data::CertificateBuffer, Data::CertificateDraftBuffer>,

	/// The facts for the agreement.
	pub agreement_facts: Facts<'a, Data::AgreementEntity, Data::AgreementBuffer>,
	/// The inferences for the agreement.
	pub agreement_inferences:
		Inferences<Data::AgreementEntity, Data::AgreementBuffer, Data::AgreementDraftBuffer>,

	/// The facts for the transaction.
	pub transaction_facts: Facts<'a, Data::TransactionEntity, Data::TransactionBuffer>,
	/// The inferences for the transaction.
	pub transaction_inferences:
		Inferences<Data::TransactionEntity, Data::TransactionBuffer, Data::TransactionDraftBuffer>,

	/// The facts for the task.
	pub task_facts: Facts<'a, Data::TaskEntity, Data::TaskBuffer>,
	/// The inferences for the task.
	pub task_inferences: Inferences<Data::TaskEntity, Data::TaskBuffer, Data::TaskDraftBuffer>,

	/// The facts for the message.
	pub message_facts: Facts<'a, Data::MessageEntity, Data::MessageBuffer>,
	/// The inferences for the message.
	pub message_inferences:
		Inferences<Data::MessageEntity, Data::MessageBuffer, Data::MessageDraftBuffer>,
}

pub struct ParabyzantineHartInferences<Data: ParabyzantineData> {
	pub certificate_inferences:
		Inferences<Data::CertificateEntity, Data::CertificateBuffer, Data::CertificateDraftBuffer>,
	pub agreement_inferences:
		Inferences<Data::AgreementEntity, Data::AgreementBuffer, Data::AgreementDraftBuffer>,
	pub transaction_inferences:
		Inferences<Data::TransactionEntity, Data::TransactionBuffer, Data::TransactionDraftBuffer>,
	pub task_inferences: Inferences<Data::TaskEntity, Data::TaskBuffer, Data::TaskDraftBuffer>,
	pub message_inferences:
		Inferences<Data::MessageEntity, Data::MessageBuffer, Data::MessageDraftBuffer>,
}

impl<'a, Data: ParabyzantineData> From<ParabyzantineWorld<'a, Data>>
	for ParabyzantineHartInferences<Data>
{
	fn from(world: ParabyzantineWorld<'a, Data>) -> Self {
		ParabyzantineHartInferences {
			certificate_inferences: world.certificate_inferences,
			agreement_inferences: world.agreement_inferences,
			transaction_inferences: world.transaction_inferences,
			task_inferences: world.task_inferences,
			message_inferences: world.message_inferences,
		}
	}
}

/// A [ParabyzantineHart] trait describes operations on the parabyzantine hart.
pub trait ParabyzantineHart: Sized {
	type Binding: ParabyzantineDataBinding;

	/// Borrows the [ParabyzantineWorld] for the parabyzantine hart.
	fn parabyzantine_hart_world<'a>(
		&mut self,
		data: &'a mut <Self::Binding as ParabyzantineDataBinding>::Data,
	) -> ParabyzantineWorld<'a, <Self::Binding as ParabyzantineDataBinding>::Data> {
		data.parabyzantine_world()
	}

	/// Compute the parabyzantine hart.
	fn update_parabyzantine_hart(
		&mut self,
		data: &mut ParabyzantineWorld<<Self::Binding as ParabyzantineDataBinding>::Data>,
	);

	/// Commits the inferences for the parabyzantine hart.
	fn commit_parabyzantine_hart(
		&mut self,
		hart_inferences: ParabyzantineHartInferences<
			<Self::Binding as ParabyzantineDataBinding>::Data,
		>,
		data: &mut <Self::Binding as ParabyzantineDataBinding>::Data,
	) {
		data.commit_parabyzantine_hart(hart_inferences);
	}

	fn act_on_parabyzantine_hart(
		&mut self,
		data: &mut <Self::Binding as ParabyzantineDataBinding>::Data,
	) {
		let mut world = self.parabyzantine_hart_world(data);
		self.update_parabyzantine_hart(&mut world);
		self.commit_parabyzantine_hart(world.into(), data);
	}
}

impl<Binding: ParabyzantineDataBinding, HartHandler: ParabyzantineHart<Binding = Binding>>
	Act<Hart, Binding::Data> for HartHandler
{
	fn act(&mut self, _action: Hart, data: &mut Binding::Data) {
		self.act_on_parabyzantine_hart(data);
	}
}

/// A [ParabyzantineDataBinding] is a binding for the [Parabyzantine] protocol.
///
/// It binds to [ParabyzantineData].
pub trait ParabyzantineDataBinding {
	type Data: ParabyzantineData;
}

/// A [ParabyzantineData] for the [NoOpData] struct.
impl ParabyzantineData for NoOpData {
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
	type Data = NoOpData;
}
