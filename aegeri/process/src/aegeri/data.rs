use aegeri_message::{AegeriSubcommittee, Index, Proposal};
use gwrdfa_container::{ContainerEntity, ContainerEntityBuffer, ContainerEntityDraftBuffer};
use gwrdfa_resample::agreement::std::container::{
	AgreementContainer, AgreementDelta, CertificateContainer, CertificateDelta,
};
use parabyzantine::{NoOp, NoOpData, ParabyzantineData};

pub struct AegeriData {
	pub certificates:
		ContainerEntityBuffer<CertificateContainer<Index, Proposal, AegeriSubcommittee>>,
	pub agreements: ContainerEntityBuffer<AgreementContainer<Index, Proposal, AegeriSubcommittee>>,
	pub noop: NoOpData,
}

impl ParabyzantineData for AegeriData {
	// Message is not used in Aegeri.
	type MessageEntity = NoOp;
	type MessageBuffer = NoOp;
	type MessageDraftBuffer = NoOp;

	// Certificates and agreements are stored in the same container.
	type CertificateEntity = ContainerEntity;
	type CertificateBuffer =
		ContainerEntityBuffer<CertificateContainer<Index, Proposal, AegeriSubcommittee>>;
	type CertificateDraftBuffer =
		ContainerEntityDraftBuffer<CertificateDelta<Index, Proposal, AegeriSubcommittee>>;

	// Certificates and agreements are stored in the same container.
	type AgreementEntity = ContainerEntity;
	type AgreementBuffer =
		ContainerEntityBuffer<AgreementContainer<Index, Proposal, AegeriSubcommittee>>;
	type AgreementDraftBuffer =
		ContainerEntityDraftBuffer<AgreementDelta<Index, Proposal, AegeriSubcommittee>>;

	// Transactions are not used in Aegeri.
	type TransactionEntity = NoOp;
	type TransactionBuffer = NoOp;
	type TransactionDraftBuffer = NoOp;

	// Tasks are not used in Aegeri.
	type TaskEntity = NoOp;
	type TaskBuffer = NoOp;
	type TaskDraftBuffer = NoOp;

	fn parabyzantine_certificate_buffer(&self) -> &Self::CertificateBuffer {
		&self.certificates
	}

	fn parabyzantine_certificate_buffer_mut(&mut self) -> &mut Self::CertificateBuffer {
		&mut self.certificates
	}

	fn parabyzantine_certificate_draft_buffer(&self) -> Self::CertificateDraftBuffer {
		Self::CertificateDraftBuffer::new()
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

	fn parabyzantine_transaction_buffer(&self) -> &Self::TransactionBuffer {
		&self.noop.no_op
	}

	fn parabyzantine_transaction_buffer_mut(&mut self) -> &mut Self::TransactionBuffer {
		&mut self.noop.no_op
	}

	fn parabyzantine_transaction_draft_buffer(&self) -> Self::TransactionDraftBuffer {
		Self::TransactionDraftBuffer::default()
	}

	fn parabyzantine_task_buffer(&self) -> &Self::TaskBuffer {
		&self.noop.no_op
	}

	fn parabyzantine_task_buffer_mut(&mut self) -> &mut Self::TaskBuffer {
		&mut self.noop.no_op
	}

	fn parabyzantine_task_draft_buffer(&self) -> Self::TaskDraftBuffer {
		Self::TaskDraftBuffer::default()
	}

	fn parabyzantine_message_buffer(&self) -> &Self::MessageBuffer {
		&self.noop.no_op
	}

	fn parabyzantine_message_buffer_mut(&mut self) -> &mut Self::MessageBuffer {
		&mut self.noop.no_op
	}

	fn parabyzantine_message_draft_buffer(&self) -> Self::MessageDraftBuffer {
		Self::MessageDraftBuffer::default()
	}
}
