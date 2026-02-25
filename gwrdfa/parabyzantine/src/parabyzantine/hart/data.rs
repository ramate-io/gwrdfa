pub mod as_agreement;
pub mod as_message_in;
pub mod as_message_out;
pub mod as_task;

use crate::act::Act;
use crate::buffer::{facts::Facts, Bufferlike};
use crate::{NoOp, NoOpData};

/// The [Hart] marker is used to indicate an act on the entirety of the parabyzantine system.
///
/// You'll note that the canonical naming for systems on the [Hart] is simply to omit
/// the term "Hart". Hennce it is [ParabyzantineData] rather than [ParabyzantineHartData].
#[derive(Debug, Clone, Copy)]
pub struct Hart;

/// A [ParabyzantineDataSpec] is a specification for the parabyzantine protocol.
pub trait ParabyzantineDataSpec: Sized {
	/// The entity type for the certificate.
	type CertificateEntity: Sized;
	/// The buffer type for the certificate.
	type CertificateBuffer: Bufferlike<Self::CertificateEntity>;

	/// The entity type for the agreement.
	type AgreementEntity: Sized;
	/// The buffer type for the agreement.
	type AgreementBuffer: Bufferlike<Self::AgreementEntity>;

	/// The entity type for the transaction.
	type TransactionEntity: Sized;
	/// The buffer type for the transaction.
	type TransactionBuffer: Bufferlike<Self::TransactionEntity>;

	/// The entity type for the task.
	type TaskEntity: Sized;
	/// The buffer type for the task.
	type TaskBuffer: Bufferlike<Self::TaskEntity>;

	/// The entity type for the message.
	type MessageEntity: Sized;
	/// The buffer type for the message.
	type MessageBuffer: Bufferlike<Self::MessageEntity>;
}

pub trait ParabyzantineData<Spec: ParabyzantineDataSpec>: Sized {
	/// The world of the parabyzantine.
	fn parabyzantine_world<'a>(&'a mut self) -> ParabyzantineWorld<'a, Spec>;
}

pub struct ParabyzantineWorld<'a, Spec: ParabyzantineDataSpec> {
	/// The facts for the certificate.
	pub certificate_facts: Facts<'a, Spec::CertificateEntity, Spec::CertificateBuffer>,

	/// The facts for the agreement.
	pub agreement_facts: Facts<'a, Spec::AgreementEntity, Spec::AgreementBuffer>,

	/// The facts for the transaction.
	pub transaction_facts: Facts<'a, Spec::TransactionEntity, Spec::TransactionBuffer>,

	/// The facts for the task.
	pub task_facts: Facts<'a, Spec::TaskEntity, Spec::TaskBuffer>,

	/// The facts for the message.
	pub message_facts: Facts<'a, Spec::MessageEntity, Spec::MessageBuffer>,
}

/// A [ParabyzantineHart] trait describes operations on the parabyzantine hart.
pub trait ParabyzantineHart: Sized {
	type Binding: ParabyzantineDataBinding;

	/// Borrows the [ParabyzantineWorld] for the parabyzantine hart.
	fn parabyzantine_hart_world<'a>(
		&mut self,
		data: &'a mut <Self::Binding as ParabyzantineDataBinding>::Data,
	) -> ParabyzantineWorld<'a, <Self::Binding as ParabyzantineDataBinding>::Spec> {
		data.parabyzantine_world()
	}

	/// Compute the parabyzantine hart.
	fn update_parabyzantine_hart(
		&mut self,
		data: ParabyzantineWorld<<Self::Binding as ParabyzantineDataBinding>::Spec>,
	);

	fn act_on_parabyzantine_hart(
		&mut self,
		data: &mut <Self::Binding as ParabyzantineDataBinding>::Data,
	) {
		let world = self.parabyzantine_hart_world(data);
		self.update_parabyzantine_hart(world);
	}
}

impl<Binding: ParabyzantineDataBinding, HartHandler: ParabyzantineHart<Binding = Binding>>
	Act<Hart, Binding::Data> for HartHandler
{
	fn act(&mut self, _action: Hart, data: &mut Binding::Data) {
		self.act_on_parabyzantine_hart(data);
	}
}

/// A [ParabyzantineDataBinding] is a binding for the [Parabyzantine] protocol.
///
/// It binds between the [ParabyzantineDataSpec] and the [ParabyzantineData].
pub trait ParabyzantineDataBinding {
	type Spec: ParabyzantineDataSpec;
	type Data: ParabyzantineData<Self::Spec>;
}

/// A [ParabyzantineDataSpec] for the [NoOp] struct.
impl ParabyzantineDataSpec for NoOp {
	type CertificateEntity = NoOp;
	type CertificateBuffer = NoOp;
	type AgreementEntity = NoOp;
	type AgreementBuffer = NoOp;
	type TransactionEntity = NoOp;
	type TransactionBuffer = NoOp;
	type TaskEntity = NoOp;
	type TaskBuffer = NoOp;
	type MessageEntity = NoOp;
	type MessageBuffer = NoOp;
}

/// A [ParabyzantineData] for the [NoOpData] struct.
impl ParabyzantineData<NoOp> for NoOpData {
	fn parabyzantine_world<'a>(&'a mut self) -> ParabyzantineWorld<'a, NoOp> {
		ParabyzantineWorld {
			certificate_facts: (&mut self.no_op_0).into(),
			agreement_facts: (&mut self.no_op_1).into(),
			transaction_facts: (&mut self.no_op_2).into(),
			task_facts: (&mut self.no_op_3).into(),
			message_facts: (&mut self.no_op_4).into(),
		}
	}
}

impl ParabyzantineDataBinding for NoOp {
	type Spec = NoOp;
	type Data = NoOpData;
}
