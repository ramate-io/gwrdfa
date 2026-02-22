use crate::{hart::gossamer_messages::GossamerMessages, GossamerMessage, GossamerMessageError};
use crate::{Broadcast, In, InFlight, Out};
use parabyzantine::{
	buffer::{
		query::{IntoQuery, Querylike},
		Stores,
	},
	hart::{ParabyzantineDataBinding, ParabyzantineDataSpec},
};

pub trait GossamerSpec<Binding: ParabyzantineDataBinding>
where
	<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer: Stores<GossamerMessageError, <Binding::Spec as ParabyzantineDataSpec>::MessageEntity>
		+ Stores<Self::Message, <Binding::Spec as ParabyzantineDataSpec>::MessageEntity>
		+ Stores<(In, Self::Message), <Binding::Spec as ParabyzantineDataSpec>::MessageEntity>
		+ Stores<Out, <Binding::Spec as ParabyzantineDataSpec>::MessageEntity>
		+ Stores<InFlight, <Binding::Spec as ParabyzantineDataSpec>::MessageEntity>
		+ Stores<Broadcast, <Binding::Spec as ParabyzantineDataSpec>::MessageEntity>,
	for<'a> &'a <Binding::Spec as ParabyzantineDataSpec>::MessageBuffer: IntoQuery<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		Self::OutQueryPlan,
		Query = Self::OutQuery<'a>,
	>,
	Self::Messages:
		GossamerMessages<Binding = Binding, Message = Self::Message, OutPlan = Self::OutQueryPlan>,
{
	type Message: GossamerMessage;
	type OutQueryPlan;

	type OutQuery<'a>: Querylike<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		Item = (&'a Out, &'a Self::Message),
	>
	where
		Self::Message: 'a;

	type Messages;
}
