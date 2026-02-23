use crate::{GossamerMessage, Out};
use parabyzantine::{
	buffer::query::Querylike,
	hart::{ParabyzantineDataBinding, ParabyzantineDataSpec},
};

/// A [GossamerMessages] trait is used to build queries over the Gossamer messages.

pub trait GossamerMessages {
	type Message: GossamerMessage;
	type Binding: ParabyzantineDataBinding;

	type OutPlan;

	/// The exact query type produced by the plan.
	type OutQuery<'a>: Querylike<
		<<Self::Binding as ParabyzantineDataBinding>::Spec as ParabyzantineDataSpec>::MessageEntity,
		Item = (&'a Out, &'a Self::Message),
	>
	where
		Self::Message: 'a;

	fn gossamer_messages_out_plan(&mut self) -> Self::OutPlan;
}
