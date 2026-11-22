# RFC-0008 - First local node prototype and contract layout

- Status: accepted for the local prototype slice only by the wave-9 task
  assignment (2026-11-22); full Phase 1 remains gated by the roadmap. Formal
  review happens when branch `community/fabric-node` is integrated.
- Decision date: 2026-11-22
- Author: Gravital Labs
- Target phase: Phase 1 (partial prototype)
- Affected modules/crates: first `fabric-node` crate, first `contracts/` entry,
  kaizen local command wiring
- Predecessor RFC: RFC-0001, RFC-0004
- Comment period: prototype scope limited to the assigned wave-9 task.

## 1. Motivation

The repository has a complete specification but no executable product code. The
wave-9 board assigned a first runnable prototype so that the node engine can be
exercised on a real host without deploying any external service. That prototype
needs an explicit authorization boundary: it is a local discovery sample, not a
Phase 1 implementation and not a Fabric alpha.

## 2. Problem

1. `CLAUDE.md` section 7 and rule K-1 require an RFC before the first crate, a
   contract or a tooling change; none currently authorizes them.
2. Without a published contract layout, the prototype would emit an ad hoc JSON
   shape that could not become a versioned interface later.
3. Without an explicit statement of scope, a local command could be mistaken for
   the roadmap deliverable "register the node with Fabric" or for the two-machine
   Phase 1 acceptance criterion.
4. The alpha-positioning document currently says the repository has no
   `fabric-*` crate; code without a documentation update would make that text
   false.

## 3. Alternatives considered

**A. Add code without an RFC and update docs afterwards.** Rejected: violates
the technical constitution and the documentation-first rule.

**B. Implement the whole Phase 1 now (provisioning, images, two hosts).**
Rejected: the roadmap gate and the seven conditions in the alpha-positioning
document are not satisfied; this task is explicitly a first prototype only.

**C. Publish a narrow implementation RFC, a versioned JSON Schema as data and a
single local-only crate (chosen).** The prototype is runnable and auditable, and
the boundary stays honest.

## 4. Proposed design

### 4.1 Prototype scope

One Rust binary crate `fabric-node` with four local commands: `manifest`,
`register [--state PATH]`, `--version` and `--help`. It reads local hardware
facts, derives a stable non-cryptographic local identifier, writes JSON to
stdout or to a caller-selected local state file, and exits. It opens no socket,
starts no daemon, requires no control plane, container runtime, hypervisor or
cloud account.

Explicitly absent: provisioning, image export/import, cryptographic identity,
control-plane registration, scheduling, replication, metering and any machine
interface. Those remain later Phase 1 or later phase work under their own RFCs.

### 4.2 Contract

A versioned JSON Schema (JSON Schema 2020-12) is published as data under
`contracts/fabric-node-manifest/v1.schema.json`, with `manifest_version`
`1.0.0` and a synthetic example under the same directory. `contracts/README.md`
defines the layout and versioning rules. Types are not generated in this
prototype; publishing the schema as data is the first step of the layout
required by RFC-0001 section 5. No crate in this repository or in Anti-Gravital
imports the other (ADR-0001, Anti-Gravital ADR-0016).

### 4.3 Identity

The identifier is a deterministic 128-bit value derived from local machine
metadata. It is stable across runs on the same host and requires no stored
secret. The exact algorithm and its limitations are recorded in ADR-0008. It is
not a cryptographic identity; roadmap Phase 1 still requires cryptographic
identity before its gate can close.

### 4.4 Tooling

A Cargo workspace root is added with `crates/fabric-node` as a member and
`tools/kaizen-insights` excluded, so the existing standalone governance tool
keeps building with its own `Cargo.lock`. `.kaizen/config.env` gets real
`KZ_FMT_CMD`, `KZ_LINT_CMD` and `KZ_TEST_CMD` values so `scripts/kaizen check`
validates the crate. No CI permission, branch protection or baseline change is
part of this RFC.

### 4.5 Documentation

README (both language sections), CHANGELOG, `docs/INDEX.md`,
`docs/releases/alpha-positioning.md`, `docs/examples/anti-gravital-prototype.md`
and a new audit note with real host evidence are updated in the same branch.
The roadmap master document is not edited: Phase 1 stays open and partial.

## 5. Implementation plan

1. Publish this RFC, ADR-0008, the contract and the descriptor.
2. Add the workspace and the zero-dependency crate with unit and integration
   tests.
3. Run the binary on this host and record unedited evidence in `docs/audits/`.
4. Run format, check, clippy, test, kaizen check and content scans; commit.
5. Submit the branch for review; no push and no merge by this role.

## 6. Risks

| Risk | Probability | Impact | Mitigation |
| --- | --- | --- | --- |
| Prototype read as a Fabric alpha | Medium | High | Scope statements in README, CHANGELOG, positioning doc and audit note; roadmap box stays partial |
| Identity changes between runs or toolchains | Medium | Medium | Fixed FNV-1a with explicit byte order and a recorded source field; unit tests lock the algorithm |
| Workspace root breaks the insights tool | Medium | Low | Explicit workspace `exclude`; verify its manifest still builds |
| JSON shape drifts from the schema | Low | Medium | Contract version field, schema-required fields, integration assertions on real output |
| Unwanted local writes | Low | Low | Only the caller-selected `--state` path is written; default is a file in the current directory |

## 7. Impact

- First Rust product code in the Fabric repository; still no published release.
- No external service, no network call, no new crate dependency.
- Kaizen check begins running format, lint and test commands locally.

## 8. Rollback

Revert the three branch commits. No global state is created: the only persistent
artifact outside the repository is the optional state file written at the path
the caller chose.

## 9. References

- `docs/master/GRAVITAL-FABRIC-Hoja-de-Ruta.md`, Phase 1 and Gates.
- `docs/master/GRAVITAL-FABRIC-Arquitectura-Tecnica.md`, sections 1, 3 and 8.
- `docs/rfc/RFC-0001-scope-and-boundaries.md`, section 5.
- `docs/rfc/RFC-0004-control-plane-and-adapters.md`, section 4.1.
- `docs/releases/alpha-positioning.md`, section 9.
