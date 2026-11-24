# Fabric issue #2 — KAIZEN_TOKEN: needs-human decision note

Repo: `Anti-Gravital/Gravital-Fabric`
Issue: https://github.com/Anti-Gravital/Gravital-Fabric/issues/2 (OPEN, sin comentarios)
Branch: `community29/fabric2`
Date: 2026-11-23 UTC

## Determinacion

Configurar el secreto `KAIZEN_TOKEN` NO puede hacerlo un agente.

- La regla S-2 de seguridad Kaizen prohibe leer o escribir credenciales
  (`.env`, claves, `~/.ssh`, almacenes de tokens, `env`/`printenv`).
- L-3/L-4 reservan secretos, autenticacion e infraestructura para humanos;
  el bucle ademas esta instalado en pausa, autonomia 1 (`.kaizen/loop/PAUSE`).
- El agente no posee ni debe usar un token con permiso de escritura en
  "Settings > Secrets and variables > Actions".

Por tanto NO se crea, edita, lee ni imprime ningun valor de secreto. Se entrega
esta nota con los pasos exactos para un humano y se deja el issue #2 abierto
(`Refs #2`, no `Closes #2`).

## Evidencia de la necesidad (solo fuentes publicas y metadatos de run)

Issue #2 (cuerpo publico, leido con `gh issue view 2`):
- Run disparado `44972846907`, job `iterate`, conclusion `failure`.
- Log: `##[error]Input required and not supplied: token` en `gh auth setup-git`.
- El workflow pasa `GH_TOKEN: ${{ secrets.KAIZEN_TOKEN }}`; el secreto no existe.
- "Done when": `KAIZEN_TOKEN` con `contents: write`, `pull-requests: write`,
  `issues: write`, `actions: read`; run disparado que alcance `Loop iteration`;
  alcance documentado en `docs/kaizen/loop/SECURITY.md` sin publicar el valor.

Verificacion propia (lectura publica, sin secretos):
```
$ gh run view 44972846907 --repo Anti-Gravital/Gravital-Fabric
X main kaizen-loop · 44972846907
X iterate in 3s (ID 137056176091)
  X Run actions/checkout@v4
  - Configure git credentials
  - Telemetry to insights (optional, read-only)
  - Loop iteration
X Input required and not supplied: token
```
Fallo real en `Run actions/checkout@v4` (linea 17: `token:` vacio); el cuerpo del
issue ademas reporta el fallo posterior en `gh auth setup-git`. Misma causa:
`secrets.KAIZEN_TOKEN` no esta configurado.

## (1) Pasos exactos para el humano en GitHub Settings

1. Abrir https://github.com/Anti-Gravital/Gravital-Fabric
2. `Settings` -> `Secrets and variables` -> `Actions` -> `New repository secret`.
3. En `Name`, escribir exactamente: `KAIZEN_TOKEN` (sensible a mayusculas).
4. En `Secret`, pegar el valor aprovisionado por una persona:
   - GitHub App instalada en el repo, o PAT de granularidad fina; NO reutilizar
     un token personal amplio (RFC-0007 seccion Risks, `SECURITY.md`).
   - Permisos minimos de la App/PAT sobre ESTE repositorio:
     `Contents: Read and write`, `Pull requests: Read and write`,
     `Issues: Read and write`, `Actions: Read-only`
     (equivale a `contents: write`, `pull-requests: write`, `issues: write`,
     `actions: read`). Mas `Metadata: Read-only` si lo exige la PAT.
   - Los PRs que cree el token deben disparar CI.
5. `Add secret`.
6. Rotacion: definir responsable y fecha de caducidad; al rotar, repetir 2-5.
7. NUNCA pegar el valor en issues, PRs, logs, chat, descriptor ni documentacion.
   Solo el NOMBRE `KAIZEN_TOKEN` puede aparecer en texto versionado.

Nota: `docs/kaizen/loop/SECURITY.md` ya documenta el requisito de alcance minimo
sin publicar valores; no hace falta anadir el valor a ningun archivo.

## (2) Workflow que consume el secreto

`.github/workflows/kaizen-loop.yml` (disparadores: `schedule` cron `17 */3 * * *`
y `workflow_dispatch` con input opcional `loop`):

- `:29` `actions/checkout@v4` -> `token: ${{ secrets.KAIZEN_TOKEN }}`
- `:34` step `Configure git credentials` -> `GH_TOKEN: ${{ secrets.KAIZEN_TOKEN }}`
- `:42` step `Telemetry to insights (optional, read-only)` -> `GH_TOKEN`
- `:48` step `Loop iteration` -> `GH_TOKEN` (ademas keys de modelo)

Consumidor secundario: `.github/workflows/kaizen-automerge.yml:24` -> `GH_TOKEN`.

El bloque `permissions:` del workflow ya declara los cuatro scopes, pero los
steps usan explicitamente el secreto; por eso el secreto debe existir.

No se modifica ningun workflow: `.github/` es ruta protegida (L-4) y el
workflow es correcto segun el propio issue.

## (3) Verificacion exacta (humano)

Comprobar el nombre del secreto (NUNCA su valor):
```
gh secret list --repo Anti-Gravital/Gravital-Fabric | grep -F KAIZEN_TOKEN
```

Disparar y esperar un run verde:
```
gh workflow run kaizen-loop --repo Anti-Gravital/Gravital-Fabric [--field loop dev]
gh run list --workflow kaizen-loop --repo Anti-Gravital/Gravital-Fabric --limit 1
gh run view <RUN_ID> --repo Anti-Gravital/Gravital-Fabric
gh run view <RUN_ID> --job <JOB_ID> --log --repo Anti-Gravital/Gravital-Fabric
```

Criterio de aceptacion (todo debe cumplirse):
- `gh run view <RUN_ID>` -> conclusion `success` (no `failure`).
- Job `iterate` en verde; el step `Loop iteration` aparece ejecutado con un
  resultado real (no `-`, no omitido), sin `Input required and not supplied: token`.
- El run no queda en `Configure git credentials` ni en `Run actions/checkout@v4`.
- `gh secret list` muestra `KAIZEN_TOKEN` (solo nombre).
- Se registra el numero de run verde como evidencia de cierre del issue #2.

Comparacion: run fallido de referencia `44972846907` (conclusion `failure`).
Si el nuevo run es verde, el humano cierra el issue #2 con esa evidencia.

## (4) Declaracion de no acceso a secretos

El agente NO leyo, escribio, imprimio, copio ni transmitio ningun valor de
secreto. No se accedio a `.env`, claves privadas, `~/.ssh`, almacenes de tokens
ni variables de entorno; no se ejecuto `env`/`printenv`. Toda la evidencia
proviene del cuerpo publico del issue #2, del codigo fuente versionado de los
workflows y de metadatos de solo lectura de GitHub (`gh issue view`,
`gh run view`). Solo se uso el NOMBRE `KAIZEN_TOKEN` como texto.

## Cierre

Este cambio NO cierra el issue #2: `Refs #2` / `Closes: none`. El issue permanece
abierto hasta que un humano configure el secreto y documente el run verde.
Sin push, PR ni merge por parte del agente.
