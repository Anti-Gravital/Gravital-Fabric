# Documentation index

## Source of truth

- `docs/master/GRAVITAL-FABRIC-Blueprint-v1.0.md` — vision, positioning, scope.
- `docs/master/GRAVITAL-FABRIC-Hoja-de-Ruta.md` — what and when.
- `docs/master/GRAVITAL-FABRIC-Arquitectura-Tecnica.md` — how.
- `docs/master/VERSION.md` — priority and version policy.

## Decisions

- `docs/rfc/` — proposals, with status and acceptance criteria.
- `docs/adr/` — accepted architectural decisions.

## Governance

- `.kaizen/RULES.md` — continuous-improvement rules, subordinate to
  `CLAUDE.md`.
- `docs/kaizen/` — descriptors, retrospectives, postmortems, runbooks and
  metric series.
- `scripts/kaizen` and `.githooks/` — local checks and hooks.
- RFC-0006 and ADR-0006 authorise the governance CI and hooks.
- RFC-0007 and ADR-0007 authorise the external loop and the offline Rust
  insights tool in `tools/kaizen-insights/`.
- `docs/kaizen/loop/` — loop rules, configuration, prompts and security
  checklist. `docs/kaizen/journal/` — one entry per iteration.

## Cross-repository synchronization

Fabric and Anti-Gravital share contracts as data, never as Rust dependencies.
The following Anti-Gravital documents are referenced by this specification:

- `ADR-0016` — open interfaces between Anti-Gravital and Gravital Fabric.
- `RFC-0024` (amended 2026-11-21) — shared capability-scoped MCP interface and
  its seven implementation prerequisites; inherited by RFC-0004 section 4.5.
- `ADR-0017` — MCP does not grant privilege by virtue of being a machine.

Anti-Gravital revisions that do not change the Fabric boundary (RFC-0025
ag-data ORM, RFC-0026 chrono replacement, RFC-0027 chumsky upgrade, ADR-0018
reqwest provider and ADR-0019 OTLP transport) are tracked in that repository
only. Last synchronization: 2026-11-22.

## Releases and prototypes

- `docs/releases/alpha-positioning.md` — what Fabric specifies, what is not
  implemented, the Anti-Gravital contract boundary, the three portability
  levels, tester decision table and Phase 1 preconditions.
- `docs/examples/anti-gravital-prototype.md` — points testers to the external
  Anti-Gravital alpha walkthrough and states that Fabric is specification-only.

These documents do not change the boundary in RFC-0001 or ADR-0001; they make
the existing boundary explicit for alpha testers.

## Design artifacts

- `docs/diagrams/` — Mermaid diagrams of layers, data flow and migration.
- `contracts/` — versioned open contracts (to be created with the first
  implementation RFC).

## Language

English is canonical. A Spanish mirror may follow the English section in
showcase documents; when the two differ, English governs.
