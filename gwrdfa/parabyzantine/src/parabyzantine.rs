pub mod agreement;
pub mod broadcast_in;
pub mod broadcast_out;
pub mod hart;
pub mod task;

use crate::act::{Invoke, Pair};
use crate::agreement::ParabyzantineAgreementDataBinding;
pub use hart::*;

/// A Parabyzantine system automatically implements the [Pair] trait for the agreement action.
impl<Spec: ParabyzantineSystemSpec, System: ParabyzantineSystem<Spec = Spec>>
	Pair<Spec::AgreementAction> for System
{
	type Left = Spec::AgreementHandler;
	type Right = <Spec::AgreementDataBinding as ParabyzantineAgreementDataBinding>::Data;

	fn pair(&mut self) -> (&mut Self::Left, &mut Self::Right) {
		self.data_and_agreement_pair()
	}
}
/// A transparent wrapper for parabyzantine system.
///
/// This is the most common way to compose parabyzantine systems.
pub struct Parabyzantine<Spec: ParabyzantineSystemSpec> {
	pub data: <Spec::AgreementDataBinding as ParabyzantineAgreementDataBinding>::Data,
	pub agreement_handler: Spec::AgreementHandler,
}

/// A [Parabyzantine] simply implements direct borrows for the data and the agreement handler.
impl<Spec: ParabyzantineSystemSpec> ParabyzantineSystem for Parabyzantine<Spec> {
	type Spec = Spec;

	fn data_and_agreement_pair(
		&mut self,
	) -> (
		&mut <Self::Spec as ParabyzantineSystemSpec>::AgreementHandler,
		&mut <<Self::Spec as ParabyzantineSystemSpec>::AgreementDataBinding as ParabyzantineAgreementDataBinding>::Data,
	){
		(&mut self.agreement_handler, &mut self.data)
	}
}

impl<Spec: ParabyzantineSystemSpec> Parabyzantine<Spec> {
	/// Allows invoking without adding the trait.
	///
	/// We use the term update because it is more specific
	/// to what the ParabyzantineSystem is doing when invoking an action.
	///
	/// It is updating some number of worlds in the system.
	pub fn update<A>(&mut self, action: A)
	where
		Self: Invoke<A>,
	{
		<Self as Invoke<A>>::invoke(self, action);
	}
}
