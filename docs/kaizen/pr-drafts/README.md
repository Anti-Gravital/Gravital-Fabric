# Pull request descriptors

One versioned descriptor per working branch. The file name is the branch name
with slashes replaced by hyphens: branch `feat/node-agent` maps to
`feat-node-agent.md`.

| Rama | Descriptor |
| --- | --- |
| `feat/node-agent` | `feat-node-agent.md` |

Create it with `scripts/kaizen start <branch>` from `_TEMPLATE.md`, update it in
the same commit that changes scope, copy it into the PR body, and delete it in
the merge PR (rule K-4). `scripts/kaizen check` rejects a working branch with no
descriptor and a descriptor missing its required sections.

`_TEMPLATE.md` and this `README.md` are excluded from validation.

Cada rama de trabajo tiene un descriptor versionado: el nombre de la rama con
las barras sustituidas por guiones. Crealo con `scripts/kaizen start <rama>` a
partir de `_TEMPLATE.md`, actualizalo en el mismo commit que cambie el alcance,
copialo al cuerpo del PR y borralo en el PR de fusion (regla K-4).
`scripts/kaizen check` rechaza una rama sin descriptor o un descriptor incompleto.

`_TEMPLATE.md` y este `README.md` quedan excluidos de la validacion.
