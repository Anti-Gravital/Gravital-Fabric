# Portable Workload migration flow

> Last reviewed: 2026-11-21. Master chapter: Architecture, section 5.

```mermaid
flowchart LR
    A[Node A: running workload] --> C[Checkpoint memory, vCPU, devices]
    C --> V[Coordinate volumes for consistent point]
    V --> P[Portable Workload package]
    P --> T[Transfer to node B or another provider]
    T --> H[Hardware adaptation probe]
    H --> Compatible{Level compatible?}
    Compatible -->|yes| R[Restore state and volumes]
    R --> Resume[Resume execution - L3]
    Compatible -->|no| Boot[Boot from image]
    Boot --> Restore[Restore persistent data - L1 or L2]
    Resume --> Audit[Record achieved level]
    Restore --> Audit
```

L3 is recorded only when the compatibility contract matches. Otherwise the
platform performs a documented downgrade and reports it to the caller.
