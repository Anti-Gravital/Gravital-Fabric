# ADR-0004 - Storage engines are replaceable adapters

- Status: accepted
- Date: 2026-11-21
- Decider: BDFL (Angel Nereira)

## Context

ZFS, Ceph and object stores have different trade-offs. A platform that bakes
one engine into its contract cannot serve small single-node installations and
large distributed deployments equally well.

## Decision

The fabric owns the volume, snapshot and replication contracts. Engines are
adapters behind traits, selected per deployment and gated by Cargo features.
An S3-compatible API is offered as an interface, not as a mandatory product.
No storage engine is implemented from scratch.

## Consequences

- A small deployment can use a native engine; a large one can use a
  distributed engine.
- Contract tests must run against every enabled adapter.
- Engine-specific features stay confined to the adapter module.

## Alternatives

- Mandatory Ceph: rejected, too heavy for small installations.
- Mandatory local ZFS: rejected, no distributed replication.
- Implement a custom filesystem: rejected, unnecessary complexity.

## Status

Accepted. Implemented by RFC-0003.
