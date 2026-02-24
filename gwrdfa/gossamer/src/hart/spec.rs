use crate::{hart::gossamer_messages::GossamerMessages, GossamerMessage, GossamerMessageError};
use crate::{Broadcast, In, InFlight, Out};
use parabyzantine::{
	buffer::{
		query::{QueryPlanlike, Querylike},
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
{
	type Message: GossamerMessage;

	type OutQuery<'a>: Querylike<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		Item = (&'a Out, &'a Self::Message),
	>
	where
		Self::Message: 'a;

	type OutQueryPlan: for<'a> QueryPlanlike<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		&'a <Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
		Query = Self::OutQuery<'a>,
	>;

	type Messages: GossamerMessages<
		Binding,
		Message = Self::Message,
		OutQueryPlan = Self::OutQueryPlan,
	>;
}
