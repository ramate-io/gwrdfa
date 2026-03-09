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

    Certificates --> Resample["Resample agreement"]
    Resample -- "agreement inferences" --> Agreements["Agreements (resample std containers)"]

    Agreements --> AT["AegeriTasks (ParabyzantineTask)"]
    Transactions --> AT

    AT -- "build block from mempool" --> BlockTx["BlockTransactions"]
    BlockTx -- "collect joiners" --> JoinSet["JoinSet"]
    BlockTx -- "execute ELF payloads" --> Fuste["Fuste"]
    Fuste -- "compute state deltas" --> BlockDeltas["BlockDeltas"]

    JoinSet --> Block
    BlockDeltas --> Block["Block"]

    Block --> Tasks["Tasks (aegeri containers)"]
    Tasks --> AMO["AegeriMessageOut (ParabyzantineMessageOut)"]
    AMO -- "sign + wrap UnifiedMessage + mark for broadcast" --> Messages
    Messages --> Gossamer

    Sub["Subcommittee"]-..->Certificates
    JoinSet-..->Sub
```