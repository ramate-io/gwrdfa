#![no_std]

#[cfg(test)]
extern crate std;

pub mod agreement;
pub mod task;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Resample;
