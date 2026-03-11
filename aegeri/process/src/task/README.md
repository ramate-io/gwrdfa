# Aegeri Task Mempool Notes

This module is an ID-only scheduler for consensus staging.

## Storage Model

- `by_slot: BTreeMap<Slot, BTreeSet<Id>>`
  - Backing storage and deterministic scheduling view.
  - Stores only IDs.
- `inflight: HashMap<Id, (IndexValue, Slot)>`
  - Tracks which consensus index currently "owns" an ID.
  - Preserves original slot while in-flight so return-to-pool is stable.

## Behavioral Policy

- Candidate selection uses slots `< t - 1` where `t = floor(now / slot_width_ms)`.
- Availability pop moves IDs from `by_slot` into `inflight`.
- Index changes preserve each ID's original slot.
- Returning IDs to the pool re-inserts them into `by_slot` with the preserved slot.
- Message storage and block reification are intentionally outside this module.

This separation keeps mempool concerns small (ordering + stage ownership) and
makes future transaction re-broadcast logic easier to extend.
