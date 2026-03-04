#![cfg_attr(not(feature = "std"), no_std)]

pub mod agreement;
pub mod task;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Resample;
