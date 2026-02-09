#![no_std]

use parabyzantine::{
	agreement::{
		AgreementWorld, ParabyzantineAgreement, ParabyzantineAgreementData,
		ParabyzantineAgreementProtocol, ParabyzantineAgreementSpec,
	},
	buffer::Bundle,
	Container, Member,
};

pub trait ResampleSpec<Protocol: ParabyzantineAgreementProtocol, Data: ResampleData<Protocol, Self>>:
	Sized
{
	/// The type of the index.
	type Index: Eq;

	/// The type of the value.
	type Value: Eq;

	/// The type of the sender of a certificate.
	type Sender: Eq;

	/// The bundle of the agreement in the buffer.
	type IndexSubcommitteeAgreementBundle: Bundle;

	/// The type of the index subcommittee agreement.
	type IndexSubcommitteeAgreement: IndexSubcommitteeAgreement<Self::Index, Self::Sender, Self::Subcommittee>
		+ From<(
			<Protocol::Spec as ParabyzantineAgreementSpec<Protocol::Data>>::AgreementEntity,
			Self::IndexSubcommitteeAgreementBundle,
		)>;

	/// The type of the subcommittee.
	type Subcommittee: Subcommittee<Self::Sender>;

	/// The bundle of the message in the buffer.
	type CertificateBundle: Bundle;

	/// The type of the certificate.
	type Certificate: Certificate<Self::Index, Self::Value, Self::Sender>
		+ From<(
			<Protocol::Spec as ParabyzantineAgreementSpec<Protocol::Data>>::CertificateEntity,
			Self::CertificateBundle,
		)>;

	/// The type of the certificate set.
	type CertificateSet: CertificateSet<
			Self::Index,
			Self::Value,
			Self::Sender,
			Self::Certificate,
			Self::Subcommittee,
		> + Member<Data>;
}

pub trait Subcommittee<Sender: Eq>: Eq {
	/// Adds a member to the subcommittee.
	fn add(&mut self, member: Sender);

	/// Removes a member from the subcommittee.
	fn remove(&mut self, member: Sender);

	/// Checks if a member is in the subcommittee.
	fn contains(&self, member: &Sender) -> bool;

	/// The length of the subcommittee.
	fn len(&self) -> usize;
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
	) -> impl Iterator<Item = Sub> + 'a
	where
		Self: 'a;

	fn partial_subcommittees_for_value<'a>(
		&'a self,
		index: &Index,
		value: &Value,
	) -> impl Iterator<Item = Sub> + 'a
	where
		Self: 'a;
}

pub trait Sampler<
	Index: Eq,
	Value: Eq,
	Sender: Eq,
	Sub: Subcommittee<Sender>,
	SubAgree: IndexSubcommitteeAgreement<Index, Sender, Sub>,
>: Sized
{
	/// Given a value and the subcommittee agreeement which
	///
	/// Note, this does not internally validate whether the subcommittee voted on the value.
	/// It assumes that has already been checked.
	fn subcommittee(&self, value: &Value, agreement: &SubAgree) -> Sub;
}

pub trait ResampleData<Protocol: ParabyzantineAgreementProtocol, Spec: ResampleSpec<Protocol, Self>>:
	Sized
{
	/// A [Resample] environment must be able to provide a [CertificateSet]
	fn certificate_set(&self) -> &Spec::CertificateSet;

	/// A [Resample] environment must be able to provide a mutable [CertificateSet]
	fn certificate_set_mut(&mut self) -> &mut Spec::CertificateSet;
}

impl<
		Protocol: ParabyzantineAgreementProtocol,
		Spec: ResampleSpec<Protocol, Data>,
		Data: ResampleData<Protocol, Spec>,
	> ResampleData<Protocol, Spec> for Data
where
	Spec::CertificateSet: Member<Data>,
{
	fn certificate_set(&self) -> &Spec::CertificateSet {
		self.member::<Spec::CertificateSet>()
	}

	fn certificate_set_mut(&mut self) -> &mut Spec::CertificateSet {
		self.member_mut::<Spec::CertificateSet>()
	}
}

pub trait ResampleProtocol {
	type ParabyzantineAgreementProtocol: ParabyzantineAgreementProtocol;
	type ResampleSpec: ResampleSpec<Self::ParabyzantineAgreementProtocol, Self::ResampleData>;
	type ResampleData: ResampleData<Self::ParabyzantineAgreementProtocol, Self::ResampleSpec>;
}

pub struct ResampleProtocolWrapper<Protocol: ResampleProtocol>(pub Protocol);

impl<Protocol: ResampleProtocol>
	ParabyzantineAgreement<
		<Protocol::ParabyzantineAgreementProtocol as ParabyzantineAgreementProtocol>::Spec,
		<Protocol::ParabyzantineAgreementProtocol as ParabyzantineAgreementProtocol>::Data,
	> for ResampleProtocolWrapper<Protocol>
{
	fn prepare_parabyzantine_agreement(
		&mut self,
		data: &mut AgreementWorld<
			<Protocol::ParabyzantineAgreementProtocol as ParabyzantineAgreementProtocol>::Spec,
			<Protocol::ParabyzantineAgreementProtocol as ParabyzantineAgreementProtocol>::Data,
		>,
	) {
		todo!()
	}

	fn compute_parabyzantine_agreement(
		&mut self,
		data: &mut AgreementWorld<
			<Protocol::ParabyzantineAgreementProtocol as ParabyzantineAgreementProtocol>::Spec,
			<Protocol::ParabyzantineAgreementProtocol as ParabyzantineAgreementProtocol>::Data,
		>,
	) {
		todo!()
	}

	fn commit_parabyzantine_agreement(
		&mut self,
		data: &mut AgreementWorld<
			<Protocol::ParabyzantineAgreementProtocol as ParabyzantineAgreementProtocol>::Spec,
			<Protocol::ParabyzantineAgreementProtocol as ParabyzantineAgreementProtocol>::Data,
		>,
	) {
		todo!()
	}
}
