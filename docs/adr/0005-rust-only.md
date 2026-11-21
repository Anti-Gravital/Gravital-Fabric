# ADR-0005 - Rust is the only implementation language

- Status: accepted
- Date: 2026-11-21
- Decider: BDFL (Angel Nereira)

## Context

A second implementation language would need its own toolchain, CI matrix,
dependency policy, linting, formatting and expertise within a single
maintainer project. The framework layer is already Rust and the runtime targets
are systems-level.

## Decision

All Fabric crates are Rust. WebAssembly is a target for user functions, not a
second implementation language for the platform. Adding a language requires an
RFC that amends this ADR and a CI change covering the new ecosystem.

## Consequences

- One MSRV, one CI matrix, one formatting and lint policy.
- Node agents and control services share types through contracts.
- Contributors need one toolchain.

## Alternatives

- Rust plus Go for agents and adapters: rejected for now; the operational cost
  outweighs the benefit at this stage.
- Language-agnostic services with a generated client in every language:
  rejected, it multiplies the maintenance surface.

## Status

Accepted.
