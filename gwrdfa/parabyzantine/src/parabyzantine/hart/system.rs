use crate::act::Act;
use crate::hart::ParabyzantineDataBinding;
use crate::Spec;
use core::marker::PhantomData;

/// A [ParabyzantineSystemSpec] is a trait that defines the system of parabyzantine.
pub trait ParabyzantineSystemSpec {
	/// The shared data for the system
	type DataBinding: ParabyzantineDataBinding;

	/// The action to use for agreement in the system.
	type AgreementAction;

	/// The handler of the agreement action.
	///
	/// The handle must be able to apply the AgreementAction to a mutable borrow of the data.
	type AgreementHandler: Act<
		Self::AgreementAction,
		<Self::DataBinding as ParabyzantineDataBinding>::Data,
	>;

	/// The action to use for task in the system.
	type TaskAction;

	/// The handler of the task action.
	///
	/// The handle must be able to apply the TaskAction to a mutable borrow of the data.
	type TaskHandler: Act<Self::TaskAction, <Self::DataBinding as ParabyzantineDataBinding>::Data>;
}

/// A [ParabyzantineSystem] is a trait that defines the system of parabyzantine.
///
/// This follows the same semantic trait pattern as elswhere,
/// where more general traits are used to form higher order requirements,
/// e.g. the use of [Act] in the [ParabyzantineSystemSpec] trait.
pub trait ParabyzantineSystem: Sized {
	type Spec: ParabyzantineSystemSpec;

	/// Get the data for the system.
	fn data(
		&mut self,
	) -> &mut <<Self::Spec as ParabyzantineSystemSpec>::DataBinding as ParabyzantineDataBinding>::Data
	{
		let (_agreement_handler, data) = self.data_and_agreement_pair();
		data
	}

	/// Get the agreement handler for the system.
	fn agreement_handler(
		&mut self,
	) -> &mut <Self::Spec as ParabyzantineSystemSpec>::AgreementHandler {
		let (agreement_handler, _data) = self.data_and_agreement_pair();
		agreement_handler
	}

	/// Gets the data and agreement pair
	fn data_and_agreement_pair(
		&mut self,
	) -> (
		&mut <Self::Spec as ParabyzantineSystemSpec>::AgreementHandler,
		&mut <<Self::Spec as ParabyzantineSystemSpec>::DataBinding as ParabyzantineDataBinding>::Data,
	);

	/// Gets the task handler for the system.
	fn task_handler(&mut self) -> &mut <Self::Spec as ParabyzantineSystemSpec>::TaskHandler {
		let (task_handler, _data) = self.data_and_task_pair();
		task_handler
	}

	/// Gets the data and task pair
	fn data_and_task_pair(
		&mut self,
	) -> (
		&mut <Self::Spec as ParabyzantineSystemSpec>::TaskHandler,
		&mut <<Self::Spec as ParabyzantineSystemSpec>::DataBinding as ParabyzantineDataBinding>::Data,
	);
}

/// A transparent wrapper for parabyzantine system.
///
/// This is the most common way to compose parabyzantine systems.
pub struct Parabyzantine<Spec: ParabyzantineSystemSpec> {
	pub data: <Spec::DataBinding as ParabyzantineDataBinding>::Data,
	pub agreement_handler: Spec::AgreementHandler,
	pub task_handler: Spec::TaskHandler,
}

/// A [Parabyzantine] simply implements direct borrows for the data and the agreement handler.
impl<Spec: ParabyzantineSystemSpec> ParabyzantineSystem for Parabyzantine<Spec> {
	type Spec = Spec;

	fn data_and_agreement_pair(
		&mut self,
	) -> (
		&mut <Self::Spec as ParabyzantineSystemSpec>::AgreementHandler,
		&mut <<Self::Spec as ParabyzantineSystemSpec>::DataBinding as ParabyzantineDataBinding>::Data,
	){
		(&mut self.agreement_handler, &mut self.data)
	}

	fn data_and_task_pair(
		&mut self,
	) -> (
		&mut <Self::Spec as ParabyzantineSystemSpec>::TaskHandler,
		&mut <<Self::Spec as ParabyzantineSystemSpec>::DataBinding as ParabyzantineDataBinding>::Data,
	){
		(&mut self.task_handler, &mut self.data)
	}
}

impl<Spec: ParabyzantineSystemSpec> Parabyzantine<Spec> {
	/// Direct call to invoke the agreement handler.
	pub fn update_agreement(&mut self, action: Spec::AgreementAction) {
		let (agreement_handler, data) = self.data_and_agreement_pair();
		agreement_handler.act(action, data);
	}

	/// Direct call to invoke the task handler.
	pub fn update_task(&mut self, action: Spec::TaskAction) {
		let (task_handler, data) = self.data_and_task_pair();
		task_handler.act(action, data);
	}
}

/// A marker struct for the agreement action.
#[derive(Debug, Clone, Copy)]
pub struct AgreementAction<A> {
	phantom: PhantomData<A>,
}

impl<A> AgreementAction<A> {
	pub fn new(_action: A) -> Self {
		Self { phantom: PhantomData }
	}
}

/// A marker struct for the data binding.
#[derive(Debug, Clone, Copy)]
pub struct DataBinding<B> {
	phantom: PhantomData<B>,
}

impl<B> DataBinding<B> {
	pub fn new(_binding: B) -> Self {
		Self { phantom: PhantomData }
	}
}

/// A marker struct for the agreement handler.
#[derive(Debug, Clone, Copy)]
pub struct AgreementHandler<H> {
	phantom: PhantomData<H>,
}

impl<H> AgreementHandler<H> {
	pub fn new(_handler: H) -> Self {
		Self { phantom: PhantomData }
	}
}

/// A marker struct for the task action.
#[derive(Debug, Clone, Copy)]
pub struct TaskAction<T> {
	phantom: PhantomData<T>,
}

impl<T> TaskAction<T> {
	pub fn new(_action: T) -> Self {
		Self { phantom: PhantomData }
	}
}

/// A marker struct for the task handler.
#[derive(Debug, Clone, Copy)]
pub struct TaskHandler<H> {
	phantom: PhantomData<H>,
}

impl<H> TaskHandler<H> {
	pub fn new(_handler: H) -> Self {
		Self { phantom: PhantomData }
	}
}

/// A Spec on ParabyzantineHartSystem is a ParabyzantineSystemSpec.
impl<D: ParabyzantineDataBinding, A, AH: Act<A, D::Data>, T, TH: Act<T, D::Data>>
	ParabyzantineSystemSpec
	for Spec<(
		DataBinding<D>,
		AgreementAction<A>,
		AgreementHandler<AH>,
		TaskAction<T>,
		TaskHandler<TH>,
	)>
{
	type DataBinding = D;
	type AgreementAction = A;
	type AgreementHandler = AH;
	type TaskAction = T;
	type TaskHandler = TH;
}
