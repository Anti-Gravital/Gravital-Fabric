## Resumen / Summary
Add a real-user terminal journey audit for fabric-node with timings, schema checks and a reproducible concurrency defect (issue #3).

## Componente afectado / Affected component
Phase 1 (partial), `fabric-node` local prototype; documentation, audits and governance only. No source code changed.

## Tipo de cambio / Type of change
- [ ] bug
- [ ] enhancement
- [ ] tech-debt
- [x] documentation
- [ ] refactor

## Documentos relacionados / Related documents
- `docs/audits/fabric-user-journey.md` (new: real-terminal evidence)
- `docs/audits/fabric-node-prototype.md` (first host evidence, unchanged)
- `docs/releases/alpha-positioning.md`, `docs/INDEX.md`, `CHANGELOG.md`
- `crates/fabric-node/README.md`, `contracts/fabric-node-manifest/v1.schema.json`
- `docs/master/GRAVITAL-FABRIC-Hoja-de-Ruta.md` (Phase 1 gate remains open)
- Fabric issue #3 (reported concurrency defect; not fixed here)

## Plan de prueba / Test plan
- `cargo build -p fabric-node --offline` in a private target dir (cold: 1.74 s)
- `cargo run -p fabric-node --offline -- manifest` (0.39 s; byte-identical on re-run)
- `cargo run -p fabric-node --offline -- register --state <tmp>` (0.15 s; timestamp changes on re-run)
- `cargo run -p fabric-node -- --version`, `--help`, no args and documented error paths
- `python3 jsonschema.validate` on manifest stdout and nested state manifest: valid
- 24 to 32 parallel `register` calls to one path: 1-2 spurious ENOENT failures (issue #3)
- `scripts/kaizen check`, `git diff --check`, emoji and AI-attribution scans

## Impacto en calidad (ratchet) / Quality impact (ratchet)
- Coverage: same (no source changes; audit executed the existing binary and tests)
- Warnings: same (clippy `-D warnings` re-run below)
- Debt (TODO/FIXME): same (zero; baseline stays 0)
- Cobertura: igual (sin cambios de codigo)
- Advertencias: igual (clippy con `-D warnings`)
- Deuda (TODO/FIXME): igual (cero; linea base sin cambios)

## Criterios de salida que avanza / Exit criteria advanced
- Records what a real user can and cannot do today with the prototype.
- Advances no Phase 1 exit criterion: two machines, provisioning, identity and alpha label stay unmet.

## Aprendizaje (opcional) / Learning (optional)
A local /tmp tmpfs quota forced a disk-backed private target; the fixed temp filename in register is a real concurrency defect that only appears under parallel use.

## Cierre de issues / Issue closing
Closes: none
Refs #3 (reported, not fixed; no issue closed)

## Checklist / Checklist
- [x] Builds / Compila
- [x] Tests pass / Pasa tests
- [x] `scripts/kaizen check` is green / en verde
- [x] Documentation updated / Documentacion actualizada
- [x] Baseline does not worsen / Linea base no empeora
