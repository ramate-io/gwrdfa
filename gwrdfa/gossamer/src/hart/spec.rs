use crate::Out;
use crate::{
	hart::gossamer_messages::GossamerMessages, hart::gossamer_storage::GossamerMessageStorage,
	GossamerMessage,
};
use parabyzantine::buffer::query::{QueryPlanlike, Querylike};
use parabyzantine::hart::{ParabyzantineDataBinding, ParabyzantineDataSpec};

pub trait GossamerSpec<Binding: ParabyzantineDataBinding>
where
	<Binding::Spec as ParabyzantineDataSpec>::MessageDraftBuffer: GossamerMessageStorage<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		Self::Message,
	>,
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
