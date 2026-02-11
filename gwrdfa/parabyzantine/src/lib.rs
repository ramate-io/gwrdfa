#![no_std]

pub mod buffer;
pub mod parabyzantine;
pub use parabyzantine::*;

/// A container is a type that can contain members.
pub trait Container: Sized {
	fn member<M: Member<Self>>(&self) -> &M {
		M::member_on(self)
	}

	fn member_mut<M: Member<Self>>(&mut self) -> &mut M {
		M::member_mut_on(self)
	}
}

/// A member is a type that can be contained in a container.
pub trait Member<Container: Sized> {
	fn member_on(container: &Container) -> &Self;

	fn member_mut_on(container: &mut Container) -> &mut Self;
}

/// All sized types are containers.
impl<T: Sized> Container for T {}

/// All containers are direct members of themselves.
///
/// Note, that this prohibits recursive membership.
impl<C: Container> Member<C> for C {
	fn member_on(container: &C) -> &Self {
		container
	}

	fn member_mut_on(container: &mut C) -> &mut Self {
		container
	}
}

/// A phase host
pub trait PhaseHost: Sized {
	fn phase<P: Phase<Self, M>, M: Member<Self>>(&mut self, phase: &mut P) {
		phase.phase_on_host(self);
	}
}

/// All sized types are phase hosts.
impl<T: Sized> PhaseHost for T {}

/// A phase is a type that can be run on a phase host.
pub trait Phase<Host: Sized, M: Member<Host>> {
	/// The product of the pre phase.
	type PreProduct: Sized;

	/// The factor of the post phase (normalized [PreProduct])
	type PostFactor: Sized;

	/// The pre hook on the phase
	fn pre(&mut self, member: &mut M) -> Self::PreProduct;

	/// The update hook on the phase.
	fn update(&mut self, member: &mut M, product: &mut Self::PreProduct);

	/// The normalize hook on the phase.
	fn normalize(&mut self, member: &mut M, product: Self::PreProduct) -> Self::PostFactor;

	/// The post phase.
	fn post(&mut self, member: &mut M, factor: Self::PostFactor);

	/// The full phase.
	fn phase_on_host(&mut self, host: &mut Host) {
		let mut member = M::member_mut_on(host);
		let mut product = self.pre(&mut member);
		self.update(&mut member, &mut product);
		let factor = self.normalize(&mut member, product);
		self.post(&mut member, factor);
	}
}
