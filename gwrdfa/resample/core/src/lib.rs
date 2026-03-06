#![cfg_attr(not(feature = "std"), no_std)]

//! `gwrdfa-resample` provides the resample protocol building blocks used by
//! higher-order parabyzantine systems.
//!
//! Design split:
//! - `agreement`: derives/advances resample agreements from certificates.
//! - `task`: executes follow-up task logic based on agreement outcomes.
//!
//! The crate is `no_std` capable. Convenience in-memory/std support lives under
//! `agreement::std` and is gated behind `cfg(any(test, feature = "std"))`.

pub mod agreement;
pub mod task;

/// Marks an artifact consumed by the resample instance.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ForResample;

/// Marks an artifact produce by the resample instance.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Resample;
