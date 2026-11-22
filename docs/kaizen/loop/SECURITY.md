# Human security setup / Configuracion de seguridad humana

The loop is installed paused at autonomy level 1. None of the following was
performed by the installer. A person must decide and complete every item
before `PAUSE` is removed.

El loop queda instalado en pausa, con autonomia 1. El instalador no realizo
ninguno de los siguientes pasos. Una persona debe decidirlos y completarlos
antes de retirar `PAUSE`.

## Checklist

- [ ] Default branch protection: require a PR, `kaizen-gates` checks and
      human reviews. Nobody, including the bot, pushes directly.
      Proteccion de la rama por defecto: exigir PR, checks `kaizen-gates` y
      revision humana. Nadie, ni el bot, hace push directo.
- [ ] `CODEOWNERS` with human owners for protected paths.
      `CODEOWNERS` con responsables humanos para las rutas protegidas.
- [ ] Auto-merge enabled in repository settings only if autonomy 2 or 3 is
      ever approved. Auto-merge habilitado solo si se aprueba el nivel 2 o 3.
- [ ] `KAIZEN_TOKEN`: GitHub App or fine-grained PAT with minimum scopes
      (contents, pull requests, issues). The PRs it creates must trigger CI.
      Do not reuse a personal token with broad scopes.
      `KAIZEN_TOKEN`: GitHub App o PAT de granularidad fina con permisos
      minimos. No reutilices un token personal con permisos amplios.
- [ ] Model API secret stored in the CI secret manager, plus a spend limit at
      the provider. No production write credentials for the loop.
      Secreto del modelo en el gestor del CI, con limite de gasto. Sin
      credenciales de produccion con escritura.
- [ ] Telemetry store is read-only for the loop, retention period defined in
      `docs/kaizen/telemetry/PRIVACY.md`, and privacy/legal review recorded.
      Almacen de telemetria de solo lectura, retencion definida en
      `docs/kaizen/telemetry/PRIVACY.md` y revision de privacidad registrada.
- [ ] Raising autonomy requires an ADR signed by a person.
      Subir la autonomia exige un ADR firmado por una persona.
- [ ] Shell permission restrictions in `opencode.json` are reviewed and added
      after the chosen agent and command set are confirmed.
      Las restricciones de permisos en `opencode.json` se revisan y anaden
      despues de confirmar el agente y los comandos.

Run `scripts/kaizen-loop status` to confirm the loop is paused.
Ejecuta `scripts/kaizen-loop status` para confirmar que el loop esta en pausa.
