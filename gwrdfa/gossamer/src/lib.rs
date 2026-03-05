//! Gossamer networking and messaging crate.
//!
//! This crate wraps a `libp2p` swarm (`gossipsub` + `kademlia` + `ping`) behind
//! a typed message API and integrates with Parabyzantine via `GossamerHart`.
//!
//! ## Submodules
//!
//! - [`config`]: build and configure a `Gossamer` instance and swarm behavior.
//! - [`p2p`]: composed `NetworkBehaviour` used by the swarm.
//! - [`task`]: async swarm driver, publish retry queue, and task-level errors.
//! - [`gossamer`]: public client-facing API (`send`, `recv`, confirmations).
//! - [`hart`]: Parabyzantine integration for message lifecycle inference updates.
//! - [`container`]: message container and deltas container types.
//! - [`local_cluster`]: local multi-node harness and stress tests.
//!
//! ### Nested submodules
//!
//! - [`hart::gossamer_messages`]: query planning trait for outbound messages.
//! - [`hart::gossamer_storage`]: storage trait bounds used by `GossamerHart`.
//! - [`container::container`]: concrete `GossamerContainer`.
//! - [`container::delta_container`]: `GossamerDeltasContainer` delta application.
//!
pub mod config;
pub use config::*;

pub mod container;
pub mod hart;

pub mod p2p;
pub use p2p::*;

pub mod gossamer;
pub use gossamer::*;

pub mod task;
pub use task::*;

pub mod local_cluster;
