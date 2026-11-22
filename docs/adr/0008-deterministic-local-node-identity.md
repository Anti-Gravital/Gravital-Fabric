# ADR-0008 - Deterministic local node identity for the prototype

- Status: accepted for the prototype slice defined in RFC-0008; cryptographic
  identity for the Phase 1 gate is still required.
- Date: 2026-11-22
- Decider: Gravital Labs, wave-9 task assignment

## Context

The roadmap Phase 1 asks for a cryptographic node identity. The first prototype
is local-only and must not store secrets or require a key service. It still
needs an identifier that is stable across runs so a local state file refers to
the same host. `std::collections::hash_map::DefaultHasher` is unsuitable because
its algorithm is explicitly not guaranteed stable across Rust releases.

## Decision

For the prototype only, derive a 128-bit identifier from a fixed domain string
and the first available source: `/etc/machine-id`, `/var/lib/dbus/machine-id`,
then the hostname. Use two independent FNV-1a 64-bit accumulators with distinct
offset basis values, little-endian byte order, and render the result as a
lowercase 8-4-4-4-12 hexadecimal string. Record which source was used. No
secret, private key, password or random value is read, generated or stored.

## Consequences

- The identifier is reproducible from public host metadata and unit-tested with
  synthetic inputs.
- It is not cryptographic: it provides no authentication, non-repudiation or
  resistance to impersonation and must never be used as a security credential.
- It may change if the machine id is regenerated or no machine id exists and the
  hostname changes; the recorded source makes that visible.
- Full Phase 1 still needs a cryptographic identity established at registration,
  as required by the architecture master and RFC-0004.

## Alternatives

- `DefaultHasher`: rejected, no stable algorithm guarantee.
- Random UUID stored in a local file: rejected, adds mutable secret-like state
  and is not reproducible from the documented source.
- Reading hardware serial numbers: rejected, requires privileged or
  third-party-specific interfaces and leaks more host information.

## Status

Accepted for the local prototype only. Counterpart: RFC-0008 section 4.3.
