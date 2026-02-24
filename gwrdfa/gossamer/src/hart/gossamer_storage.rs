use crate::GossamerMessageError;
use crate::{Broadcast, In, InFlight, Out};
use parabyzantine::buffer::Stores;

pub trait GossamerMessageStorage<E, M>: Sized
where
	Self: Stores<GossamerMessageError, E>
		+ Stores<M, E>
		+ Stores<(In, M), E>
		+ Stores<Out, E>
		+ Stores<InFlight, E>
		+ Stores<Broadcast, E>,
{
}

impl<T, E, M> GossamerMessageStorage<E, M> for T where
	T: Stores<GossamerMessageError, E>
		+ Stores<M, E>
		+ Stores<(In, M), E>
		+ Stores<Out, E>
		+ Stores<InFlight, E>
		+ Stores<Broadcast, E>
{
}
