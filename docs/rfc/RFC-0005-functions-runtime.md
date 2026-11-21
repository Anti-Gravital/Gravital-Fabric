# RFC-0005 - Functions runtime

- Status: accepted
- Decision date: 2026-11-21
- Author: Angel Nereira (BDFL), Gravital Labs
- Target phase: Phase 4
- Affected modules/crates: `fabric-runtime`, `fabric-compute`, `fabric-security`
- Predecessor RFC: RFC-0004
- Comment period: waived by BDFL decision.

## 1. Motivation

A large share of backend work is small, event-shaped code: a webhook, an image
resize, a nightly reconciliation. It needs its own unit of deployment,
isolation and limits instead of sharing the privileges and failure domain of a
long-running service.

## 2. Problem

1. A worker handler is compiled into the service; it cannot be deployed or
   rolled back independently.
2. Starting a virtual machine for each invocation is too slow for short calls.
3. Requiring a container runtime would make containers mandatory, contrary to
   the native-operation rule.
4. An external function platform would become a required service.

## 3. Alternatives considered

**A. Native process per invocation.** Rejected: startup cost, platform-specific
sandboxing and hard resource limits.

**B. Container per invocation.** Rejected as the default: container runtime
requirement and slow startup. Kept as a runtime adapter.

**C. External function platform only.** Rejected: external service required.
Kept as an adapter.

**D. WebAssembly components in the Fabric runtime with capability grants
(chosen).** Sandboxed by construction, millisecond startup, portable across
supported platforms, with containers and external platforms as adapters.

## 4. Proposed design

- An API and event gateway accepts HTTP, queue and schedule triggers.
- A function scheduler assigns invocations, enforces quotas and concurrency,
  and scales from zero where viable.
- A Rust runtime runs native code for trusted functions.
- A WebAssembly runtime runs untrusted functions with deny-by-default
  capabilities and explicit CPU, memory and wall-clock limits.
- A container runtime adapter runs functions that need operating-system
  dependencies.
- Each invocation emits structured telemetry with its function version,
  trigger and resource usage.

## 5. Implementation plan

1. Define the function manifest and trigger contracts.
2. Implement the WebAssembly runtime with capability grants.
3. Add the scheduler and scale-from-zero policy.
4. Add the native and container runtime adapters.

## 6. Risks

| Risk | Probability | Impact | Mitigation |
| --- | --- | --- | --- |
| Sandbox escape | Low | High | Capability model, no ambient authority, independent audits. |
| Resource exhaustion | Medium | Medium | Hard per-invocation limits and admission control. |
| Telemetry leaks secrets | Low | High | Redaction and tests. |

## 7. Impact

Defines the Functions product in Phase 4 without coupling it to a commercial
platform.

## 8. Rollback

Disable the `functions` feature; no existing workload is affected.

## 9. References

- Anti-Gravital RFC-0023 (serverless functions in the framework layer).
- ADR-0001, ADR-0005.
