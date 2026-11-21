---
description: Kaizen flow (start <branch> | check | done | metrics | retro)
argument-hint: start <branch> | check | done | metrics | retro
---
Read `CLAUDE.md` as the technical constitution and `.kaizen/RULES.md` as its
subordinate continuous-improvement adapter, then perform the requested action
in $ARGUMENTS:

- start <branch>: create the branch, copy
  `docs/kaizen/pr-drafts/_TEMPLATE.md` to
  `docs/kaizen/pr-drafts/<flat-branch>.md`, and fill in Summary, Affected
  component and Test plan from the issue.
- check: run `scripts/kaizen check` and summarise failures with a proposed fix.
- done: run `scripts/kaizen check`, confirm the descriptor is complete and
  current, that affected documentation was updated and that issue closing is
  correct. Report anything pending.
- metrics: run `scripts/kaizen metrics` and summarise the trend.
- retro: run `scripts/kaizen retro` and propose at most three improvements with
  suggested owner and issue.

Do not invent results: if a command was not run, say so.

Lee `CLAUDE.md` como constitucion tecnica y `.kaizen/RULES.md` como su adaptador
subordinado de mejora continua, y ejecuta la accion pedida en $ARGUMENTS. No
inventes resultados: si un comando no se ejecuto, dilo.
