## Resumen / Summary
Add a local-only fabric-node prototype: hardware discovery, stable local identity, JSON manifest and a register command writing a local state file.

## Componente afectado / Affected component
Phase 1 (partial), `fabric-node` engine, first Cargo workspace and first versioned contract under `contracts/`.

## Tipo de cambio / Type of change
- [x] enhancement
- [x] documentation
- [ ] bug
- [ ] tech-debt
- [ ] refactor

## Documentos relacionados / Related documents
- `docs/rfc/RFC-0008-local-node-prototype.md` (new, prototype scope)
- `docs/adr/0008-deterministic-local-node-identity.md` (new)
- `docs/rfc/RFC-0001-scope-and-boundaries.md`, `docs/rfc/RFC-0004-control-plane-and-adapters.md`
- `docs/adr/0001-no-compile-time-dependency-with-anti-gravital.md`, `docs/adr/0005-rust-only.md`
- `docs/master/GRAVITAL-FABRIC-Hoja-de-Ruta.md` Phase 1 (unchanged; gate stays open)
- `docs/releases/alpha-positioning.md`, `docs/INDEX.md`, `README.md`, `CHANGELOG.md`
- `scripts/kaizen`: fix the zero-debt pipefail abort exposed by the first `crates/` directory (RFC-0008 section 4.4)

## Plan de prueba / Test plan
- `cargo fmt --all -- --check`
- `cargo check --workspace --locked --offline`
- `cargo clippy --workspace --all-targets --locked --offline -- -D warnings`
- `cargo test --workspace --locked --offline`
- `cargo run -p fabric-node -- manifest` (real host, JSON on stdout)
- `cargo run -p fabric-node -- register --state /tmp/fabric-node-state.json`
- `scripts/kaizen check`, `git diff --check`, emoji and AI-attribution scans
- Evidence is recorded in `docs/audits/fabric-node-prototype.md`.

## Impacto en calidad (ratchet) / Quality impact (ratchet)
- Coverage: up (new unit and integration tests; no framework coverage tool configured)
- Warnings: same (clippy `-D warnings` enforced in kaizen config)
- Debt (TODO/FIXME): same (zero; baseline stays 0)
- Cobertura: sube (nuevos tests unitarios y de integracion; sin herramienta de cobertura)
- Advertencias: igual (clippy con `-D warnings`)
- Deuda (TODO/FIXME): igual (cero; linea base sin cambios)

## Criterios de salida que avanza / Exit criteria advanced
- Partial Phase 1: host discovery and a local registration artifact are demonstrated.
- NOT advanced to closure: two physical machines, container/VM provisioning,
  image export/import, control-plane registration and the Phase 1 gate remain unmet.

## Aprendizaje (opcional) / Learning (optional)
Repository rules require an RFC before the first crate; the prototype is documented
as partial so it cannot be read as Fabric reaching alpha.

## Cierre de issues / Issue closing
Closes: none

## Checklist / Checklist
- [ ] Builds / Compila
- [ ] Tests pass / Pasa tests
- [ ] `scripts/kaizen check` is green / en verde
- [ ] Documentation updated / Documentacion actualizada
- [ ] Baseline does not worsen / Linea base no empeora
