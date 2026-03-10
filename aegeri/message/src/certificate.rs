//! Certificate-side consensus model for Aegeri.
//!
//! The model intentionally separates agreement into four layers:
//! availability -> confirmation -> block proposal -> transition.

mod block;
mod certificate_type;
mod index;
mod proposal;
mod transition;

pub use block::{Block, TransactionSet};
pub use certificate_type::Certificate;
pub use index::Index;
pub use proposal::{Availability, BlockProposal, Confirmation, Proposal};
pub use transition::{JoinSet, StateRoot, Transition};
