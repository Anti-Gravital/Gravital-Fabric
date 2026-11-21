# ADR-0003 - Portability levels are explicit and conditional

- Status: accepted
- Date: 2026-11-21
- Decider: BDFL (Angel Nereira)

## Context

A captured memory and CPU state depends on instruction sets, hypervisor,
virtual devices and kernel versions. Promising that any execution state can be
resumed anywhere would be false.

## Decision

Three levels are published: L1 image, L2 workload and L3 execution. L1 and L2
are standard targets; L3 is conditional on a compatibility contract. If L3
cannot be restored, the platform boots from the image, restores persistent
data and records the downgrade.

## Consequences

- Users can reason about what a migration preserves.
- Documentation and API results must name the level actually achieved.
- Tests assert the downgrade path, not only the happy path.

## Alternatives

- Single promise of full portability: rejected, technically false.
- No L3 at all: rejected, it removes a real capability for compatible hosts.

## Status

Accepted. Implemented by RFC-0002.
