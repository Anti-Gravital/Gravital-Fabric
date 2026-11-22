# DEV loop: one task from the approved backlog / Loop DEV: una tarea del backlog aprobado

1. Read `.kaizen/RULES.md`, `.kaizen/loop/LOOP.md` and the last 10 entries in
   `docs/kaizen/journal/`.
   Lee `.kaizen/RULES.md`, `.kaizen/loop/LOOP.md` y las ultimas 10 entradas de
   `docs/kaizen/journal/`.
2. Choose ONE open issue labelled `kaizen:ready` (highest priority, then
   oldest): `gh issue list --label kaizen:ready --json number,title,labels`.
   Without `gh`, take the first unchecked item in `docs/kaizen/BACKLOG.md`.
   If there is no work, write a "no work" journal entry and stop.
   Elige UN issue abierto con etiqueta `kaizen:ready` (mayor prioridad y luego
   el mas antiguo). Sin `gh`, toma el primer item sin marcar del BACKLOG. Si no
   hay trabajo, escribe la entrada de diario "sin trabajo" y termina.
3. Review the acceptance criteria. If they are ambiguous or not verifiable,
   comment on the issue with concrete questions, label it `needs-human` and
   stop.
   Revisa los criterios de aceptacion. Si son ambiguos o no verificables,
   comenta en el issue con preguntas concretas, etiquetalo `needs-human` y
   termina.
4. Write the test first (bug: failing test; feature: acceptance test).
   Escribe primero la prueba (bug: prueba que falla; feature: prueba de
   aceptacion).
5. Implement the minimum to pass it. Run format, lint and tests.
   Implementa lo minimo para pasarla. Ejecuta formato, lint y tests.
6. Update affected documentation and fill in the descriptor, including the
   quality-impact section, with `Closes #N`.
   Actualiza la documentacion afectada y rellena el descriptor, incluida la
   seccion de impacto en calidad, con `Closes #N`.
7. `scripts/kaizen check` must be green. Commit according to K-3.
   `scripts/kaizen check` debe terminar en verde. Haz commit segun K-3.
8. Write the journal entry (L-13) and stop. Do not open the PR; the
   orchestrator does that.
   Escribe la entrada de diario (L-13) y termina. No abras el PR: lo hace el
   orquestador.
