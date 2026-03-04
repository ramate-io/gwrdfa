use crate::{GossamerMessage, Out};
use parabyzantine::{
	buffer::query::{QueryPlanlike, Querylike},
	hart::ParabyzantineDataBinding,
};

/// A [GossamerMessages] trait is used to build queries over the Gossamer messages.

pub trait GossamerMessages<Binding: ParabyzantineDataBinding> {
	type Message: GossamerMessage;

	/// The exact query type produced by the plan.
	type OutQuery<'a>: Querylike<
		<<Binding as ParabyzantineDataBinding>::Spec as ParabyzantineDataSpec>::MessageEntity,
		(&'a Out, &'a Self::Message),
	>
	where
		Self::Message: 'a;

	type OutQueryPlan: for<'a> QueryPlanlike<
		<<Binding as ParabyzantineDataBinding>::Spec as ParabyzantineDataSpec>::MessageEntity,
		&'a <<Binding as ParabyzantineDataBinding>::Spec as ParabyzantineDataSpec>::MessageBuffer,
		(&'a Out, &'a Self::Message),
		Self::OutQuery<'a>,
	>;

	fn gossamer_messages_out_plan(&mut self) -> Self::OutQueryPlan;
}
