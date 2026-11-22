# Changelog

All notable changes to Gravital Fabric are recorded here. The project has no
published release yet; entries remain under `[Unreleased]`.

## [Unreleased]

### Fix concurrent register race (issue #3, 2026-11-22)

- Fixed the spurious failure when several `fabric-node register` processes write
  to the same `--state` path. `write_state` no longer uses a shared
  `.<file>.tmp` name: it creates a unique per-writer temporary file in the
  destination directory with `create_new` (atomic O_CREAT|O_EXCL) and retries on
  collision, writes the document, then moves it onto the destination with the
  same atomic rename. Failure cleanup removes only the file that writer
  created, so no writer can delete another writer's temporary file.
- The JSON state format is unchanged: `state_version`, `registered_at_unix` and
  the nested `manifest` are produced by the same code as before.
- Added `concurrent_registers_to_one_state_path_all_succeed`: 32 concurrent
  processes against one path, all exit 0, final state is whole JSON with no
  temporary leftovers and no sleeps. Verified 10/10 reruns and 5 trials of 32
  writers of the original shell reproduction with zero failures.
- Updated `docs/audits/fabric-user-journey.md` with the fix evidence; issue #3
  is closed after full verification.
- Verified: `cargo fmt`, `cargo check`, `cargo clippy -D warnings`, 16 unit and
  7 integration tests, `scripts/kaizen check`, `git diff --check`. No new
  dependency; the Phase 1 gate and the Fabric alpha position are unchanged.

### Real-user terminal QA of the local prototype (2026-11-22)

- Added `docs/audits/fabric-user-journey.md`: a second tester followed the
  documented `cargo run` commands as written in an isolated worktree with a
  private target dir, timing each command and recording verbatim output. Cold
  build 1.74 s, `manifest` 0.39 s, `register` 0.15 s (Cargo included); direct
  binary 0.02 s and 0.01 s. The manifest and the nested state manifest validate
  against the published JSON Schema; two `manifest` runs are byte-identical;
  `register` is intentionally not idempotent because it rewrites the timestamp.
- Documented the honest gap that the registration state document has no
  published schema (only the discovery manifest does) and that there is no
  installer or `cargo install` path: a user can only build from a checkout.
- Reported a reproducible real defect as Fabric issue #3: concurrent `register`
  calls to the same `--state` path can fail spuriously because `write_state`
  uses a fixed temporary filename. Evidence: 1-2 failures in 24-32 parallel
  writers across four trials, with verbatim stderr. No issue was closed and no
  source code was changed by this QA pass.
- `docs/INDEX.md` links the new audit. Fabric remains a specification with a
  local-only prototype; Phase 1 is partial and this is not a Fabric alpha.

### First local node prototype (partial Phase 1)

- RFC-0008 authorizes the first crate and the first versioned contract layout;
  ADR-0008 records the deterministic, non-cryptographic local identifier used
  by the prototype. Both are limited to the prototype slice.
- New workspace and `crates/fabric-node`: a zero-dependency Rust binary that
  discovers OS, architecture, logical CPU count, total memory, block devices
  and network interfaces from std, procfs and sysfs. It prints a versioned JSON
  manifest and writes a local state file with `register --state PATH`.
- Published `contracts/fabric-node-manifest/v1.schema.json` (JSON Schema
  2020-12) and a synthetic example. No generated types and no cross-repository
  Rust dependency; contracts remain data (ADR-0001).
- Real host evidence in `docs/audits/fabric-node-prototype.md`: verbatim
  manifest and registration output, environment, 16 unit and 6 integration
  tests, schema validation, format, check, clippy, audit, deny and kaizen check.
- Documentation updated: README (EN+ES), `docs/INDEX.md`, alpha positioning,
  external prototype pointer and the crate README. The roadmap master and its
  Phase 1 box are unchanged: the gate stays open.
- `scripts/kaizen` now counts an empty debt grep as zero instead of aborting
  under `pipefail`; `.kaizen/config.env` wires format, lint and test commands.
  The baseline remains `debt_max: 0`.
- Explicitly not delivered: cryptographic identity, control-plane registration,
  container or VM provisioning, image export/import, volumes, replication,
  scheduling and the two-machine Phase 1 acceptance evidence. This is not a
  Fabric alpha.

### Alpha positioning (documentation only)

- `docs/releases/alpha-positioning.md` states honestly that Fabric is
  specification-only: it records what is specified, what is not implemented,
  the contract boundary with Anti-Gravital (ADR-0001 and Anti-Gravital
  ADR-0016), the L1/L2/L3 levels, the prototype scenarios a tester can run with
  Anti-Gravital alone, the scenarios that need Fabric code, a decision table,
  the Phase 1 preconditions and the condition for Fabric's own alpha.
- `docs/examples/anti-gravital-prototype.md` points testers to the external
  alpha walkthrough in the sibling Anti-Gravital repository and repeats that
  Fabric has no product implementation yet.
- `docs/INDEX.md` and the README status section link both documents; the
  bilingual status statement is unchanged: Fabric remains at specification
  stage.

### Fixes

- `.github/workflows/kaizen-loop.yml` no longer declares `variables: write`, an
  invalid `GITHUB_TOKEN` permission scope that made GitHub reject the whole
  file, register it under its path and fail on every push with zero jobs
  (issue #1). The pause mechanism is unaffected: `KAIZEN_PAUSED` is read and
  written through `secrets.KAIZEN_TOKEN`, not the workflow token.
- `scripts/kaizen check` now validates workflow `permissions` scopes against
  GitHub's documented allowlist (`--only workflows`), so this class of schema
  error is caught locally before push.

### Synchronization with Anti-Gravital (2026-11-22)

Changed:

- RFC-0004 section 4.5 now inherits the seven implementation prerequisites
  added to Anti-Gravital RFC-0024 on 2026-11-21. No MCP code is authorized
  until the consumer has machine-readable output, plan/apply, an audit sink,
  the `Secret<T>` contract and typed executors for every command group.
- `docs/INDEX.md` records the shared cross-repository documents and which
  Anti-Gravital revisions do not affect the Fabric boundary.

### Initial specification

Added (documentation only; no implementation):

- Three master documents: Blueprint v1.0, Arquitectura Tecnica and Hoja de
  Ruta, plus VERSION.md and a documentation index.
- RFC-0001 fixes the scope and the boundary with Anti-Gravital and the
  commercial catalog: open versioned contracts and no crate dependency between
  the open projects.
- RFC-0002 defines the Portable Workload and the L1/L2/L3 portability levels.
- RFC-0003 defines image, volume, snapshot and replication services and the
  durability contract for confirmed, in-transit and in-memory data.
- RFC-0004 defines the control plane, twelve engines, node autonomy and
  provider adapters, and separates replication, availability, migration,
  scaling and disaster recovery.
- RFC-0005 defines the functions runtime with WebAssembly capabilities and
  native and container adapters.
- ADR-0001 to ADR-0005 record the corresponding architectural decisions.
- Mermaid diagrams for the layered architecture and the migration flow.

### Governance

- RFC-0006 and ADR-0006 authorise a Kaizen governance layer subordinate to
  `CLAUDE.md`: `.kaizen/RULES.md`, `scripts/kaizen`, git hooks, PR descriptors,
  a quality ratchet, a metric series and CI checks.
- Content policies enforce the existing rules on commit length, emojis and
  AI-tool attribution. Generated rules and templates are bilingual with
  English canonical.

### Continuous loop

- RFC-0007 and ADR-0007 add an external Kaizen loop: versioned rules, five
  loop prompts, a journal, an autonomy framework with brakes, synthetic
  telemetry schemas and two CI workflows.
- The loop is installed paused at autonomy level 1. Label creation, tokens,
  secrets and branch protection remain human setup tasks.
- Telemetry aggregation is implemented as an offline Rust tool in
  `tools/kaizen-insights/`, keeping the Rust-only rule unchanged.
