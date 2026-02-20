use crate::{GossamerMessage, Out};
use parabyzantine::{
	buffer::query::{QueryPlanlike, Querylike},
	hart::{ParabyzantineDataBinding, ParabyzantineDataSpec},
};

/// A [GossamerMessages] trait is used to build queries over the Gossamer messages.
pub trait GossamerMessages<
	'a,
	Message: GossamerMessage + 'a,
	Binding: ParabyzantineDataBinding + 'a,
	OutQuery: Querylike<
			'a,
			<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
			<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
			Item = (&'a Out, &'a Message),
		> + 'a,
	OutQueryPlan: QueryPlanlike<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
		Query<'a> = OutQuery,
	>,
>
{
	fn gossamer_messages_out_plan(&mut self) -> OutQueryPlan;
}
