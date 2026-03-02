use crate::Resample;
use parabyzantine::{agreement::Agreement, buffer::Stores};

pub trait ResampleAgreementStorage<E, Subcommittee, Value>: Sized
where
	// Self stores subcommittee agreements
	Self: Stores<(Agreement, Resample, Subcommittee), E>
		// Self stores value agreements
		+ Stores<(Agreement, Resample, Value), E>,
{
}

impl<T, E, Subcommittee, Value> ResampleAgreementStorage<E, Subcommittee, Value> for T where
	T: Stores<(Agreement, Resample, Subcommittee), E> + Stores<(Agreement, Resample, Value), E>
{
}
