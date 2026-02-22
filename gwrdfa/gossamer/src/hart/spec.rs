use crate::{hart::gossamer_messages::GossamerMessages, GossamerMessage, GossamerMessageError};
use crate::{Broadcast, In, InFlight, Out};
use parabyzantine::{
	buffer::{
		query::{IntoQuery, Querylike},
		Stores,
	},
	hart::{ParabyzantineDataBinding, ParabyzantineDataSpec},
};

pub trait GossamerSpec<'a, Binding: ParabyzantineDataBinding + 'a>
where
	<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer: Stores<GossamerMessageError, <Binding::Spec as ParabyzantineDataSpec>::MessageEntity>
		+ Stores<Self::Message, <Binding::Spec as ParabyzantineDataSpec>::MessageEntity>
		+ Stores<(In, Self::Message), <Binding::Spec as ParabyzantineDataSpec>::MessageEntity>
		+ Stores<Out, <Binding::Spec as ParabyzantineDataSpec>::MessageEntity>
		+ Stores<InFlight, <Binding::Spec as ParabyzantineDataSpec>::MessageEntity>
		+ Stores<Broadcast, <Binding::Spec as ParabyzantineDataSpec>::MessageEntity>,
	&'a <Binding::Spec as ParabyzantineDataSpec>::MessageBuffer: IntoQuery<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		Self::MessageOutQueryPlan,
		Query = Self::MessageOutQuery,
	>,
{
	/// The type of the message.
	type Message: GossamerMessage + 'a;

	/// The type of the query for the message.
	type MessageOutQuery: Querylike<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		Item = (&'a Out, &'a Self::Message),
	>;

	/// The type of the query plan for the message.
	type MessageOutQueryPlan;

	/// The type of the message builder.
	type Messages: GossamerMessages<
		'a,
		Self::Message,
		Binding,
		Self::MessageOutQuery,
		Self::MessageOutQueryPlan,
	>;
}
