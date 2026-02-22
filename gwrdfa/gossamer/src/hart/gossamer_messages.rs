use crate::{GossamerMessage, Out};
use parabyzantine::{
	buffer::query::{IntoQuery, Querylike},
	hart::{ParabyzantineDataBinding, ParabyzantineDataSpec},
};

/// A [GossamerMessages] trait is used to build queries over the Gossamer messages.
pub trait GossamerMessages<
	'a,
	Message: GossamerMessage + 'a,
	Binding: ParabyzantineDataBinding + 'a,
	OutQuery: Querylike<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		Item = (&'a Out, &'a Message),
	>,
	OutQueryPlan,
> where
	&'a <Binding::Spec as ParabyzantineDataSpec>::MessageBuffer: IntoQuery<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		OutQueryPlan,
		Query = OutQuery,
	>,
{
	fn gossamer_messages_out_plan(&mut self) -> OutQueryPlan;
}
