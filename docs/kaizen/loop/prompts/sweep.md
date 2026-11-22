# SWEEP loop: fighting entropy / Loop SWEEP: combatir la entropia

Choose ONE item from the list, the one longest since it was last run according
to the journal: Elige UN item de la lista, el que lleve mas tiempo sin
ejecutarse segun el diario:

1. Dependencies: patch and minor updates with green tests; open an issue for
   major ones. Dependencias: actualizaciones de parche y menores con tests
   verdes; abre issue para las mayores.
2. Debt: turn TODO/FIXME/HACK older than 90 days into issues with context, or
   remove them if they no longer apply. Deuda: convierte en issues los
   TODO/FIXME/HACK con mas de 90 dias, o eliminarlos si ya no aplican.
3. Documentation drift: compare README and docs with actual code behaviour
   and fix it. Desviacion documental: compara README y docs con el
   comportamiento real del codigo y corrige.
4. Flaky tests: repeat the suite three times, open issues or stabilise them
   without skipping. Pruebas inestables: repite la suite 3 veces, abre issues
   o estabilizalas sin saltarlas.
5. Dead and duplicated code: remove only what is verified unused, with green
   tests. Codigo muerto y duplicado: elimina lo comprobadamente sin uso, con
   tests verdes.
6. Architectural fitness: check the rules in `docs/kaizen/fitness.md` and
   report violations. Aptitud arquitectonica: verifica las reglas de
   `docs/kaizen/fitness.md` y reporta violaciones.
7. Security and licences: run the project audit (`cargo audit`, `cargo deny`)
   and open issues. Seguridad y licencias: ejecuta `cargo audit` y
   `cargo deny` y abre issues.
8. Orphan descriptors: delete `docs/kaizen/pr-drafts/*.md` whose branch no
   longer exists or whose PR is merged. Descriptores huérfanos: elimina los
   cuya rama ya no existe o cuyo PR esta fusionado.
9. Retro: run `scripts/kaizen retro` and draft `docs/kaizen/retros/` with up
   to three proposed improvements; a person decides. Retro: ejecuta
   `scripts/kaizen retro` y redacta la retro con hasta 3 mejoras propuestas;
   decide el humano.
10. Baseline: if measured quality improved, prepare the
    `.kaizen/baseline.json` change as a PR requiring human review. Linea base:
    si la calidad mejoro, prepara el cambio como PR con revision humana.

Rules: small diff, green tests, green `scripts/kaizen check`, journal at the
end. Reglas: diff pequeno, tests verdes, check verde, diario al final.
