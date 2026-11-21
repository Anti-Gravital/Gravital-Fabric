# RFC-0001 - Scope and boundaries of Gravital Fabric

- Status: accepted
- Decision date: 2026-11-21
- Author: Angel Nereira (BDFL), Gravital Labs
- Target phase: governance; applies from acceptance
- Affected modules/crates: all future `fabric-*` crates
- Predecessor RFC: none
- Comment period: waived by BDFL decision.

## 1. Motivation

The Gravital platform has three layers: Anti-Gravital as the open technology
foundation, Gravital Fabric as the infrastructure platform, and a commercial
catalog built on both. Without an explicit boundary, the infrastructure engine
would absorb product features and the open projects would become mutually
mandatory. This RFC fixes the boundary before code exists.

## 2. Problem

1. A platform that provisions infrastructure is easily confused with a hosting
   provider; the two must not be mixed in one repository.
2. If Fabric required Anti-Gravital, it could not be installed on machines that
   only need infrastructure management.
3. If the commercial catalog lived in this repository, open and paid code
   would share a release cycle and a license.
4. Without a scope statement, adapter work tends to become a reimplementation
   of dominant tools.

## 3. Alternatives considered

**A. One repository for all three layers.** Rejected: mixed licenses, mixed
release cycles and an implicit mandatory dependency.

**B. Fabric inside the Anti-Gravital workspace as `fabric-*` crates.** Rejected:
it still couples installation and makes independence hard to prove.

**C. Three repositories with open contracts (chosen).** Each layer is
installable and replaceable on its own.

## 4. Proposed design

1. This repository contains only the open infrastructure engine, its
   contracts, its documentation and its adapters.
2. Anti-Gravital and Fabric communicate through versioned open data:
   Protobuf, OpenAPI, JSON Schema and declarative manifests.
3. No `fabric-*` crate depends on an `ag-*` crate and no `ag-*` crate is
   required at runtime. An Anti-Gravital implementation may satisfy a Fabric
   contract as an optional adapter.
4. The commercial catalog lives in a separate repository, consumes the same
   contracts and contains no private code here.
5. Integration with Docker, Kubernetes, Terraform and cloud providers is done
   through adapters; those tools are not replaced.
6. A user can export their workloads, images, volumes, configuration and
   snapshots in documented formats without an active account.

### Scope statement

Gravital Fabric is open infrastructure software that turns customer-owned
physical or cloud resources into a portable, administrable and replicable
infrastructure. It is not a hosting service, a general-purpose orchestrator or
a replacement for the tools it integrates with. Any proposal outside this
statement needs an RFC that amends it first.

## 5. Implementation plan

1. Publish the master documents and this RFC.
2. Define the contract repository layout under `contracts/`.
3. Implement Phase 1 of the roadmap after the engine and contract RFCs are
   accepted.

## 6. Risks

| Risk | Probability | Impact | Mitigation |
| --- | --- | --- | --- |
| Scope grows into a hosting product | Medium | High | Scope statement and RFC requirement. |
| Contract drift with Anti-Gravital | Medium | Medium | Versioned schemas and snapshot tests. |
| Adapter becomes a fork of a third-party tool | Medium | Medium | Adapter labels behind features; no own component named after a brand. |

## 7. Impact

- Three independent repositories and one shared set of contracts.
- No change to Anti-Gravital public APIs.
- Users may install Fabric without any framework dependency.

## 8. Rollback

If the separation proves wrong, the open contracts remain valid and the
repositories can be archived independently; nothing is deleted from history.

## 9. References

- Anti-Gravital `ADR-0016`.
- `GRAVITAL-FABRIC-Blueprint-v1.0.md` sections 2 and 3.
