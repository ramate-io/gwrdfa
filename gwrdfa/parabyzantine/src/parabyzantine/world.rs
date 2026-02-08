pub mod as_agreement;
pub mod spec;

pub use spec::ParabyzantineWorldSpec;

use crate::buffer::{facts::Facts, inferences::Inferences};

pub trait ParabyzantineWorld<Spec: ParabyzantineWorldSpec> {
	/// Gets the certificate buffer.
	fn certificate_buffer(&self) -> &Spec::CertificateBuffer;

	/// Gets the certificate facts.
	fn certificate_facts(&self) -> Facts<Spec::CertificateEntity, Spec::CertificateBuffer> {
		Facts::new(self.certificate_buffer())
	}

	/// Gets the certificate draft buffer.
	fn certificate_draft_buffer(&self) -> Spec::CertificateDraftBuffer;

	/// Gets the certificate inferences.
	fn certificate_inferences(
		&self,
	) -> Inferences<Spec::CertificateEntity, Spec::CertificateBuffer, Spec::CertificateDraftBuffer>
	{
		Inferences::new(self.certificate_draft_buffer())
	}

	/// Gets the agreement buffer.
	fn agreement_buffer(&self) -> &Spec::AgreementBuffer;

	/// Gets the agreement facts.
	fn agreement_facts(&self) -> Facts<Spec::AgreementEntity, Spec::AgreementBuffer> {
		Facts::new(self.agreement_buffer())
	}

	/// Gets the agreement draft buffer.
	fn agreement_draft_buffer(&self) -> Spec::AgreementDraftBuffer;

	/// Gets the agreement inferences.
	fn agreement_inferences(
		&self,
	) -> Inferences<Spec::AgreementEntity, Spec::AgreementBuffer, Spec::AgreementDraftBuffer> {
		Inferences::new(self.agreement_draft_buffer())
	}

	/// Gets the transaction buffer.
	fn transaction_buffer(&self) -> &Spec::TransactionBuffer;

	/// Gets the transaction facts.
	fn transaction_facts(&self) -> Facts<Spec::TransactionEntity, Spec::TransactionBuffer> {
		Facts::new(self.transaction_buffer())
	}

	/// Gets the transaction draft buffer.
	fn transaction_draft_buffer(&self) -> Spec::TransactionDraftBuffer;

	/// Gets the transaction inferences.
	fn transaction_inferences(
		&self,
	) -> Inferences<Spec::TransactionEntity, Spec::TransactionBuffer, Spec::TransactionDraftBuffer>
	{
		Inferences::new(self.transaction_draft_buffer())
	}

	/// Gets the task buffer.
	fn task_buffer(&self) -> &Spec::TaskBuffer;

	/// Gets the task facts.
	fn task_facts(&self) -> Facts<Spec::TaskEntity, Spec::TaskBuffer> {
		Facts::new(self.task_buffer())
	}

	/// Gets the task draft buffer.
	fn task_draft_buffer(&self) -> Spec::TaskDraftBuffer;

	/// Gets the task inferences.
	fn task_inferences(
		&self,
	) -> Inferences<Spec::TaskEntity, Spec::TaskBuffer, Spec::TaskDraftBuffer> {
		Inferences::new(self.task_draft_buffer())
	}

	/// Gets the broadcast buffer.
	fn broadcast_buffer(&self) -> &Spec::BroadcastBuffer;

	/// Gets the broadcast facts.
	fn broadcast_facts(&self) -> Facts<Spec::BroadcastEntity, Spec::BroadcastBuffer> {
		Facts::new(self.broadcast_buffer())
	}

	/// Gets the broadcast draft buffer.
	fn broadcast_draft_buffer(&self) -> Spec::BroadcastDraftBuffer;

	/// Gets the broadcast inferences.
	fn broadcast_inferences(
		&self,
	) -> Inferences<Spec::BroadcastEntity, Spec::BroadcastBuffer, Spec::BroadcastDraftBuffer> {
		Inferences::new(self.broadcast_draft_buffer())
	}
}
