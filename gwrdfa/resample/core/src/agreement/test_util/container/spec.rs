use super::{
	TestResampleAgreementContainer, TestResampleAgreementDelta, TestResampleCertificateContainer,
	TestResampleCertificateDelta,
};
use crate::agreement::Subcommittee;
use core::marker::PhantomData;
use gwrdfa_container::{ContainerEntity, ContainerEntityBuffer, ContainerEntityDraftBuffer};
use parabyzantine::agreement::ParabyzantineAgreementDataSpec;

pub struct TestResampleParabyzantineSpec<Index: Eq, Value: Eq + 'static, Sub: Subcommittee<Value>> {
	__marker: PhantomData<(Index, Value, Sub)>,
}

impl<Index: Eq, Value: Eq + 'static, Sub: Subcommittee<Value>> ParabyzantineAgreementDataSpec
	for TestResampleParabyzantineSpec<Index, Value, Sub>
{
	type CertificateEntity = ContainerEntity;
	type CertificateBuffer =
		ContainerEntityBuffer<TestResampleCertificateContainer<Index, Value, Sub>>;
	type CertificateDraftBuffer =
		ContainerEntityDraftBuffer<TestResampleCertificateDelta<Index, Value, Sub>>;
	type AgreementEntity = ContainerEntity;
	type AgreementBuffer = ContainerEntityBuffer<TestResampleAgreementContainer<Index, Value, Sub>>;
	type AgreementDraftBuffer =
		ContainerEntityDraftBuffer<TestResampleAgreementDelta<Index, Value, Sub>>;
}
