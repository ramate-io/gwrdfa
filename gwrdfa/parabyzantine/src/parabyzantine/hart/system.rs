use crate::act::{Act, Invoke, Pair};
use crate::agreement::ParabyzantineAgreementDataBinding;
use core::marker::PhantomData;

/// A [ParabyzantineSystemSpec] is a trait that defines the system of parabyzantine.
pub trait ParabyzantineSystemSpec {
	/// The action to use for agreement in the system.
	type AgreementAction;

	/// The bindging between the data and the dataspec
	type AgreementDataBinding: ParabyzantineAgreementDataBinding;

	/// The handler of the agreement action.
	///
	/// The handle must be able to apply the AgreementAction to a mutable borrow of the data.
	type AgreementHandler: Act<
		Self::AgreementAction,
		<Self::AgreementDataBinding as ParabyzantineAgreementDataBinding>::Data,
	>;
}

/// A [Spec] is a specification for a parabyzantine system.
#[derive(Debug, Clone, Copy)]
pub struct SystemSpec<
	AgreementAction,
	AgreementDataBinding: ParabyzantineAgreementDataBinding,
	AgreementHandler: Act<AgreementAction, AgreementDataBinding::Data>,
> {
	agreement_data_binding: PhantomData<AgreementDataBinding>,
	agreement_action: PhantomData<AgreementAction>,
	agreement_handler: PhantomData<AgreementHandler>,
}

impl<
		AgreementAction,
		AgreementDataBinding: ParabyzantineAgreementDataBinding,
		AgreementHandler: Act<AgreementAction, AgreementDataBinding::Data>,
	> SystemSpec<AgreementAction, AgreementDataBinding, AgreementHandler>
{
	pub fn new() -> Self {
		Self {
			agreement_data_binding: PhantomData,
			agreement_action: PhantomData,
			agreement_handler: PhantomData,
		}
	}
}

impl<
		AgreementDataBinding: ParabyzantineAgreementDataBinding,
		AgreementAction,
		AgreementHandler: Act<AgreementAction, AgreementDataBinding::Data>,
	> ParabyzantineSystemSpec for SystemSpec<AgreementAction, AgreementDataBinding, AgreementHandler>
{
	type AgreementDataBinding = AgreementDataBinding;
	type AgreementAction = AgreementAction;
	type AgreementHandler = AgreementHandler;
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
	) -> &mut <<Self::Spec as ParabyzantineSystemSpec>::AgreementDataBinding as ParabyzantineAgreementDataBinding>::Data
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
		&mut <<Self::Spec as ParabyzantineSystemSpec>::AgreementDataBinding as ParabyzantineAgreementDataBinding>::Data,
	);
}
