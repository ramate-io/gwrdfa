# `aegeri-process`
The process crate for running Aegeri resample consensus and computing state transitions over Fuste.

## Integration Design

`aegeri-process` composes `gossamer` broadcast, `gwrdfa-resample` consensus, and `fuste` execution.

Planned actors:
- `AegeriMessageIn`: implements `ParabyzantineMessageIn`.
- `AegeriTasks`: implements `ParabyzantineTask`.
- `AegeriMessageOut`: implements `ParabyzantineMessageOut`.

Container ownership:
- `Messages`: reuse gossamer message containers.
- `Certificates` and `Agreements`: reuse resample std-backed containers.
- `Transactions` and `Tasks`: define Aegeri-specific containers in this crate.

Message model:
- `aegeri-message` is already implemented and provides `UnifiedMessage`, `Message<T>`, and verification primitives.
- It is expected to evolve as process integration hardens (especially around certificate/transaction boundaries and execution payload ergonomics).

## Dataflow

```mermaid
flowchart TB
    Gossamer["Gossamer"] -- "ingest broadcast UnifiedMessage" --> Messages["Messages (gossamer containers)"]
    Messages --> AMI["AegeriMessageIn (ParabyzantineMessageIn)"]

    AMI -- "verify + split certificate messages" --> Certificates["Certificates (resample std containers)"]
    AMI -- "verify + split transaction messages" --> Transactions["Transactions (aegeri containers)"]

    Sub["Subcommittee"]-..->Certificates
    Certificates --> Resample["Resample agreement"]
    Resample -- "agreement inferences" --> Agreements["Agreements (resample std containers)"]

    Agreements --> AT["AegeriTasks (ParabyzantineTask)"]
    Transactions --> AT

    AT -- "build block from mempool" --> BlockTx["BlockTransactions"]
    BlockTx -- "collect joiners" --> JoinSet["JoinSet"]
    JoinSet-. "derives" .->Sub
    BlockTx -- "execute ELF payloads" --> Fuste["Fuste"]
    Fuste -- "compute state deltas" --> BlockDeltas["BlockDeltas"]

    JoinSet --> Block
    BlockDeltas --> Block["Block"]

    Block --> Tasks["Tasks (aegeri containers)"]
    Tasks --> AMO["AegeriMessageOut (ParabyzantineMessageOut)"]
    AMO -- "sign + wrap UnifiedMessage + mark for broadcast" --> Messages
    Messages --> Gossamer
```

## Consensus Stages

`aegeri-message` models certificate consensus as layered proposal stages for the same round index:

- `Availability`: replicas advertise candidate transaction IDs they have seen.
- `Confirmation`: replicas narrow to quorum-observed candidates.
- `BlockHeader`: replicas converge on exact block transaction IDs.
- `Transition`: replicas converge on exact post-state commitment (`state_root`, `join_set`).

This staging reduces sensitivity to mempool timing skew by deferring exact agreement
until after candidate-set convergence.

```mermaid
flowchart LR
    A["Index::Availability(i)\nProposal::Availability"] --> C["Index::Confirmation(i)\nProposal::Confirmation"]
    C --> B["Index::Block(i)\nProposal::BlockHeader"]
    B --> T["Index::Transition(i)\nProposal::Transition"]
    T --> N["Index::Availability(i+1)\nnext round"]

    A -. "union-like candidate spread" .-> C
    C -. "quorum filtering" .-> B
    B -. "deterministic block selection" .-> T
    T -. "state commitment finalized" .-> N
```