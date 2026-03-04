use crate::{GossamerMessage, Out};
use parabyzantine::{
	buffer::query::{QueryPlanlike, Querylike},
	hart::ParabyzantineData,
};

/// A [GossamerMessages] trait is used to build queries over the Gossamer messages.

pub trait GossamerMessages<Data: ParabyzantineData> {
	type Message: GossamerMessage;

	/// The exact query type produced by the plan.
	type OutQuery<'a>: Querylike<
		Data::MessageEntity,
		(&'a Out, &'a Self::Message),
	>
	where
		Self::Message: 'a;

	type OutQueryPlan: for<'a> QueryPlanlike<
		Data::MessageEntity,
		&'a Data::MessageBuffer,
		(&'a Out, &'a Self::Message),
		Self::OutQuery<'a>,
	>;

	fn gossamer_messages_out_plan(&mut self) -> Self::OutQueryPlan;
}
