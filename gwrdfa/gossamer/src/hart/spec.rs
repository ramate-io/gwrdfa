use crate::{hart::gossamer_messages::GossamerMessages, GossamerMessage, GossamerMessageError};
use crate::{Broadcast, In, InFlight, Out};
use parabyzantine::{
	buffer::{
		query::{QueryPlanlike, Querylike},
		Bundle,
	},
	hart::{ParabyzantineDataBinding, ParabyzantineDataSpec},
};

pub trait GossamerSpec<Binding: ParabyzantineDataBinding>
where
	GossamerMessageError: Bundle<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
	>,
	(In, Self::Message): Bundle<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
	>,
	Out: Bundle<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
	>,
	InFlight: Bundle<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
	>,
	Broadcast: Bundle<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
	>,
{
	/// The type of the message.
	type Message: GossamerMessage;

	/// The type of the query for the message.
	type MessageOutQuery: for<'a> Querylike<
		'a,
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
		Item = (&'a Out, &'a Self::Message),
	>;

	/// The type of the query plan for the message.
	type MessageOutQueryPlan: for<'a> QueryPlanlike<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
		Query<'a> = Self::MessageOutQuery,
	>;

	/// The type of the message builder.
	type Messages: for<'a> GossamerMessages<
		'a,
		Self::Message,
		Binding,
		Self::MessageOutQuery,
		Self::MessageOutQueryPlan,
	>;
}
