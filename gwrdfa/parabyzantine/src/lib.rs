#![no_std]

pub mod buffer;
pub mod parabyzantine;
pub use parabyzantine::*;
pub mod act;

/// A [NoOp] is a no-op implementation of the [Act] trait.
#[derive(Debug, Clone, Copy)]
pub struct NoOp;

/// A [NoOpData] is a container for the [NoOp] struct.
#[derive(Debug, Clone, Copy)]
pub struct NoOpData {
	pub no_op: NoOp,
}

impl NoOpData {
	pub fn new() -> Self {
		Self { no_op: NoOp }
	}
}
