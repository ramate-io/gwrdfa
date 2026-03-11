# Aegeri Task Mempool Notes

This module keeps transaction payloads in one place and indexes everything else by ID.

## Storage Model

- `by_id: HashMap<Id, VerifiedMessage<Transaction>>`
  - Backing storage for transaction payloads.
  - O(1) ID lookup for proposal building and reification.
- `by_slot: BTreeMap<Slot, BTreeSet<Id>>`
  - Deterministic ordering/scheduling view for candidate selection.
  - Stores IDs only, never transaction payloads.
- `inflight: HashMap<Id, IndexValue>`
  - Tracks which consensus index currently "owns" a transaction.
  - Internal selection methods must skip IDs already mapped in-flight.

## Behavioral Policy

- Candidate selection uses slots `< t - 1` where `t = floor(now / slot_width_ms)`.
- `by_slot` cleanup is lazy:
  - if an ID exists in `by_slot` but not in `by_id`, it is pruned when encountered.
- Reification errors are handled similarly:
  - if a requested ID is missing from `by_id`, reification returns an error.
- In-flight mapping is updated as proposals progress:
  - availability marks selected IDs in-flight,
  - confirmation and block-header updates reconcile mapping with the latest stage.

This model keeps mempool payload ownership simple and leaves room for future
re-broadcast flows without moving payloads between multiple stores.
