//! Certificate-side consensus model for Aegeri.
//!
//! The model intentionally separates agreement into four layers:
//! availability -> confirmation -> block header -> transition.

mod block;
mod certificate;
mod index;
mod proposal;
mod transition;

pub use block::{Block, TransactionSet};
pub use certificate::Certificate;
pub use index::Index;
pub use proposal::{Availability, BlockHeader, Confirmation, Proposal};
pub use transition::{JoinSet, StateRoot, Transition};
