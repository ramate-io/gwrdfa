/// A schedule host
pub trait ScheduleData: Sized {
	fn schedule<P: Schedule<Self>>(&mut self, schedule: &mut P) {
		schedule.schedule_on_host(self);
	}
}

/// All sized types are schedule hosts.
impl<T: Sized> ScheduleData for T {}

/// A schedule is a type that can be run on a schedule host.
pub trait Schedule<Data: Sized> {
	/// The product of the pre schedule.
	type PreProduct: Sized;

	/// The factor of the post schedule (normalized [PreProduct])
	type PostFactor: Sized;

	/// The pre hook on the schedule
	fn pre(&mut self, schedule_member: &mut Data) -> Self::PreProduct;

	/// The update hook on the schedule.
	fn update(&mut self, schedule_member: &mut Data, product: &mut Self::PreProduct);

	/// The normalize hook on the schedule.
	fn normalize(
		&mut self,
		schedule_member: &mut Data,
		product: Self::PreProduct,
	) -> Self::PostFactor;

	/// The post schedule.
	fn post(&mut self, schedule_member: &mut Data, factor: Self::PostFactor);

	/// The full schedule.
	fn schedule_on_host(&mut self, data: &mut Data) {
		let mut product = self.pre(data);
		self.update(data, &mut product);
		let factor = self.normalize(data, product);
		self.post(data, factor);
	}
}

/// A container is a type that can contain schedulemembers.
pub trait ScheduleContainer<Member>: Sized {
	fn schedule_member(&self) -> &Member;

	fn schedule_member_mut(&mut self) -> &mut Member;
}

/// A schedulemember is a type that can be contained in a container.
pub trait ScheduleMember<Container: Sized> {
	fn schedule_member_on(container: &Container) -> &Self;

	fn schedule_member_mut_on(container: &mut Container) -> &mut Self;
}

/// All sized types are containers.
impl<T: Sized, M: ScheduleMember<T>> ScheduleContainer<M> for T {
	fn schedule_member(&self) -> &M {
		M::schedule_member_on(self)
	}

	fn schedule_member_mut(&mut self) -> &mut M {
		M::schedule_member_mut_on(self)
	}
}

/// All containers are direct schedulemembers of themselves.
///
/// Note, that this prohibits recursive schedulemembership.
impl<C> ScheduleMember<C> for C {
	fn schedule_member_on(container: &C) -> &Self {
		container
	}

	fn schedule_member_mut_on(container: &mut C) -> &mut Self {
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

	/// Runs a schedule if membership is satisfied.
	pub fn schedule<M: ScheduleMember<Container>, P: Schedule<M>>(&mut self, schedule: &mut P) {
		let mut member = self.container.schedule_member_mut();
		schedule.schedule_on_host(&mut member);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	pub struct TestSchedule {
		left: u8,
		right: u8,
	}

	pub struct TestMember {
		state: u8,
	}

	pub struct TestContainer {
		member: TestMember,
	}

	impl ScheduleMember<TestContainer> for TestMember {
		fn schedule_member_on(container: &TestContainer) -> &Self {
			&container.member
		}

		fn schedule_member_mut_on(container: &mut TestContainer) -> &mut Self {
			&mut container.member
		}
	}

	impl Schedule<TestMember> for TestSchedule {
		type PreProduct = u8;
		type PostFactor = u8;

		fn pre(&mut self, _schedule_member: &mut TestMember) -> Self::PreProduct {
			self.left + self.left
		}

		fn update(&mut self, _schedule_member: &mut TestMember, product: &mut Self::PreProduct) {
			self.right = *product;
		}

		fn normalize(
			&mut self,
			_schedule_member: &mut TestMember,
			product: Self::PreProduct,
		) -> Self::PostFactor {
			product
		}

		fn post(&mut self, schedule_member: &mut TestMember, factor: Self::PostFactor) {
			schedule_member.state = factor;
		}
	}

	#[test]
	fn test_schedule_host() {
		let mut scheduler = Scheduler::new(TestMember { state: 0 });
		scheduler.schedule(&mut TestSchedule { left: 1, right: 1 });
		assert_eq!(scheduler.container.state, 2);
	}

	#[test]
	fn test_schedule_member() {
		let mut scheduler = Scheduler::new(TestContainer { member: TestMember { state: 0 } });
		scheduler.schedule(&mut TestSchedule { left: 1, right: 1 });
		assert_eq!(scheduler.container.member.state, 2);
	}
}
