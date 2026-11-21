# Gravital Fabric

## Kaizen

`CLAUDE.md` is the technical constitution of this repository.
`.kaizen/RULES.md` is a subordinate continuous-improvement adapter; if they
conflict, `CLAUDE.md` governs.

Workflow: branch plus descriptor in `docs/kaizen/pr-drafts/` -> small commits
-> `scripts/kaizen check` -> PR.

Commands: `scripts/kaizen start <branch>`, `check`, `baseline`, `metrics`,
`retro`. Never use `--no-verify`, never relax `.kaizen/baseline.json` without
human approval, and never claim a test passed without running it.

## Kaizen (Espanol)

`CLAUDE.md` es la constitucion tecnica de este repositorio.
`.kaizen/RULES.md` es un adaptador subordinado de mejora continua; si discrepan,
manda `CLAUDE.md`.

Flujo: rama mas descriptor en `docs/kaizen/pr-drafts/` -> cambios pequenos ->
`scripts/kaizen check` -> PR.

Comandos: `scripts/kaizen start <rama>`, `check`, `baseline`, `metrics`, `retro`.
Nunca uses `--no-verify`, nunca relajes `.kaizen/baseline.json` sin aprobacion
humana y nunca declares un test como pasado sin haberlo ejecutado.
