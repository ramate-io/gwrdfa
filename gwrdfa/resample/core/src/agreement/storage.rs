use crate::Resample;
use parabyzantine::{agreement::Agreement, buffer::Stores};

pub trait ResampleAgreementStorage<E, Index, Subcommittee, Value>: Sized
where
	// Self stores subcommittee agreements
	Self: Stores<(Agreement, Resample, Index, Subcommittee), E>
		// Self stores value agreements
		+ Stores<(Agreement, Resample, Index, Value), E>,
{
}

impl<T, E, Index, Subcommittee, Value> ResampleAgreementStorage<E, Index, Subcommittee, Value> for T where
	T: Stores<(Agreement, Resample, Index, Subcommittee), E>
		+ Stores<(Agreement, Resample, Index, Value), E>
{
}
