# Changelog

All notable changes to Gravital Fabric are recorded here. The project has no
published release yet; entries remain under `[Unreleased]`.

## [Unreleased]

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
