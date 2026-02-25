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
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub struct NoOp;

/// A [NoOpData] is a container for the [NoOp] struct.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoOpData {
	pub no_op_0: NoOp,
	pub no_op_1: NoOp,
	pub no_op_2: NoOp,
	pub no_op_3: NoOp,
	pub no_op_4: NoOp,
	pub no_op_5: NoOp,
	pub no_op_6: NoOp,
	pub no_op_7: NoOp,
	pub no_op_8: NoOp,
	pub no_op_9: NoOp,
	pub no_op_10: NoOp,
	pub no_op_11: NoOp,
	pub no_op_12: NoOp,
	pub no_op_13: NoOp,
	pub no_op_14: NoOp,
	pub no_op_15: NoOp,
}

impl NoOpData {
	pub fn new() -> Self {
		Self::default()
	}
}

impl From<&NoOp> for NoOp {
	fn from(_: &NoOp) -> Self {
		NoOp
	}
}

impl<T> From<(T, NoOp)> for NoOp {
	fn from((_entity, _bundle): (T, NoOp)) -> Self {
		NoOp
	}
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct NoOpOn<T>(PhantomData<T>);

impl<T> NoOpOn<T> {
	pub fn new() -> Self {
		Self(PhantomData)
	}
}
