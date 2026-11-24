## Resumen / Summary
Documentar que KAIZEN_TOKEN debe configurarlo un humano y entregar pasos, workflow consumidor y verificacion para el issue #2.

## Componente afectado / Affected component
CI del bucle Kaizen: `.github/workflows/kaizen-loop.yml`, secretos del repositorio Gravital-Fabric.

## Tipo de cambio / Type of change
- [ ] bug
- [ ] enhancement
- [ ] tech-debt
- [x] documentation

## Documentos relacionados / Related documents
- Fabric issue #2 (https://github.com/Anti-Gravital/Gravital-Fabric/issues/2).
- `docs/kaizen/loop/SECURITY.md`, `docs/rfc/RFC-0007-kaizen-loop.md`.
- `docs/audits/fabric-issue-2-kaizen-token-needs-human.md`.

## Plan de prueba / Test plan
- `gh issue view 2 --repo Anti-Gravital/Gravital-Fabric` (cuerpo publico).
- `gh run view 44972846907 --repo Anti-Gravital/Gravital-Fabric` (run fallido).
- `git diff --check`
- `bash scripts/kaizen check`

## Impacto en calidad (ratchet) / Quality impact (ratchet)
- Coverage: same / igual (sin cambios de codigo)
- Warnings: same / igual
- Debt (TODO/FIXME): same / igual

## Criterios de salida que avanza / Exit criteria advanced
- Determinacion needs-human documentada con base en S-2 y L-3/L-4.
- Pasos humanos exactos, workflow consumidor y verificacion de run verde entregados.
- Declaracion explicita: ningun valor de secreto fue leido ni escrito por el agente.

## Aprendizaje (opcional) / Learning (optional)
Un secreto de CI es configuracion humana por diseno: el agente puede diagnosticar el fallo con metadatos publicos, pero no debe pedir, leer ni copiar el valor.

## Cierre de issues / Issue closing
Closes: none
Refs #2

## Checklist / Checklist
- [x] Builds / Compila (sin cambios de codigo)
- [x] Tests pass / Pasa tests (ver `scripts/kaizen check`)
- [ ] `scripts/kaizen check` is green / en verde (se reporta resultado real)
- [x] Documentation updated / Documentacion actualizada
- [x] Baseline does not worsen / Linea base no empeora
