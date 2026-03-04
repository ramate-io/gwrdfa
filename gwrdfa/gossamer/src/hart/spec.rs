use crate::Out;
use crate::{
	hart::gossamer_messages::GossamerMessages, hart::gossamer_storage::GossamerMessageStorage,
	GossamerMessage,
};
use parabyzantine::buffer::query::{QueryPlanlike, Querylike};
use parabyzantine::hart::ParabyzantineData;

pub trait GossamerSpec<Data: ParabyzantineData>
where
	Data::MessageDraftBuffer: GossamerMessageStorage<Data::MessageEntity, Self::Message>,
{
	type Message: GossamerMessage;

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

	type Messages: GossamerMessages<
		Data,
		Message = Self::Message,
		OutQueryPlan = Self::OutQueryPlan,
	>;
}
