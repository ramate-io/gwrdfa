pub trait Agreementlike {}

/// Agreements represent values that a replica asserts as agreed upon by the protocol.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Agreement<A: Agreementlike> {
	inner: A,
}

impl<A: Agreementlike> Agreement<A> {
	pub fn new(inner: A) -> Self {
		Self { inner }
	}

	pub fn inner(&self) -> &A {
		&self.inner
	}
}
