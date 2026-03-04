#![no_std]

#[cfg(test)]
extern crate std;

pub mod agreement;
pub mod task;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Resample;
