/// A phase host
pub trait PhaseData: Sized {
	fn phase<P: Phase<Self>>(&mut self, phase: &mut P) {
		phase.phase_on_host(self);
	}
}

/// All sized types are phase hosts.
impl<T: Sized> PhaseData for T {}

/// A phase is a type that can be run on a phase host.
pub trait Phase<Data: Sized> {
	/// The product of the pre phase.
	type PreProduct: Sized;

	/// The factor of the post phase (normalized [PreProduct])
	type PostFactor: Sized;

	/// The pre hook on the phase
	fn pre(&mut self, phase_member: &mut Data) -> Self::PreProduct;

	/// The update hook on the phase.
	fn update(&mut self, phase_member: &mut Data, product: &mut Self::PreProduct);

	/// The normalize hook on the phase.
	fn normalize(&mut self, phase_member: &mut Data, product: Self::PreProduct)
		-> Self::PostFactor;

	/// The post phase.
	fn post(&mut self, phase_member: &mut Data, factor: Self::PostFactor);

	/// The full phase.
	fn phase_on_host(&mut self, data: &mut Data) {
		let mut product = self.pre(data);
		self.update(data, &mut product);
		let factor = self.normalize(data, product);
		self.post(data, factor);
	}
}

/// A container is a type that can contain phasemembers.
pub trait PhaseContainer<Member>: Sized {
	fn phase_member(&self) -> &Member;

	fn phase_member_mut(&mut self) -> &mut Member;
}

/// A phasemember is a type that can be contained in a container.
pub trait PhaseMember<Container: Sized> {
	fn phase_member_on(container: &Container) -> &Self;

	fn phase_member_mut_on(container: &mut Container) -> &mut Self;
}

/// All sized types are containers.
impl<T: Sized, M: PhaseMember<T>> PhaseContainer<M> for T {
	fn phase_member(&self) -> &M {
		M::phase_member_on(self)
	}

	fn phase_member_mut(&mut self) -> &mut M {
		M::phase_member_mut_on(self)
	}
}

/// All containers are direct phasemembers of themselves.
///
/// Note, that this prohibits recursive phasemembership.
impl<C> PhaseMember<C> for C {
	fn phase_member_on(container: &C) -> &Self {
		container
	}

	fn phase_member_mut_on(container: &mut C) -> &mut Self {
		container
	}
}

pub struct Scheduler<Container: Sized> {
	container: Container,
}

impl<Container: Sized> Scheduler<Container> {
	pub fn new(container: Container) -> Self {
		Self { container }
	}

	/// Runs a phase if membership is satisfied.
	pub fn phase<M: PhaseMember<Container>, P: Phase<M>>(&mut self, phase: &mut P) {
		let mut member = self.container.phase_member_mut();
		phase.phase_on_host(&mut member);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	pub struct TestPhase {
		left: u8,
		right: u8,
	}

	pub struct TestMember {
		state: u8,
	}

	pub struct TestContainer {
		member: TestMember,
	}

	impl PhaseMember<TestContainer> for TestMember {
		fn phase_member_on(container: &TestContainer) -> &Self {
			&container.member
		}

		fn phase_member_mut_on(container: &mut TestContainer) -> &mut Self {
			&mut container.member
		}
	}

	impl Phase<TestMember> for TestPhase {
		type PreProduct = u8;
		type PostFactor = u8;

		fn pre(&mut self, _phase_member: &mut TestMember) -> Self::PreProduct {
			self.left + self.left
		}

		fn update(&mut self, _phase_member: &mut TestMember, product: &mut Self::PreProduct) {
			self.right = *product;
		}

		fn normalize(
			&mut self,
			_phase_member: &mut TestMember,
			product: Self::PreProduct,
		) -> Self::PostFactor {
			product
		}

		fn post(&mut self, phase_member: &mut TestMember, factor: Self::PostFactor) {
			phase_member.state = factor;
		}
	}

	#[test]
	fn test_phase_host() {
		let mut scheduler = Scheduler::new(TestMember { state: 0 });
		scheduler.phase(&mut TestPhase { left: 1, right: 1 });
		assert_eq!(scheduler.container.state, 2);
	}

	#[test]
	fn test_phase_member() {
		let mut scheduler = Scheduler::new(TestContainer { member: TestMember { state: 0 } });
		scheduler.phase(&mut TestPhase { left: 1, right: 1 });
		assert_eq!(scheduler.container.member.state, 2);
	}
}
