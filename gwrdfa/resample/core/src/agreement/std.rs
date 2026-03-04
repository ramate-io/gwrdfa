//! Standard-library support types for `resample::agreement`.
//!
//! The core crate remains `no_std` capable, but this module provides ergonomic
//! in-memory/reference implementations for:
//! - typed wrappers (`Index`, `Value`, `Subcom`) to avoid tuple mixups,
//! - sampler and committee policies (`ConstantCommittee`, `VoterSet`),
//! - container-backed data adapters (`container::*`),
//! - an opinionated data implementation (`MemoryAgreementData`).
//!
//! All items here are gated behind `cfg(any(test, feature = "std"))`.

pub mod container;
pub mod agreement_data;
pub mod constant_committee;
pub mod voter_set;

use crate::agreement::{Condition, Subcommittee};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Index<T: Eq>(pub T);

impl<T: Eq> Index<T> {
	pub fn new(value: T) -> Self {
		Self(value)
	}
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Value<T: Eq + 'static>(pub T);

impl<T: Eq + 'static> Value<T> {
	pub fn new(value: T) -> Self {
		Self(value)
	}
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Subcom<T: Eq>(pub T);

impl<T: Eq> Subcom<T> {
	pub fn new(value: T) -> Self {
		Self(value)
	}
}

impl<T: Eq + 'static + Subcommittee<V>, V: Eq + 'static> Subcommittee<Value<V>> for Subcom<T> {
	fn condition<'a>(
		&'a self,
		partials: impl Iterator<Item = (&'a Self, &'a Value<V>)> + 'a,
	) -> Condition<Value<V>> {
		self.0
			.condition(partials.map(|(committee, vote)| (&committee.0, &vote.0)))
			.map(Value)
	}
}

/// Minimal helper trait used by [`ConstantCommittee`] to derive the next index.
///
/// This keeps the sampler generic across index types without requiring arithmetic.
pub trait NextRound: Sized {
	fn next(&self) -> Option<Self>;
}

impl NextRound for u32 {
	fn next(&self) -> Option<Self> {
		Some(self + 1)
	}
}

impl<T: Eq + 'static + NextRound> NextRound for Index<T> {
	fn next(&self) -> Option<Self> {
		self.0.next().map(Index)
	}
}

pub use agreement_data::MemoryAgreementData;
pub use constant_committee::ConstantCommittee;
pub use voter_set::VoterSet;
pub use container::{
	AgreementContainer, AgreementDelta, AgreementParabyzantineData, CertificateContainer,
	CertificateDelta,
};
