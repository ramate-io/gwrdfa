use crate::buffers::{
	TaskContainer, TaskDeltasContainer, TransactionContainer, TransactionDeltasContainer,
};
use aegeri_message::{AegeriSubcommittee, Index, Proposal, UnifiedMessage};
use gossamer::container::{GossamerContainer, GossamerDeltasContainer};
use gwrdfa_container::{ContainerEntity, ContainerEntityBuffer, ContainerEntityDraftBuffer};
use gwrdfa_resample::agreement::std::container::{
	AgreementContainer, AgreementDelta, CertificateContainer, CertificateDelta,
};
use parabyzantine::ParabyzantineData;

pub struct AegeriParabyzantineData {
	pub messages: ContainerEntityBuffer<GossamerContainer<UnifiedMessage>>,
	pub certificates:
		ContainerEntityBuffer<CertificateContainer<Index, Proposal, AegeriSubcommittee>>,
	pub transactions: ContainerEntityBuffer<TransactionContainer>,
	pub agreements: ContainerEntityBuffer<AgreementContainer<Index, Proposal, AegeriSubcommittee>>,
	pub tasks: ContainerEntityBuffer<TaskContainer>,
}

impl AegeriParabyzantineData {
	pub fn new() -> Self {
		Self {
			messages: ContainerEntityBuffer::new(),
			certificates: ContainerEntityBuffer::new(),
			agreements: ContainerEntityBuffer::new(),
			transactions: ContainerEntityBuffer::new(),
			tasks: ContainerEntityBuffer::new(),
		}
	}
}

impl ParabyzantineData for AegeriParabyzantineData {
	// Message is just gossamer messages over [UnifiedMessage].
	type MessageEntity = ContainerEntity;
	type MessageBuffer = ContainerEntityBuffer<GossamerContainer<UnifiedMessage>>;
	type MessageDraftBuffer = ContainerEntityDraftBuffer<GossamerDeltasContainer<UnifiedMessage>>;

	// Certificates are stoed in a ResampleAgreement container
	type CertificateEntity = ContainerEntity;
	type CertificateBuffer =
		ContainerEntityBuffer<CertificateContainer<Index, Proposal, AegeriSubcommittee>>;
	type CertificateDraftBuffer =
		ContainerEntityDraftBuffer<CertificateDelta<Index, Proposal, AegeriSubcommittee>>;

	// Transactions are stored in a TransactionContainer
	type TransactionEntity = ContainerEntity;
	type TransactionBuffer = ContainerEntityBuffer<TransactionContainer>;
	type TransactionDraftBuffer = ContainerEntityDraftBuffer<TransactionDeltasContainer>;

	// Agreements are stored in a ResampleAgreement container
	type AgreementEntity = ContainerEntity;
	type AgreementBuffer =
		ContainerEntityBuffer<AgreementContainer<Index, Proposal, AegeriSubcommittee>>;
	type AgreementDraftBuffer =
		ContainerEntityDraftBuffer<AgreementDelta<Index, Proposal, AegeriSubcommittee>>;

	// Tasks are stored in a TaskContainer
	type TaskEntity = ContainerEntity;
	type TaskBuffer = ContainerEntityBuffer<TaskContainer>;
	type TaskDraftBuffer = ContainerEntityDraftBuffer<TaskDeltasContainer>;

	fn parabyzantine_message_buffer(&self) -> &Self::MessageBuffer {
		&self.messages
	}

	fn parabyzantine_message_buffer_mut(&mut self) -> &mut Self::MessageBuffer {
		&mut self.messages
	}

	fn parabyzantine_message_draft_buffer(&self) -> Self::MessageDraftBuffer {
		Self::MessageDraftBuffer::new()
	}

	fn parabyzantine_certificate_buffer(&self) -> &Self::CertificateBuffer {
		&self.certificates
	}

	fn parabyzantine_certificate_buffer_mut(&mut self) -> &mut Self::CertificateBuffer {
		&mut self.certificates
	}

	fn parabyzantine_certificate_draft_buffer(&self) -> Self::CertificateDraftBuffer {
		Self::CertificateDraftBuffer::new()
	}

	fn parabyzantine_transaction_buffer(&self) -> &Self::TransactionBuffer {
		&self.transactions
	}

	fn parabyzantine_transaction_buffer_mut(&mut self) -> &mut Self::TransactionBuffer {
		&mut self.transactions
	}

	fn parabyzantine_transaction_draft_buffer(&self) -> Self::TransactionDraftBuffer {
		Self::TransactionDraftBuffer::new()
	}

	fn parabyzantine_agreement_buffer(&self) -> &Self::AgreementBuffer {
		&self.agreements
	}

	fn parabyzantine_agreement_buffer_mut(&mut self) -> &mut Self::AgreementBuffer {
		&mut self.agreements
	}

	fn parabyzantine_agreement_draft_buffer(&self) -> Self::AgreementDraftBuffer {
		Self::AgreementDraftBuffer::new()
	}

	fn parabyzantine_task_buffer(&self) -> &Self::TaskBuffer {
		&self.tasks
	}

	fn parabyzantine_task_buffer_mut(&mut self) -> &mut Self::TaskBuffer {
		&mut self.tasks
	}

	fn parabyzantine_task_draft_buffer(&self) -> Self::TaskDraftBuffer {
		Self::TaskDraftBuffer::new()
	}
}
