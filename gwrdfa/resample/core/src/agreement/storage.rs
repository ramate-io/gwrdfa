use crate::Resample;
use parabyzantine::{agreement::Agreement, buffer::Stores};

/// Storage capability required by resample agreement inferences.
///
/// Implementers can store both:
/// - subcommittee agreements: `(Agreement, Resample, Index, Subcommittee)`
/// - value agreements: `(Agreement, Resample, Index, Value)`
///
/// This is intentionally a marker trait over `Stores` bounds so consumers can
/// describe capability without tying to a concrete storage type.
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
