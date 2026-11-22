## Resumen / Summary

Remove the invalid `variables` permission scope so GitHub parses kaizen-loop and registers its declared name.

Elimina el permiso `variables`, invalido, para que GitHub parsee kaizen-loop y registre su nombre declarado.

## Componente afectado / Affected component

`.github/workflows/kaizen-loop.yml`, `scripts/kaizen`, external Kaizen loop (RFC-0007 / ADR-0007).

## Tipo de cambio / Type of change

- [x] bug
- [ ] enhancement
- [ ] tech-debt
- [ ] documentation
- [ ] refactor

## Documentos relacionados / Related documents

- Issue #1: kaizen-loop workflow fails validation on every run.
- RFC-0007 / ADR-0007 (external loop), `.kaizen/RULES.md` (K-2, K-7, K-10), `docs/kaizen/loop/SECURITY.md`.
- CHANGELOG.md (entry added in this change).
- GitHub Actions workflow syntax reference, `permissions` table (authoritative scope list).

Root cause: `permissions.variables: write` is not a valid `GITHUB_TOKEN` permission scope. GitHub rejects the entire workflow file, registers it under the file path instead of `name: kaizen-loop`, and emits a zero-job failure on every push with `created_at == updated_at`. YAML parsers accept the key, which is why `scripts/kaizen` and the gates did not catch it. The defect dates to commit `3d5dc01`, so it predates the 2026-11-22 sync.

The loop sets `KAIZEN_PAUSED` through `gh variable set` using `secrets.KAIZEN_TOKEN` (see `scripts/kaizen-loop`), not the implicit `GITHUB_TOKEN` governed by the workflow `permissions:` block. Removing the invalid scope therefore does not weaken the pause mechanism or any loop security rule. Triggers remain `schedule` and `workflow_dispatch`; no `push` trigger was added.

## Plan de prueba / Test plan

- `scripts/kaizen check` (all checks, including the new `workflows` mode).
- `scripts/kaizen check --only workflows` (passes on the fixed file).
- Negative test: reintroduce `variables: write` in a temporary copy and confirm `--only workflows` exits 1 with `invalid GITHUB_TOKEN permission scope 'variables'`.
- `actionlint .github/workflows/kaizen-loop.yml` (clean; both v1.7.7 and v1.7.9 flagged only the removed line).
- Python `yaml.safe_load` of every workflow file (parses correctly).
- `git diff --check` and the emoji / AI-attribution content policy via `scripts/kaizen check --only policy`.

Coordinator verification after merge to `main` (cannot be run from this worktree: branch is not the default branch and the dispatch needs secrets):

- `gh api repos/Anti-Gravital/Gravital-Fabric/actions/workflows --jq '.workflows[]|select(.path==".github/workflows/kaizen-loop.yml")|{name,path,state}'` must report `name == "kaizen-loop"`.
- Dispatch the workflow (`gh workflow run kaizen-loop`) and confirm the run reaches the `iterate` job with a real conclusion; confirm no zero-job `push` failures remain.

## Impacto en calidad (ratchet) / Quality impact (ratchet)

- Coverage: same (not measured; no source tree).
- Warnings: same.
- Debt (TODO/FIXME): same.
- Cobertura: igual. Advertencias: igual. Deuda: igual.

## Criterios de salida que avanza / Exit criteria advanced

- GitHub shows the workflow name as `kaizen-loop`: root cause removed; VERIFY AFTER MERGE on the default branch.
- A manual `workflow_dispatch` run reaches `iterate`: BLOCKED on coordinator verification with secrets.
- No spurious `push`-event failure remains: root cause removed; VERIFY AFTER MERGE.
- Local regression guard added: DONE (`scripts/kaizen check --only workflows`).

## Aprendizaje (opcional) / Learning (optional)

A syntactically valid YAML key can still be rejected by the GitHub Actions schema. Local YAML parsing is not sufficient for workflow validation; the permission allowlist check closes that gap. `actionlint` is the comprehensive tool and remains the recommended manual check.

## Cierre de issues / Issue closing

Closes: none
Refs #1

Issue #1 stays open on purpose: the parse fix is verified locally, but the issue requires a `workflow_dispatch` run reaching `iterate` with real secrets on the default branch, which only the coordinator can perform after merge. Do not close #1 until that evidence exists (rules K-2 and K-10).

## Checklist / Checklist

- [x] Builds / Compila (no build system at specification stage)
- [x] Tests pass / Pasa tests (`scripts/kaizen check` green)
- [x] `scripts/kaizen check` is green / en verde
- [x] Documentation updated / Documentacion actualizada (CHANGELOG.md)
- [x] Baseline does not worsen / Linea base no empeora
