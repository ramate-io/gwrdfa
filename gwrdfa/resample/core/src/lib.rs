#![no_std]

use parabyzantine::{
	agreement::{
		AgreementWorld, ParabyzantineAgreement, ParabyzantineAgreementData,
		ParabyzantineAgreementSpec,
	},
	buffer::Bundle,
};

pub trait ResampleSpec: Sized {
	/// The type of the index.
	type Index: Eq;

	/// The type of the value.
	type Value: Eq;

	/// The type of the sender of a certificate.
	type Sender: Eq;

	/// The entity of the agreement
	type AgreementEntity: Sized;

	/// The bundle of the agreement in the buffer.
	type IndexSubcommitteeAgreementBundle: Bundle;

	/// The type of the index subcommittee agreement.
	type IndexSubcommitteeAgreement: IndexAgreement<Self::Index>
		+ From<(Self::AgreementEntity, Self::IndexSubcommitteeAgreementBundle)>;

	/// The type of the subcommittee.
	type Subcommittee: Subcommittee<Self::Sender>;

	/// The entity of the agreement.
	type CertificateEntity: Sized;

	/// The bundle of the message in the buffer.
	type CertificateBundle: Bundle;

	/// The type of the certificate.
	type Certificate: Certificate<Self::Index, Self::Value, Self::Sender>
		+ From<(Self::CertificateEntity, Self::CertificateBundle)>;
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

pub trait IndexAgreement<Index: Eq, Sender: Eq, Sub: Subcommittee<Sender>>: Eq {
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

	fn iter<'a>(&'a self) -> impl Iterator<Item = &'a Item> + 'a
	where
		Item: 'a;

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

pub trait Sampler<Index: Eq, Value: Eq, Sender: Eq, Cert: Certificate<Index, Value, Sender>>:
	Sized
{
	fn subcommittee(&self, message: &Cert) -> impl Iterator<Item = Sender>;
}

pub trait Resample: Sized {}
