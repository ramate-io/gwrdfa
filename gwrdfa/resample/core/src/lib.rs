#![no_std]

use parabyzantine::{
	agreement::{
		AgreementWorld, ParabyzantineAgreement, ParabyzantineAgreementBinding,
		ParabyzantineAgreementSpec,
	},
	buffer::Bundle,
	Container, Member,
};

pub trait ResampleSpec<Binding: ResampleBinding>: Sized {
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
			<<Binding::ParabyzantineAgreementBinding as ParabyzantineAgreementBinding>::Spec as ParabyzantineAgreementSpec<
				Binding::ParabyzantineAgreementBinding,
			>>::AgreementEntity,
			Self::IndexSubcommitteeAgreementBundle,
		)>;

	/// The type of the subcommittee.
	type Subcommittee: Subcommittee<Self::Sender>;

	/// The bundle of the message in the buffer.
	type CertificateBundle: Bundle;

	/// The type of the certificate.
	type Certificate: Certificate<Self::Index, Self::Value, Self::Sender>
		+ From<(
			<<Binding::ParabyzantineAgreementBinding as ParabyzantineAgreementBinding>::Spec as ParabyzantineAgreementSpec<
				Binding::ParabyzantineAgreementBinding,
			>>::CertificateEntity,
			Self::CertificateBundle,
		)>;

	/// The type of the certificate set.
	type CertificateSet: CertificateSet<
			Self::Index,
			Self::Value,
			Self::Sender,
			Self::Certificate,
			Self::Subcommittee,
		> + Member<Binding::ResampleData>;
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

pub trait ResampleData<Binding: ResampleBinding>: Sized {
	/// A [Resample] data must be able to provide a [CertificateSet]
	fn certificate_set(&self) -> &<Binding::ResampleSpec as ResampleSpec<Binding>>::CertificateSet;

	/// A [Resample] data must be able to provide a mutable [CertificateSet]
	fn certificate_set_mut(
		&mut self,
	) -> &mut <Binding::ResampleSpec as ResampleSpec<Binding>>::CertificateSet;
}

impl<Binding: ResampleBinding> ResampleData<Binding> for Binding::ResampleData {
	fn certificate_set(&self) -> &<Binding::ResampleSpec as ResampleSpec<Binding>>::CertificateSet {
		self.member::<<Binding::ResampleSpec as ResampleSpec<Binding>>::CertificateSet>()
	}

	fn certificate_set_mut(
		&mut self,
	) -> &mut <Binding::ResampleSpec as ResampleSpec<Binding>>::CertificateSet {
		self.member_mut::<<Binding::ResampleSpec as ResampleSpec<Binding>>::CertificateSet>()
	}
}

pub trait ResampleBinding: Sized {
	type ParabyzantineAgreementBinding: ParabyzantineAgreementBinding;
	type ResampleSpec: ResampleSpec<Self>;
	type ResampleData: ResampleData<Self>;
}

pub struct Resample<Binding: ResampleBinding>(pub Binding);

impl<Binding: ResampleBinding> ParabyzantineAgreement<Binding::ParabyzantineAgreementBinding>
	for Resample<Binding>
{
	fn update_parabyzantine_agreement(
		&mut self,
		_data: &mut AgreementWorld<Binding::ParabyzantineAgreementBinding>,
	) {
		todo!()
	}
}
