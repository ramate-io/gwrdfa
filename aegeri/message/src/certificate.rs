//! Certificate-side consensus model for Aegeri.
//!
//! The model intentionally separates agreement into four layers:
//! availability -> confirmation -> block header -> transition.

mod certificate;
mod index;
mod proposal;
mod transition;

pub use certificate::Certificate;
pub use index::{Index, IndexValue};
pub use proposal::{Availability, BlockHeader, ByzantineRequirement, Confirmation, Proposal};
pub use transition::{JoinSet, StateRoot, Transition};
