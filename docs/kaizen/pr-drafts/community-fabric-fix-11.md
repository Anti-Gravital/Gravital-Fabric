## Resumen / Summary
Fix the concurrent register race: unique per-writer temporary files and atomic rename, with a 32-process regression test (issue #3).

## Componente afectado / Affected component
Phase 1 (partial), `fabric-node` local prototype; `write_state` in `crates/fabric-node/src/register.rs`, its integration tests, CHANGELOG and the user-journey audit.

## Tipo de cambio / Type of change
- [x] bug
- [ ] enhancement
- [ ] tech-debt
- [x] documentation
- [ ] refactor

## Documentos relacionados / Related documents
- `docs/audits/fabric-user-journey.md` (original reproduction kept; fix evidence added)
- `CHANGELOG.md` (new "Fix concurrent register race" entry)
- `docs/adr/0008-deterministic-local-node-identity.md` and `docs/rfc/RFC-0008-local-node-prototype.md` (prototype scope; JSON format unchanged, no new dependency)
- `docs/master/GRAVITAL-FABRIC-Hoja-de-Ruta.md` (Phase 1 gate stays open)
- Fabric issue #3 (concurrent register race; fixed and verified here)

## Plan de prueba / Test plan
- `cargo fmt --all -- --check`
- `cargo check --workspace --locked --offline`
- `cargo clippy --workspace --all-targets --locked --offline -- -D warnings`
- `cargo test --workspace --locked --offline` (16 unit + 7 integration tests, 0 failed)
- `cargo test --workspace --locked --offline --test cli concurrent_registers_to_one_state_path_all_succeed -- --exact` repeated 10 times: 10 passed, 0 failed
- Real binary, 5 trials of 32 concurrent `register` processes to one path: 0 failing invocations per trial (was 1-2 before the fix)
- `scripts/kaizen check` and `git diff --check`

## Impacto en calidad (ratchet) / Quality impact (ratchet)
- Coverage: up (new concurrent-process integration test; no coverage tool configured)
- Warnings: same (clippy `-D warnings` clean)
- Debt (TODO/FIXME): same (zero; baseline stays 0)
- Cobertura: sube (nuevo test de integracion concurrente; sin herramienta de cobertura)
- Advertencias: igual (clippy con `-D warnings` limpio)
- Deuda (TODO/FIXME): igual (cero; linea base sin cambios)

## Criterios de salida que avanza / Exit criteria advanced
- Concurrent registrations to one `--state` path all succeed; no spurious ENOENT and no caller loses its write.
- The final state document remains whole JSON and no temporary files are left behind.
- Advances no Phase 1 exit criterion: two machines, provisioning, cryptographic identity and the alpha label stay unmet.

## Aprendizaje (opcional) / Learning (optional)
The fixed temporary filename only failed under parallel use; `create_new` plus a unique name makes concurrent writers independent without any new dependency or lock.

## Cierre de issues / Issue closing
Closes #3

## Checklist / Checklist
- [x] Builds / Compila
- [x] Tests pass / Pasa tests (16 unit + 7 integration, regression 10/10)
- [x] `scripts/kaizen check` is green / en verde
- [x] Documentation updated / Documentacion actualizada (CHANGELOG + audit)
- [x] Baseline does not worsen / Linea base no empeora (debt 0 <= 0)
