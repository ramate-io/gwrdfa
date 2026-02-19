use crate::GossamerMessage;
use parabyzantine::{
	buffer::{QueryPlanlike, Querylike},
	hart::{ParabyzantineDataBinding, ParabyzantineDataSpec},
};

/// A [GossamerMessages] trait is used to build queries over the Gossamer messages.
pub trait GossamerMessages<
	Message: GossamerMessage,
	Binding: ParabyzantineDataBinding,
	Query: Querylike<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
		Message,
	>,
	QueryPlan: for<'a> QueryPlanlike<
		'a,
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
		Message,
		Query,
	>,
>
{
	fn gossamer_messages(&mut self) -> QueryPlan;
}
