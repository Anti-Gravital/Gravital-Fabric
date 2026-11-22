# Anti-Gravital alpha prototype (external walkthrough)

[English](#english) | [Espanol](#espanol)

## English

**Gravital Fabric is specification-only.** It has no product implementation,
no node agent and no published contract yet. The runnable alpha prototype lives
in the sibling [Anti-Gravital](https://github.com/Anti-Gravital/Anti-Gravital)
repository. This page only points to it; it is not a Fabric walkthrough.

Start with the alpha "Try it" or quickstart section of the Anti-Gravital README
(English is canonical), then the release notes for version `0.1.0-alpha.1`:

- [Anti-Gravital README](https://github.com/Anti-Gravital/Anti-Gravital#readme)
- [Release notes 0.1.0-alpha.1](https://github.com/Anti-Gravital/Anti-Gravital/blob/main/docs/releases/0.1.0-alpha.1.md)
- [Examples index](https://github.com/Anti-Gravital/Anti-Gravital/tree/main/docs/examples)

The core of that walkthrough is:

```
cargo install --path crates/ag-cli --locked
ag new my-api --template rest
cd my-api
ag schema lint --schema schema.ag
ag generate --schema schema.ag --output generated
ag dev
```

This demonstrates a scaffolded local service, generated artifacts and a loopback
endpoint (`127.0.0.1:8080` by default). It does **not** demonstrate node
discovery, provisioning, scheduling or migration. Those require Fabric code and
are not available yet; see
[Alpha positioning and prototype guide](../releases/alpha-positioning.md) for
the exact boundary, the decision table and the Phase 1 preconditions.

## Espanol

**Gravital Fabric esta en fase de especificacion.** No tiene implementacion de
producto, ni agente de nodo, ni contratos publicados. El prototipo alpha
ejecutable esta en el repositorio hermano
[Anti-Gravital](https://github.com/Anti-Gravital/Anti-Gravital). Esta pagina solo
senala hacia el; no es un recorrido de Fabric.

Comience con la seccion "Try it" o inicio rapido del README de Anti-Gravital
(el ingles es canonico) y luego con las notas de la version `0.1.0-alpha.1`:

- [README de Anti-Gravital](https://github.com/Anti-Gravital/Anti-Gravital#readme)
- [Notas de 0.1.0-alpha.1](https://github.com/Anti-Gravital/Anti-Gravital/blob/main/docs/releases/0.1.0-alpha.1.md)
- [Indice de ejemplos](https://github.com/Anti-Gravital/Anti-Gravital/tree/main/docs/examples)

El nucleo del recorrido es:

```
cargo install --path crates/ag-cli --locked
ag new my-api --template rest
cd my-api
ag schema lint --schema schema.ag
ag generate --schema schema.ag --output generated
ag dev
```

Esto demuestra un servicio local generado a partir de una plantilla, artefactos
generados y un endpoint en loopback (`127.0.0.1:8080` por defecto). No demuestra
descubrimiento de nodos, aprovisionamiento, planificacion ni migracion. Eso
requiere codigo de Fabric y aun no existe; consulte
[Posicionamiento alpha y guia de prototipo](../releases/alpha-positioning.md)
para el limite exacto, la tabla de decisiones y los requisitos de la Fase 1.
