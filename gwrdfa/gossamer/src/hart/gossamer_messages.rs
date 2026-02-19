use crate::{GossamerMessage, InFlight, Out};
use parabyzantine::{
	buffer::{QueryPlanlike, Querylike},
	hart::{ParabyzantineDataBinding, ParabyzantineDataSpec},
};

/// A [GossamerMessages] trait is used to build queries over the Gossamer messages.
pub trait GossamerMessages<
	Message: GossamerMessage,
	Binding: ParabyzantineDataBinding,
	OutQuery: Querylike<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
		(Out, Message),
	>,
	OutQueryPlan: for<'a> QueryPlanlike<
		'a,
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
		(Out, Message),
		OutQuery,
	>,
	InFlightQuery: Querylike<
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
		(InFlight, Message),
	>,
	InFlightQueryPlan: for<'a> QueryPlanlike<
		'a,
		<Binding::Spec as ParabyzantineDataSpec>::MessageEntity,
		<Binding::Spec as ParabyzantineDataSpec>::MessageBuffer,
		(InFlight, Message),
		InFlightQuery,
	>,
>
{
	fn gossamer_messages_out_plan(&mut self) -> OutQueryPlan;

	fn gossamer_messages_in_flight_plan(&mut self) -> InFlightQueryPlan;
}
