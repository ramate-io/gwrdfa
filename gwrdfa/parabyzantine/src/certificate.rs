pub trait Certificatelike {}

/// Certificates are messages that originate within the system.
/// That is, from the participant set.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Certificate<C: Certificatelike> {
	inner: C,
}

impl<C: Certificatelike> Certificate<C> {
	pub fn new(inner: C) -> Self {
		Self { inner }
	}

	pub fn inner(&self) -> &C {
		&self.inner
	}
}
