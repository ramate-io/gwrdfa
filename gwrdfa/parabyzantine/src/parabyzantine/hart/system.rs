use crate::act::Act;
use crate::hart::ParabyzantineData;
use core::marker::PhantomData;

/// A [ParabyzantineSystem] is a trait that defines the system of parabyzantine.
///
/// This follows the same semantic trait pattern as elsewhere, where [Act] bounds
/// define handler capabilities on the shared data.
pub trait ParabyzantineSystem: Sized {
	type Data: ParabyzantineData;
	type AgreementAction;
	type AgreementHandler: Act<Self::AgreementAction, Self::Data>;
	type TaskAction;
	type TaskHandler: Act<Self::TaskAction, Self::Data>;

	/// Get the data for the system.
	fn data(&mut self) -> &mut Self::Data {
		let (_agreement_handler, data) = self.data_and_agreement_pair();
		data
	}

	/// Get the agreement handler for the system.
	fn agreement_handler(&mut self) -> &mut Self::AgreementHandler {
		let (agreement_handler, _data) = self.data_and_agreement_pair();
		agreement_handler
	}

	/// Gets the data and agreement pair
	fn data_and_agreement_pair(&mut self) -> (&mut Self::AgreementHandler, &mut Self::Data);

	/// Gets the task handler for the system.
	fn task_handler(&mut self) -> &mut Self::TaskHandler {
		let (task_handler, _data) = self.data_and_task_pair();
		task_handler
	}

	/// Gets the data and task pair
	fn data_and_task_pair(&mut self) -> (&mut Self::TaskHandler, &mut Self::Data);
}

/// A transparent wrapper for parabyzantine system.
///
/// This is the most common way to compose parabyzantine systems.
pub struct Parabyzantine<Data, AgreementAction, AgreementHandler, TaskAction, TaskHandler>
where
	Data: ParabyzantineData,
	AgreementHandler: Act<AgreementAction, Data>,
	TaskHandler: Act<TaskAction, Data>,
{
	pub data: Data,
	pub agreement_handler: AgreementHandler,
	pub task_handler: TaskHandler,
	_agreement_action: PhantomData<AgreementAction>,
	_task_action: PhantomData<TaskAction>,
}

/// A [Parabyzantine] simply implements direct borrows for the data and the agreement handler.
impl<Data, AgreementAction, AgreementHandler, TaskAction, TaskHandler> ParabyzantineSystem
	for Parabyzantine<Data, AgreementAction, AgreementHandler, TaskAction, TaskHandler>
where
	Data: ParabyzantineData,
	AgreementHandler: Act<AgreementAction, Data>,
	TaskHandler: Act<TaskAction, Data>,
{
	type Data = Data;
	type AgreementAction = AgreementAction;
	type AgreementHandler = AgreementHandler;
	type TaskAction = TaskAction;
	type TaskHandler = TaskHandler;

	fn data_and_agreement_pair(&mut self) -> (&mut Self::AgreementHandler, &mut Self::Data) {
		(&mut self.agreement_handler, &mut self.data)
	}

	fn data_and_task_pair(&mut self) -> (&mut Self::TaskHandler, &mut Self::Data) {
		(&mut self.task_handler, &mut self.data)
	}
}

impl<Data, AgreementAction, AgreementHandler, TaskAction, TaskHandler>
	Parabyzantine<Data, AgreementAction, AgreementHandler, TaskAction, TaskHandler>
where
	Data: ParabyzantineData,
	AgreementHandler: Act<AgreementAction, Data>,
	TaskHandler: Act<TaskAction, Data>,
{
	pub fn new(data: Data, agreement_handler: AgreementHandler, task_handler: TaskHandler) -> Self {
		Self {
			data,
			agreement_handler,
			task_handler,
			_agreement_action: PhantomData,
			_task_action: PhantomData,
		}
	}

	/// Direct call to invoke the agreement handler.
	pub fn update_agreement(&mut self, action: AgreementAction) {
		let (agreement_handler, data) = self.data_and_agreement_pair();
		agreement_handler.act(action, data);
	}

	/// Direct call to invoke the task handler.
	pub fn update_task(&mut self, action: TaskAction) {
		let (task_handler, data) = self.data_and_task_pair();
		task_handler.act(action, data);
	}
}
