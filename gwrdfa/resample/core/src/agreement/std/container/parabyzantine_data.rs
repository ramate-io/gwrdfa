use super::{AgreementContainer, AgreementDelta, CertificateContainer, CertificateDelta};
use crate::agreement::std::{Index, Subcom, Value};
use crate::agreement::{ResampleAgreementStorage, Subcommittee};
use crate::ForResample;
use core::marker::PhantomData;
use gwrdfa_container::{
	ContainerEntity, ContainerEntityBuffer, ContainerEntityDraftBuffer, ContainerGiving,
	DeltasContainer,
};
use parabyzantine::agreement::Agreement;
use parabyzantine::agreement::ParabyzantineAgreementData;
use crate::Resample;

/// Generic adapter from container buffers to `ParabyzantineAgreementData`.
///
/// This type is intentionally parameterized over:
/// - certificate/agreement container shapes, and
/// - certificate/agreement delta types,
///
/// so downstream code can swap data layouts while preserving the same agreement
/// execution model.
///
/// Trait bounds enforce two key contracts:
/// - containers must expose required fields via `ContainerGiving` so query plans
///   can project protocol tuples;
/// - agreement draft buffers must satisfy `ResampleAgreementStorage` to support
///   both `(Agreement, Resample, Index, Subcom)` and
///   `(Agreement, Resample, Index, Value)` inference writes.
pub struct AgreementParabyzantineData<
	I: Eq,
	V: Eq + 'static,
	S: Subcommittee<V>,
	CertContainer = CertificateContainer<I, V, S>,
	CertDelta = CertificateDelta<I, V, S>,
	AgreementContainerT = AgreementContainer<I, V, S>,
	AgreementDeltaT = AgreementDelta<I, V, S>,
> {
	pub certificate_buffer: ContainerEntityBuffer<CertContainer>,
	pub agreement_buffer: ContainerEntityBuffer<AgreementContainerT>,
	_phantom: PhantomData<(CertDelta, AgreementDeltaT, I, V, S)>,
}

impl<
		I: Eq,
		V: Eq + 'static,
		S: Subcommittee<V>,
		CertContainer,
		CertDelta,
		AgreementContainerT,
		AgreementDeltaT,
	> AgreementParabyzantineData<I, V, S, CertContainer, CertDelta, AgreementContainerT, AgreementDeltaT>
{
	/// Builds empty certificate/agreement buffers using default container buffers.
	pub fn new() -> Self {
		Self {
			certificate_buffer: ContainerEntityBuffer::new(),
			agreement_buffer: ContainerEntityBuffer::new(),
			_phantom: PhantomData,
		}
	}
}

impl<
		I: Eq,
		V: Eq + 'static,
		S: Subcommittee<V>,
		CertContainer,
		CertDelta,
		AgreementContainerT,
		AgreementDeltaT,
	> ParabyzantineAgreementData
	for AgreementParabyzantineData<I, V, S, CertContainer, CertDelta, AgreementContainerT, AgreementDeltaT>
where
	CertContainer: ContainerGiving<ForResample>
		+ ContainerGiving<Index<I>>
		+ ContainerGiving<Value<V>>
		+ ContainerGiving<Subcom<S>>,
	AgreementContainerT: ContainerGiving<Agreement>
		+ ContainerGiving<Resample>
		+ ContainerGiving<Index<I>>
		+ ContainerGiving<Value<V>>
		+ ContainerGiving<Subcom<S>>,
	CertDelta: DeltasContainer<CertContainer>,
	AgreementDeltaT: DeltasContainer<AgreementContainerT>,
	ContainerEntityDraftBuffer<AgreementDeltaT>:
		ResampleAgreementStorage<ContainerEntity, Index<I>, Subcom<S>, Value<V>>,
{
	type CertificateEntity = ContainerEntity;
	type CertificateBuffer = ContainerEntityBuffer<CertContainer>;
	type CertificateDraftBuffer = ContainerEntityDraftBuffer<CertDelta>;
	type AgreementEntity = ContainerEntity;
	type AgreementBuffer = ContainerEntityBuffer<AgreementContainerT>;
	type AgreementDraftBuffer = ContainerEntityDraftBuffer<AgreementDeltaT>;

	fn parabyzantine_agreement_certificate_buffer(
		&self,
	) -> &ContainerEntityBuffer<CertContainer> {
		&self.certificate_buffer
	}

	fn parabyzantine_agreement_certificate_buffer_mut(
		&mut self,
	) -> &mut ContainerEntityBuffer<CertContainer> {
		&mut self.certificate_buffer
	}

	fn parabyzantine_agreement_certificate_draft_buffer(
		&self,
	) -> ContainerEntityDraftBuffer<CertDelta> {
		ContainerEntityDraftBuffer::new()
	}

	fn parabyzantine_agreement_agreement_buffer(
		&self,
	) -> &ContainerEntityBuffer<AgreementContainerT> {
		&self.agreement_buffer
	}

	fn parabyzantine_agreement_agreement_buffer_mut(
		&mut self,
	) -> &mut ContainerEntityBuffer<AgreementContainerT> {
		&mut self.agreement_buffer
	}

	fn parabyzantine_agreement_agreement_draft_buffer(
		&self,
	) -> ContainerEntityDraftBuffer<AgreementDeltaT> {
		ContainerEntityDraftBuffer::new()
	}
}
