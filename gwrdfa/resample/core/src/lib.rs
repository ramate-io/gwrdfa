#![no_std]

use parabyzantine::{
	agreement::{
		AgreementWorld, ParabyzantineAgreement, ParabyzantineAgreementBinding,
		ParabyzantineAgreementData, ParabyzantineAgreementSpec,
	},
	buffer::{Bundle, Inferences, Querylike},
};

pub trait ResampleSpec<Binding: ParabyzantineAgreementBinding>: Sized {
	/// The type of the index.
	type Index: Eq;

	/// The type of the value.
	type Value: Eq;

	/// The type of the sender of a certificate.
	type Sender: Eq;

	/// The type of the subcommittee.
	type Subcommittee: Subcommittee<Self::Sender>;

	/// The bundle of the agreement in the buffer.
	type IndexSubcommitteeAgreementBundle: Bundle;

	/// The query for the index subcommittee agreement.
	type IndexSubcommitteeAgreementQuery: Querylike<
		<Binding::Spec as ParabyzantineAgreementSpec>::AgreementEntity,
		<Binding::Spec as ParabyzantineAgreementSpec>::AgreementBuffer,
		Self::IndexSubcommitteeAgreementBundle,
	>;

	/// The type of the index subcommittee agreement.
	type IndexSubcommitteeAgreement: IndexSubcommitteeAgreement<Self::Index, Self::Sender, Self::Subcommittee>
		+ for<'a> From<&'a (
			<Binding::Spec as ParabyzantineAgreementSpec>::AgreementEntity,
			Self::IndexSubcommitteeAgreementBundle,
		)>;

	/// The bundle of the certificate in the buffer.
	type CertificateBundle: Bundle;

	/// The query for the certificate.
	type CertificateQuery: Querylike<
		<Binding::Spec as ParabyzantineAgreementSpec>::CertificateEntity,
		<Binding::Spec as ParabyzantineAgreementSpec>::CertificateBuffer,
		Self::CertificateBundle,
	>;

	/// The type of the certificate.
	type Certificate: Certificate<Self::Index, Self::Value, Self::Sender>
		+ for<'a> From<&'a (
			<Binding::Spec as ParabyzantineAgreementSpec>::CertificateEntity,
			Self::CertificateBundle,
		)>;

	/// The type of the certificate set.
	type CertificateSet: CertificateSet<
		Self::Index,
		Self::Value,
		Self::Sender,
		Self::Certificate,
		Self::Subcommittee,
	>;

	/// The type of the sampler.
	type Sampler: Sampler<
		Self::Index,
		Self::Value,
		Self::Sender,
		Self::Subcommittee,
		Self::IndexSubcommitteeAgreement,
		Binding,
	>;
}

#[derive(Debug, Clone, Copy)]
pub enum Condition<Value: Eq> {
	Consensus(Value),
	Hung,
	InProgress,
}

pub trait Subcommittee<Sender: Eq>: Eq {
	fn condition<'a, Value: 'a + Eq>(
		&'a self,
		partials: impl Iterator<Item = (&'a Self, &'a Value)> + 'a,
	) -> Condition<Value>;
}

pub trait IndexSubcommitteeAgreement<Index: Eq, Sender: Eq, Sub: Subcommittee<Sender>>: Eq {
	/// The index of the agreement.
	fn index(&self) -> Index;

	/// The subcommittee of the agreement.
	fn subcommittee(&self) -> Sub;
}

pub trait Certificate<Index: Eq, Value: Eq, Sender: Eq>: Eq + Sized {
	/// The index of the message.
	fn index(&self) -> Index;

	/// The value of the message.
	fn value(&self) -> Value;

	/// The sender of the message.
	fn sender(&self) -> Sender;
}

pub trait CertificateSet<
	Index: Eq,
	Value: Eq,
	Sender: Eq,
	Item: Certificate<Index, Value, Sender>,
	Sub: Subcommittee<Sender>,
>: Eq + Sized
{
	fn contains(&self, item: &Item) -> bool;

	fn insert(&mut self, item: Item);

	fn remove(&mut self, item: Item);

	fn partial_subcommittees_for_index<'a>(
		&'a self,
		index: &Index,
	) -> impl Iterator<Item = (&'a Sub, &'a Value)> + 'a
	where
		Self: 'a,
		Sub: 'a,
		Value: 'a;

	fn partial_subcommittee_for_value<'a>(
		&'a self,
		index: &Index,
		value: &Value,
	) -> Option<(&'a Sub, &'a Value)>
	where
		Self: 'a,
		Sub: 'a,
		Value: 'a;
}

pub trait Sampler<
	Index: Eq,
	Value: Eq,
	Sender: Eq,
	Sub: Subcommittee<Sender>,
	SubAgree: IndexSubcommitteeAgreement<Index, Sender, Sub>,
	Binding: ParabyzantineAgreementBinding,
>: Sized
{
	/// Given a value and the subcommittee agreeement which gave that value,
	/// the sampler has the option to insert agreements into the buffer.
	///
	/// Note, that this is an offline rule set,
	/// so it does not depend on a consensus value itself.
	/// If you want to write a protocol which changes the rule set based on a consensus value,
	/// there are two major patterns:
	/// 1. Write your [Sampler] so that values themselves can effectively update the sampler to reflect the new rule set.
	/// 2. Write your [Sampler] so that it inserts agreements which defer the actual subcommittee election to a later stage, e.g., ParabyzantineTask.
	fn elect_subcommittees_from_consensus_value(
		&mut self,
		value: &Value,
		agreement: &SubAgree,
		agreeement_inferences: &mut Inferences<
			<Binding::Spec as ParabyzantineAgreementSpec>::AgreementEntity,
			<Binding::Spec as ParabyzantineAgreementSpec>::AgreementBuffer,
			<Binding::Spec as ParabyzantineAgreementSpec>::AgreementDraftBuffer,
		>,
	);

	/// Given a hung subcommittee agreement, the sampler has the option to insert agreements into the buffer.
	fn elect_subcommittees_from_hung_value(
		&mut self,
		agreement: &SubAgree,
		agreement_inferences: &mut Inferences<
			<Binding::Spec as ParabyzantineAgreementSpec>::AgreementEntity,
			<Binding::Spec as ParabyzantineAgreementSpec>::AgreementBuffer,
			<Binding::Spec as ParabyzantineAgreementSpec>::AgreementDraftBuffer,
		>,
	);
}

pub trait ResampleData<Binding: ParabyzantineAgreementBinding, Spec: ResampleSpec<Binding>>:
	Sized
{
	/// A [Resample] data must be able to provide a [CertificateSet]
	fn certificate_set(&self) -> &Spec::CertificateSet;

	/// A [Resample] data must be able to provide a mutable [CertificateSet]
	fn certificate_set_mut(&mut self) -> &mut Spec::CertificateSet;

	/// Resample data must be able to provide a [Sampler]
	fn sampler(&self) -> &Spec::Sampler;

	/// Resample data must be able to provide a mutable [Sampler]
	fn sampler_mut(&mut self) -> &mut Spec::Sampler;

	/// Resample data must be able to prduce a [Spec::IndexSubcommitteeAgreementQuery]
	fn index_subcommittee_agreement_query(&mut self) -> Spec::IndexSubcommitteeAgreementQuery;

	/// Resample data must be able to produce a [Spec::CertificateQuery]
	fn certificate_query(
		&mut self,
		index: &(
			<Binding::Spec as ParabyzantineAgreementSpec>::AgreementEntity,
			Spec::IndexSubcommitteeAgreementBundle,
		),
	) -> Spec::CertificateQuery;
}

pub trait ResampleBinding: Sized {
	type ParabyzantineAgreementBinding: ParabyzantineAgreementBinding;
	type ResampleSpec: ResampleSpec<Self::ParabyzantineAgreementBinding>;
	type ResampleData: ResampleData<Self::ParabyzantineAgreementBinding, Self::ResampleSpec>;
}

/// Resample wraps around the resample data indicated by the binding.
///
/// This is mainly used s.t. we can implement the foreign trait.
pub struct Resample<Binding: ResampleBinding>(pub Binding::ResampleData);

impl<Binding: ResampleBinding>
	ParabyzantineAgreement<
		<Binding::ParabyzantineAgreementBinding as ParabyzantineAgreementBinding>::Spec,
	> for Resample<Binding>
{
	fn update_parabyzantine_agreement(
		&mut self,
		agreement_world: &mut AgreementWorld<
			<Binding::ParabyzantineAgreementBinding as ParabyzantineAgreementBinding>::Spec,
		>,
	) {
		// over all the index subcommittee agreements
		let index_query = self.0.index_subcommittee_agreement_query();
		for index_bundle in agreement_world.agreement_facts.query(index_query) {
			let index: <Binding::ResampleSpec as ResampleSpec<
				Binding::ParabyzantineAgreementBinding,
			>>::IndexSubcommitteeAgreement = (&index_bundle).into();

			// insert all of the certificates for this index into the certificate set
			let certificate_query = self.0.certificate_query(&index_bundle);
			for certificate_bundle in agreement_world.certificate_facts.query(certificate_query) {
				let certificate: <Binding::ResampleSpec as ResampleSpec<
					Binding::ParabyzantineAgreementBinding,
				>>::Certificate = (&certificate_bundle).into();

				self.0.certificate_set_mut().insert(certificate);
			}

			// check the subcommittee condition
			let subcommittee_condition = index.subcommittee().condition(
				self.0.certificate_set().partial_subcommittees_for_index(&index.index()),
			);
			match subcommittee_condition {
				Condition::Consensus(value) => {
					self.0.sampler_mut().elect_subcommittees_from_consensus_value(
						&value,
						&index,
						&mut agreement_world.agreement_inferences,
					);
				}
				Condition::Hung => {
					self.0.sampler_mut().elect_subcommittees_from_hung_value(
						&index,
						&mut agreement_world.agreement_inferences,
					);
				}
				Condition::InProgress => {}
			}
		}
	}
}

impl<Binding: ResampleBinding> Resample<Binding> {
	/// A direct implementation of resampling on an agreement world.
	///
	/// This is most useful for experimenting.
	pub fn resample(
		&mut self,
		agreement_data: &<Binding::ParabyzantineAgreementBinding as ParabyzantineAgreementBinding>::Data,
	) {
		let mut agreement_world = agreement_data.parabyzantine_agreement_world();

		self.update_parabyzantine_agreement(&mut agreement_world);
	}
}
