use crate::{GossamerMessage, Out};
use parabyzantine::{
	buffer::query::{QueryPlanlike, Querylike},
	hart::ParabyzantineData,
};

/// Describes how a Hart implementation finds outbound messages to publish.
///
/// Implementors provide a query plan over Parabyzantine message buffers that
/// yields `(Out, Message)` tuples. `GossamerHart` uses that plan each update
/// cycle to locate messages that should be sent via `Gossamer`.
pub trait GossamerMessages<Data: ParabyzantineData> {
	/// Message payload type routed through `Gossamer`.
	type Message: GossamerMessage;

	/// Concrete query type produced by [`Self::OutQueryPlan`].
	type OutQuery<'a>: Querylike<
		Data::MessageEntity,
		(&'a Out, &'a Self::Message),
	>
	where
		Self::Message: 'a;

	/// Query plan that discovers entities marked as outbound.
	type OutQueryPlan: for<'a> QueryPlanlike<
		Data::MessageEntity,
		&'a Data::MessageBuffer,
		(&'a Out, &'a Self::Message),
		Self::OutQuery<'a>,
	>;

	/// Build the outbound query plan consumed by `GossamerHart`.
	fn gossamer_messages_out_plan(&mut self) -> Self::OutQueryPlan;
}
