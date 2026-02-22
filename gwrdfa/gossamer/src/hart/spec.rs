use crate::{hart::gossamer_messages::GossamerMessages, GossamerMessage, GossamerMessageError};
use crate::{Broadcast, In, InFlight, Out};
use parabyzantine::{
	buffer::{
		query::{IntoQuery, Querylike},
		Stores,
	},
	hart::{ParabyzantineDataBinding, ParabyzantineDataSpec},
};

pub trait GossamerSpec<Binding: ParabyzantineDataBinding> {
	type Message: GossamerMessage;
	type Binding: ParabyzantineDataBinding;
	type OutQuery<'a>: Querylike<
		<<Self::Binding as ParabyzantineDataBinding>::Spec as ParabyzantineDataSpec>::MessageEntity,
		Item = (&'a Out, &'a Self::Message)
	> where Self::Message: 'a, &'a <<Self::Binding as ParabyzantineDataBinding>::Spec as ParabyzantineDataSpec>::MessageBuffer:
	IntoQuery<
		<<Self::Binding as ParabyzantineDataBinding>::Spec as ParabyzantineDataSpec>::MessageEntity,
		Self::OutQueryPlan,
	>;
	type OutQueryPlan;

	/// The type of the message builder.
	type Messages<'a>: GossamerMessages<
		Binding = Self::Binding,
		Message = Self::Message,
		OutQuery<'a> = Self::OutQuery<'a>,
		OutQueryPlan = Self::OutQueryPlan,
	>;
}
