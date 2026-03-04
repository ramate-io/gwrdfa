pub mod container;

use crate::agreement::{Condition, Subcommittee};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Round<T: Eq>(pub T);

impl<T: Eq> Round<T> {
	pub fn new(value: T) -> Self {
		Self(value)
	}
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vote<T: Eq + 'static>(pub T);

impl<T: Eq + 'static> Vote<T> {
	pub fn new(value: T) -> Self {
		Self(value)
	}
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommitteeRef<T: Eq>(pub T);

impl<T: Eq> CommitteeRef<T> {
	pub fn new(value: T) -> Self {
		Self(value)
	}
}

impl<T: Eq + 'static + Subcommittee<V>, V: Eq + 'static> Subcommittee<Vote<V>> for CommitteeRef<T> {
	fn condition<'a>(
		&'a self,
		partials: impl Iterator<Item = (&'a Self, &'a Vote<V>)> + 'a,
	) -> Condition<Vote<V>> {
		self.0
			.condition(partials.map(|(committee, vote)| (&committee.0, &vote.0)))
			.map(Vote)
	}
}

pub trait NextRound: Sized {
	fn next(&self) -> Option<Self>;
}

impl NextRound for u32 {
	fn next(&self) -> Option<Self> {
		Some(self + 1)
	}
}

impl<T: Eq + 'static + NextRound> NextRound for Round<T> {
	fn next(&self) -> Option<Self> {
		self.0.next().map(Round)
	}
}

pub use super::data::test::AgreementData;
pub use super::sampler::test::ConstantCommittee;
pub use super::subcommittee::test::Committee;
pub use container::{
	AgreementContainer, AgreementDelta, AgreementParabyzantineData, CertificateContainer,
	CertificateDelta,
};
