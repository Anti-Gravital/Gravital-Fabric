# RFC-0004 - Control plane, engines and provider adapters

- Status: accepted
- Decision date: 2026-11-21
- Author: Angel Nereira (BDFL), Gravital Labs
- Target phase: Phase 3
- Affected modules/crates: all `fabric-*` engines
- Predecessor RFC: RFC-0001
- Comment period: waived by BDFL decision.

## 1. Motivation

A heterogeneous fleet cannot be managed by assuming every host has the same
capabilities. The control plane must coordinate nodes without becoming a
single point of failure, and provider differences must be confined to
adapters.

## 2. Problem

1. Nodes in a colocation, a customer office, a rented dedicated server, a VPS
   and a cloud account have different capabilities.
2. A control plane inside a provider would be lost with that provider.
3. Replication, high availability, migration, scaling and disaster recovery
   are often treated as one feature, producing misleading guarantees.
4. A connected agent needs a safe way to operate the fleet.

## 3. Alternatives considered

**A. One agent per provider with a private protocol.** Rejected: duplicate
security work and no offline operation.

**B. Central controller that must be reachable for every workload action.**
Rejected: a network partition would stop existing workloads.

**C. External control plane plus autonomous nodes and open contracts (chosen).**

## 4. Proposed design

### 4.1 Engines

The twelve engines of `GRAVITAL-FABRIC-Arquitectura-Tecnica.md` communicate
through versioned contracts. A node agent implements `fabric-node`,
`fabric-compute`, `fabric-runtime` and the local part of `fabric-security`.
Global inventory, scheduling and reconciliation live in `fabric-control`.

### 4.2 Node autonomy

A node keeps running assigned workloads when the control plane is
unreachable. It queues state changes and reconciles when connectivity returns.
A newly started workload still requires a reachable control plane or a
pre-delivered plan.

### 4.3 Provider adapters

Adapters cover owned colocation, customer servers, dedicated servers, third-party
VPS, cloud virtual machines and external clusters. Each adapter declares the
capabilities it supports; the scheduler places a workload only where its
requirements are satisfied. No adapter is mandatory, and the native path
works without any external service.

### 4.4 Distinct guarantees

| Capability | Meaning |
| --- | --- |
| Replication | Copies of data exist |
| High availability | Service continues after a failure |
| Migration | A workload moves to another host |
| Scaling | Capacity is added or removed |
| Disaster recovery | Service is restored after a severe interruption |

Each capability has its own acceptance test. One does not imply another.

### 4.5 Machine interface

The capability-scoped MCP contract of RFC-0024 is implemented here with
Fabric-specific tools under the same capability names. A connected agent
operates projects and infrastructure through one contract.

Anti-Gravital amended RFC-0024 on 2026-11-21 with seven implementation
prerequisites (section 9 of that RFC). Fabric inherits them before any MCP
code is written: machine-readable output, plan/apply and an audit sink in the
consumer, the `Secret<T>` redaction contract, typed executors for every
command group, one coherent plan model, a named owner for `contract_version`,
and no DNS egress from a stdio-only debug interface. `deploy.*` tools remain
unimplemented until Phase 5 exists and must not become a second control-plane
contract alongside the open Protobuf/OpenAPI/JSON Schema definitions.

## 5. Implementation plan

1. Define engine contracts and the capability schema.
2. Implement the node agent and local autonomy.
3. Implement the control plane with inventory and scheduling.
4. Add provider adapters one at a time behind features.

## 6. Risks

| Risk | Probability | Impact | Mitigation |
| --- | --- | --- | --- |
| Control plane becomes a single point of failure | Medium | High | Node autonomy and local plans. |
| Adapter capability claims are wrong | Medium | Medium | Capability probes and contract tests. |
| Agent gains excessive privileges | Low | High | Deny-by-default capability manifest and audit log. |

## 7. Impact

Defines Phase 3 and the runtime shape of every engine.

## 8. Rollback

An adapter is disabled by feature; the node continues with native capabilities.

## 9. References

- RFC-0024 of Anti-Gravital, including its 2026-11-21 amendment (shared MCP
  contract and implementation prerequisites).
- ADR-0001, ADR-0005.
