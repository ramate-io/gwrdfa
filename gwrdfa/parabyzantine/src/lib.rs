#![no_std]

pub mod buffer;
pub mod parabyzantine;
pub use parabyzantine::*;
pub mod act;

use core::marker::PhantomData;

/// A [Spec] is a specification for a parabyzantine system.
#[derive(Debug, Clone, Copy)]
pub struct Spec<T> {
	phantom: PhantomData<T>,
}

impl<T> Spec<T> {
	pub fn new() -> Self {
		Self { phantom: PhantomData }
	}
}

/// A [NoOp] is a no-op implementation of the [Act] trait.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
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

impl From<&NoOp> for NoOp {
	fn from(_: &NoOp) -> Self {
		NoOp
	}
}

impl From<&(NoOp, NoOp)> for NoOp {
	fn from((_entity, _bundle): &(NoOp, NoOp)) -> Self {
		NoOp
	}
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct TypedNoOp<T> {
	pub no_op: NoOp,
	pub phantom: PhantomData<T>,
}

impl<T> TypedNoOp<T> {
	pub fn new() -> Self {
		Self { no_op: NoOp, phantom: PhantomData }
	}
}
