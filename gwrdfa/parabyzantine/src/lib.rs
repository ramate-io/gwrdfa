#![no_std]

pub mod buffer;
pub mod parabyzantine;
pub use parabyzantine::*;
pub mod act;

/// A [NoOp] is a no-op implementation of the [Act] trait.
#[derive(Debug, Clone, Copy)]
pub struct NoOp;
