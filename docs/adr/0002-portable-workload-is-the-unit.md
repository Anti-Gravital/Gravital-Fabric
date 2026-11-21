# ADR-0002 - Portable Workload is the unit of portability

- Status: accepted
- Date: 2026-11-21
- Decider: BDFL (Angel Nereira)

## Context

The platform needs a unit that can be exported, inspected, moved and restored
without depending on a provider's private format.

## Decision

The unit is a declarative `Portable Workload`: image, virtual hardware,
volumes, networks, identity, policies, dependencies and an optional checkpoint.
A disk image or a provider snapshot is only one field of that unit, not the
unit itself.

## Consequences

- Every engine works with the same manifest instead of provider formats.
- A workload can be exported without an account on the platform.
- Migration records which level was achieved and which state was restored.

## Alternatives

- Disk image: rejected, insufficient context.
- Provider snapshot: rejected, vendor lock-in.
- OCI image only: rejected, no volumes, identity or policy.

## Status

Accepted. Implemented by RFC-0002.
