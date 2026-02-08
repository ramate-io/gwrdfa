use crate::buffer::{Bufferlike, DraftBufferlike};

pub trait ParabyzantineWorldSpec {
	type CertificateEntity: Sized;
	type CertificateBuffer: Bufferlike<Self::CertificateEntity>;
	type CertificateDraftBuffer: DraftBufferlike<Self::CertificateEntity, Self::CertificateBuffer>;
	type AgreementEntity: Sized;
	type AgreementBuffer: Bufferlike<Self::AgreementEntity>;
	type AgreementDraftBuffer: DraftBufferlike<Self::AgreementEntity, Self::AgreementBuffer>;
	type TransactionEntity: Sized;
	type TransactionBuffer: Bufferlike<Self::TransactionEntity>;
	type TransactionDraftBuffer: DraftBufferlike<Self::TransactionEntity, Self::TransactionBuffer>;
	type TaskEntity: Sized;
	type TaskBuffer: Bufferlike<Self::TaskEntity>;
	type TaskDraftBuffer: DraftBufferlike<Self::TaskEntity, Self::TaskBuffer>;
	type BroadcastEntity: Sized;
	type BroadcastBuffer: Bufferlike<Self::BroadcastEntity>;
	type BroadcastDraftBuffer: DraftBufferlike<Self::BroadcastEntity, Self::BroadcastBuffer>;
}
