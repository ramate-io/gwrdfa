use crate::{hart::gossamer_messages::GossamerMessages, GossamerMessage, GossamerMessageError};
use parabyzantine::{
	buffer::{Bundle, QueryPlanlike, Querylike},
	hart::{ParabyzantineDataBinding, ParabyzantineDataSpec},
};

pub trait GossamerSpec<Binding: ParabyzantineDataBinding>
where
	GossamerMessageError: Bundle<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
	>,
{
	/// The type of the message.
	type Message: GossamerMessage
		+ Bundle<
			<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
			<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
		>;

	/// The type of the query for the message.
	type MessageQuery: Querylike<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
		Self::Message,
	>;

	/// The type of the query plan for the message.
	type MessageQueryPlan: for<'a> QueryPlanlike<
		'a,
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
		Self::Message,
		Self::MessageQuery,
	>;

	/// The type of the message builder.
	type Messages: GossamerMessages<
		Self::Message,
		Binding,
		Self::MessageQuery,
		Self::MessageQueryPlan,
	>;
}
