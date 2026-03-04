use super::{CertificateSet, ResampleAgreementStorage, Sampler, Subcommittee};
use parabyzantine::agreement::ParabyzantineAgreementData;
use parabyzantine::buffer::query::{QueryPlanlike, Querylike};
use parabyzantine::{NoOp, NoOpData};

pub trait ResampleAgreementData<Data: ParabyzantineAgreementData>: Sized
where
	Data::AgreementDraftBuffer:
		ResampleAgreementStorage<
			Data::AgreementEntity,
			Self::Index,
			Self::Subcommittee,
			Self::Value,
		>,
{
	type Index: Clone + Eq;
	type Value: Clone + Eq + 'static;
	type Subcommittee: Subcommittee<Self::Value> + Clone;
	type IndexSubcommitteeAgreementQuery<'a>: Querylike<
		Data::AgreementEntity,
		(&'a Self::Index, &'a Self::Subcommittee),
	>
	where
		Self::Index: 'a,
		Self::Subcommittee: 'a;
	type IndexSubcommitteeAgreementQueryPlan: for<'a> QueryPlanlike<
		Data::AgreementEntity,
		&'a Data::AgreementBuffer,
		(&'a Self::Index, &'a Self::Subcommittee),
		Self::IndexSubcommitteeAgreementQuery<'a>,
	>;
	type CertificateQuery<'a>: Querylike<
		Data::CertificateEntity,
		(&'a Self::Index, &'a Self::Value, &'a Self::Subcommittee),
	>
	where
		Self::Index: 'a,
		Self::Value: 'a,
		Self::Subcommittee: 'a;
	type CertificateQueryPlan: for<'a> QueryPlanlike<
		Data::CertificateEntity,
		&'a Data::CertificateBuffer,
		(&'a Self::Index, &'a Self::Value, &'a Self::Subcommittee),
		Self::CertificateQuery<'a>,
	>;
	type CertificateSet: CertificateSet<Self::Index, Self::Value, Self::Subcommittee>;
	type Sampler: Sampler<Self::Index, Self::Value, Self::Subcommittee>;

	fn certificate_set(&self) -> &Self::CertificateSet;
	fn certificate_set_mut(&mut self) -> &mut Self::CertificateSet;
	fn sampler(&self) -> &Self::Sampler;
	fn sampler_mut(&mut self) -> &mut Self::Sampler;
	fn index_subcommittee_agreement_query_plan(&mut self) -> Self::IndexSubcommitteeAgreementQueryPlan;
	fn certificate_query_plan(&mut self, index: &Self::Index) -> Self::CertificateQueryPlan;
}

impl ResampleAgreementData<NoOpData> for NoOpData
where
	NoOp: ResampleAgreementStorage<NoOp, NoOp, NoOp, NoOp>,
{
	type Index = NoOp;
	type Value = NoOp;
	type Subcommittee = NoOp;
	type IndexSubcommitteeAgreementQuery<'a> = NoOp;
	type IndexSubcommitteeAgreementQueryPlan = NoOp;
	type CertificateQuery<'a> = NoOp;
	type CertificateQueryPlan = NoOp;
	type CertificateSet = NoOp;
	type Sampler = NoOp;

	fn certificate_set(&self) -> &NoOp {
		&self.no_op
	}
	fn certificate_set_mut(&mut self) -> &mut NoOp {
		&mut self.no_op
	}
	fn sampler(&self) -> &NoOp {
		&self.no_op
	}
	fn sampler_mut(&mut self) -> &mut NoOp {
		&mut self.no_op
	}
	fn index_subcommittee_agreement_query_plan(&mut self) -> NoOp {
		NoOp
	}
	fn certificate_query_plan(&mut self, _index: &NoOp) -> NoOp {
		NoOp
	}
}

