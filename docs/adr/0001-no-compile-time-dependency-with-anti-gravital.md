# ADR-0001 - No compile-time dependency with Anti-Gravital

- Status: accepted
- Date: 2026-11-21
- Decider: BDFL (Angel Nereira)

## Context

Gravital Fabric and Anti-Gravital are independent open projects. Fabric must be
installable on a machine that has no framework installed, and Anti-Gravital
must be usable on infrastructure that does not run Fabric.

## Decision

No crate named `fabric-*` depends on a crate named `ag-*`. Both projects
consume the same versioned contracts as data. An Anti-Gravital component may
implement a Fabric component as an optional adapter, selected at deployment
time, never as a mandatory link.

## Consequences

- Neither project controls the other's release cycle.
- Independence is mechanically checkable by inspecting manifests.
- Shared types come from generated code, so contract changes are explicit.
- Some duplication of small value types is acceptable to keep the boundary
  clean.

## Alternatives

- Shared Rust crate: rejected, coupling release cycles.
- One-way dependency: rejected, asymmetric lock-in.
- Runtime dynamic loading of framework code: rejected, complexity without
  need when the contract is data.

## Status

Accepted. Counterpart: Anti-Gravital ADR-0016.
