# Kaizen Rules

These rules are a subordinate adapter of `CLAUDE.md`, which remains the
technical constitution of this repository. Where this file and `CLAUDE.md`
disagree, `CLAUDE.md` governs. Every agent adapter points here so the rules are
defined once.

---

## English

### K-1 Source of truth

`docs/master/`, `docs/rfc/` and `docs/adr/` are the source of truth and govern
the code. `CLAUDE.md` is the technical constitution. No architectural decision
without an ADR; no new crate, engine, contract change, security change,
performance target, roadmap change or CI change without an RFC.

### K-2 One issue board

Every issue has a type (bug, enhancement, tech-debt, documentation), an area,
a priority and verifiable acceptance criteria with concrete steps. An issue is
not closed without evidence: a test, a reproducible command or green CI. If it
becomes blocked, it stays open with the `blocked` label and a reason. It is
closed as soon as the fix is verified.

### K-3 Commits

- One-line summary of at most 256 characters, in the imperative mood.
- Includes `Closes #NNN`, `Fixes #NNN` or `Resolves #NNN` when applicable.
- One commit is one logical change that builds and passes tests.
- Optional policies (see `.kaizen/config.env`): no emojis, no AI-tool
  attribution. Both are active in this repository, as required by `CLAUDE.md`
  section 7.

### K-4 Workflow (PDCA)

1. Plan: read the issue, define exit criteria and a test plan. Create a branch
   and a descriptor at `docs/kaizen/pr-drafts/<flat-branch>.md` (`/` becomes
   `-`).
2. Do: implement in small commits. Update the descriptor in the same commit
   that changes scope.
3. Verify: run `scripts/kaizen check`. Everything must pass.
4. Act: open a PR (the body is copied from the descriptor). On merge, delete
   the descriptor in that same PR. Record what was learned if there were
   surprises.

No descriptor, no PR. Direct commits to the default branch are allowed only
with an explicit exception from the repository owner, and then every commit
carries its closing keyword.

### K-5 Quality ratchet

`.kaizen/baseline.json` stores the quality metrics. A change may not worsen
them: coverage does not decrease, lint warnings do not increase, and marked
debt (TODO/FIXME/HACK) does not increase without an associated issue. To move
the baseline up, run `scripts/kaizen baseline`. To accept a regression, record
an ADR or a note in the descriptor with justification and human approval. The
agent never relaxes the baseline on its own.

### K-6 Data collection

`scripts/kaizen metrics` appends one line to `docs/kaizen/data/metrics.jsonl`
per run (date, commit, size, debt, coverage, warnings, open issues). It runs in
CI on merge to the default branch and before every retrospective. The data is
used to make decisions, not to monitor people.

### K-7 Living documentation

Every behavior change updates the affected README, user documentation, ADR or
runbook and the CHANGELOG. The descriptor lists the documents touched. A change
without documentation is not complete.

### K-8 Operations

- Every service or recurring operation has a runbook in
  `docs/kaizen/runbooks/`.
- Every incident produces a blameless postmortem in
  `docs/kaizen/postmortems/` within five days, with root cause (five whys), a
  timeline and preventive actions converted into issues.
- Noisy alerts are fixed or removed, not ignored.

### K-9 Retrospective

Every two weeks (or per milestone), `scripts/kaizen retro` generates the
summary with metrics. The team decides on at most three improvements, each
with an owner, an issue and a date. The next retrospective reviews whether
they worked.

### K-10 Agent limits

- Do not fabricate evidence: if a test was not run, say so.
- Do not close issues without verification.
- Do not bypass hooks (`--no-verify`) or disable CI to make something pass.
- If a rule conflicts with a user request, inform the user and ask for a
  decision.
- The PR description is written by a person or by the agent in the descriptor;
  automation only copies and validates it.

---

## Espanol

Estas reglas son un adaptador subordinado de `CLAUDE.md`, que sigue siendo la
constitucion tecnica de este repositorio. Si este archivo y `CLAUDE.md`
discrepan, manda `CLAUDE.md`. Todos los adaptadores apuntan aqui para que las
reglas esten definidas una sola vez.

### K-1 Fuente de verdad

`docs/master/`, `docs/rfc/` y `docs/adr/` son la fuente de verdad y mandan sobre
el codigo. `CLAUDE.md` es la constitucion tecnica. Ninguna decision de
arquitectura sin ADR; ningun crate, motor, cambio de contrato, cambio de
seguridad, objetivo de rendimiento, cambio de hoja de ruta o cambio de CI sin
RFC.

### K-2 Un tablero de issues

Cada issue lleva tipo (bug, enhancement, tech-debt, documentation), area,
prioridad y criterios de aceptacion verificables con pasos concretos. No se
cierra sin evidencia: test, comando reproducible o CI verde. Si queda
bloqueado, permanece abierto con la etiqueta `blocked` y el motivo. Se cierra en
cuanto se verifica.

### K-3 Commits

- Una linea de resumen de maximo 256 caracteres, en imperativo.
- Incluye `Closes #NNN`, `Fixes #NNN` o `Resolves #NNN` cuando corresponda.
- Un commit es un cambio logico, que compila y pasa tests.
- Politicas opcionales (ver `.kaizen/config.env`): sin emojis, sin atribucion a
  herramientas de IA. Ambas estan activas en este repositorio, segun la seccion
  7 de `CLAUDE.md`.

### K-4 Flujo de trabajo (PDCA)

1. Planear: leer el issue, definir criterios de salida y plan de prueba. Crear
   rama y descriptor `docs/kaizen/pr-drafts/<rama-aplanada>.md` (`/` se convierte
   en `-`).
2. Hacer: implementar en cambios pequenos. Actualizar el descriptor en el mismo
   commit que cambie el alcance.
3. Verificar: ejecutar `scripts/kaizen check`. Todo debe pasar.
4. Actuar: abrir PR (el cuerpo se copia del descriptor). Al fusionar, borrar el
   descriptor en esa misma PR. Registrar lo aprendido si hubo sorpresas.

Sin descriptor no hay PR. Los commits directos a la rama por defecto solo se
permiten con excepcion explicita del responsable del repositorio, y entonces
cada commit lleva su keyword de cierre.

### K-5 Ratchet de calidad

`.kaizen/baseline.json` guarda las metricas de calidad. Un cambio no puede
empeorarlas: la cobertura no baja, las advertencias de lint no suben y la deuda
marcada (TODO/FIXME/HACK) no sube sin issue asociado. Para mover la linea base
hacia arriba: `scripts/kaizen baseline`. Para aceptar un empeoramiento: ADR o
nota en el descriptor con justificacion y aprobacion humana. El agente nunca
relaja la linea base por su cuenta.

### K-6 Recoleccion de datos

`scripts/kaizen metrics` agrega una linea a `docs/kaizen/data/metrics.jsonl` por
ejecucion (fecha, commit, tamano, deuda, cobertura, advertencias, issues
abiertos). Se ejecuta en CI al fusionar a la rama por defecto y antes de cada
retrospectiva. Los datos se usan para decidir, no para vigilar a personas.

### K-7 Documentacion viva

Todo cambio de comportamiento actualiza el README, la documentacion de usuario,
el ADR o el runbook afectado y el CHANGELOG. El descriptor lista los documentos
tocados. Un cambio sin documentacion no esta terminado.

### K-8 Operaciones

- Cada servicio u operacion recurrente tiene runbook en
  `docs/kaizen/runbooks/`.
- Todo incidente genera postmortem sin culpables en
  `docs/kaizen/postmortems/` en un maximo de 5 dias, con causa raiz (5
  porques), linea de tiempo y acciones preventivas convertidas en issues.
- Las alertas ruidosas se corrigen o se eliminan; no se ignoran.

### K-9 Retrospectiva

Cada 2 semanas (o por hito): `scripts/kaizen retro` genera el resumen con
metricas. El equipo decide como maximo 3 mejoras, cada una con dueno, issue y
fecha. En la siguiente retrospectiva se revisa si funcionaron.

### K-10 Limites del agente

- No fabricar evidencia: si un test no se ejecuto, se dice.
- No cerrar issues sin verificacion.
- No saltarse hooks (`--no-verify`) ni desactivar CI para que algo pase.
- Ante conflicto entre una regla y una peticion del usuario, informar al
  usuario y pedir decision.
- La descripcion del PR la redacta una persona o el agente en el descriptor; la
  automatizacion solo copia y valida.
