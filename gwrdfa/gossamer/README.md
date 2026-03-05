# `gwrdfa-gossamer`

`gossamer` is the networking/messaging crate for `gwrdfa`. It wraps `libp2p` gossipsub + kademlia behind a small API and integrates with the Parabyzantine "Hart" layer for message lifecycle bookkeeping.

## Goals

- Provide a typed message API (`GossamerMessage`) over a `libp2p` swarm.
- Keep transport concerns inside the crate, while higher-order policy stays in Hart/client code.
- Support eventual delivery under weak liveness assumptions (peers can appear late).
- Expose enough signal for callers to reason per-entity about send/confirm outcomes.

## Module Overview

- [`src/config.rs`](./src/config.rs): `GossamerConfig` and swarm construction.
- [`src/p2p.rs`](./src/p2p.rs): `GossamerBehaviour` (`gossipsub`, `kad`, `ping`).
- [`src/task.rs`](./src/task.rs): `GossamerTask` future driving outbound publish + inbound ingest.
- [`src/gossamer.rs`](./src/gossamer.rs): public API (`send`, `recv`, confirmation methods, timeout helpers).
- [`src/hart.rs`](./src/hart.rs): Parabyzantine integration (`GossamerHart`).
- [`src/container/*`](./src/container): ECS/container types and deltas for `In`, `Out`, `InFlight`, `Broadcast`, and error markers.
- [`src/local_cluster.rs`](./src/local_cluster.rs): local test harness for multi-peer convergence/stress testing.

## Runtime Architecture

`Gossamer::spawn_tokio` builds the task and spawns it on Tokio. The underlying swarm is built with:

- TCP + Noise + Yamux + DNS transport.
- `gossipsub` subscribed to one topic.
- `kademlia` memory store.
- `ping`.

The spawned task owns the swarm and pumps three channels:

- outbound entities/messages into gossipsub publish,
- inbound gossipsub messages back to `Gossamer`,
- publish confirmations/errors back to `Gossamer`.

## Core API Semantics

### Message encoding

Callers define:

- `to_gossamer_bytes()`
- `from_gossamer_bytes()`

via `GossamerMessage`.

### Send path

- `send_message(entity, message)` queues outbound data to the task.
- `send_and_confirm*` are convenience wrappers that wait for one confirmation event.

### Receive path

- `recv_message*` reads inbound gossipsub message data and decodes it.

### Confirmation path

Confirmations are entity-aware:

- `GossamerConfirmation<Entity> = Result<Entity, (Entity, GossamerTaskError)>`
- `try_recv_confirmation` / `wait_for_confirmation*` return `Option<GossamerConfirmation<_>>`

This lets higher layers attribute confirmation failures to the originating entity.

The current [`GossamerHart`](./src/hart.rs) implementation already does this entity-scoped attribution when processing confirmations. There is not yet a separate raw client in this crate that continuously drains confirmations and applies policy itself. Today you can either (a) use the Hart path if you are plugging into Parabyzantine systems (for example a light client), or (b) implement a custom client loop with similar confirmation tracking semantics to the facts/inferences model used by the Hart buffers.

## Convergence and Delivery Decisions

Recent design decisions in this crate:

1. **Fully-qualified listen addresses**
   - `spawn_tokio` appends `/p2p/<peer_id>` to returned listen addresses.
   - Rationale: bootstrap peers must carry peer identity for robust dialing/kad seeding.

2. **Startup publish tolerance**
   - gossipsub is configured with `flood_publish(true)`.
   - Rationale: reduce early mesh timing flakiness when publishing before full mesh stabilization.

3. **Swarm lifecycle handling in task**
   - On `ConnectionEstablished`: add explicit gossipsub peer + trigger `kad.bootstrap()`.
   - On `ConnectionClosed`: remove explicit peer.
   - On `OutgoingConnectionError`: log for visibility.
   - Rationale: make recovery and discovery progress explicit under partial connectivity.

4. **Deferred retry on `InsufficientPeers`**
   - `PublishError::InsufficientPeers` does not immediately fail the message.
   - Message is deferred in an internal pending queue and retried by the task loop.
   - Rationale: preserve low-liveness assumptions and allow eventual convergence.

5. **Bounded pending queue with byte cap**
   - `GossamerConfig::max_pending_outbound_bytes` (default `1 MiB`).
   - Queue logic is encapsulated in `PendingOutbound`.
   - If cap would be exceeded, task emits `GossamerTaskError::PendingOutboundFull`.
   - Rationale: avoid unbounded memory growth while still supporting deferred retries.

6. **Eviction policy stays out of transport**
   - This crate enforces only a bounded pending queue and reports pressure via error.
   - Higher-order eviction/retention policy remains a Hart/client concern.

## `PendingOutbound` helper

`PendingOutbound<Entity>` (in `task.rs`) centralizes queue/backpressure logic:

- tracks queue bytes,
- checks overflow and max cap in `push`,
- decrements accounting in `pop`.

This keeps `GossamerTask` focused on swarm/event orchestration.

## Hart Integration

`GossamerHart` translates transport events into inference markers:

- Outbound (`Out`) $\to$ `InFlight` after `send_message`,
- Confirmation success $\to$ remove `InFlight`, add `Broadcast`,
- Confirmation failure $\to$ attach error on the same entity,
- Inbound messages $\to$ `(In, message)` inference.

This is where application-level policy can react to per-entity status.

## Configuration

`GossamerConfig` fields:

- `identity`
- `topic`
- `listen_on`
- `bootstrap_peers`
- `max_pending_outbound_bytes`

Common builder methods:

- `with_identity(...)`
- `with_topic(...)`
- `with_listen_on(...)`
- `with_bootstrap_peers(...)`
- `with_max_pending_outbound_bytes(...)`

## Testing Notes

- `local_cluster` provides ignored tests that use ephemeral ports.
- Stress test (`issue #18`) repeatedly checks send/receive convergence.
- Run ignored tests manually:
  - `cargo test -p gossamer -- --ignored`

## Non-goals (current)

- No transport-layer eviction semantics beyond queue byte cap enforcement.
- No "global delivered set" tracking in `gossamer`; confirmation tracking belongs in higher layers.
- No hard assumption that all peers are online at publish time.