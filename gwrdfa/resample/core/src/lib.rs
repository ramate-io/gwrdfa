#![cfg_attr(not(feature = "std"), no_std)]

pub mod agreement;
pub mod task;

/// Marks an artifact consumed by the resample instance.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ToResample;

/// Marks an artifact produce by the resample instance.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Resample;
