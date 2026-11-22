# Anti-Gravital alpha prototype (external walkthrough)

[English](#english) | [Espanol](#espanol)

## English

**Gravital Fabric is at specification stage with one local-only prototype.**
It has no product implementation, no installable node agent and no control
plane. The `fabric-node` prototype crate discovers local hardware and writes a
local JSON manifest or state file; see
[the prototype evidence](../audits/fabric-node-prototype.md). The runnable API
alpha prototype lives in the sibling
[Anti-Gravital](https://github.com/Anti-Gravital/Anti-Gravital) repository. This
page points to it; it is not a Fabric walkthrough.

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
endpoint (`127.0.0.1:8080` by default). It does **not** demonstrate Fabric
provisioning, scheduling or migration. Fabric's own local discovery demo is
`cargo run -p fabric-node -- manifest` from the Fabric checkout; it writes a
manifest but provisions nothing. See
[Alpha positioning and prototype guide](../releases/alpha-positioning.md) for
the exact boundary, the decision table and the Phase 1 preconditions.

## Espanol

**Gravital Fabric esta en fase de especificacion con un prototipo local.** No
tiene implementacion de producto, ni agente instalable, ni plano de control. El
crate `fabric-node` descubre hardware local y escribe un manifiesto JSON o un
archivo de estado local; consulte
[la evidencia del prototipo](../audits/fabric-node-prototype.md). El prototipo
alpha ejecutable de API esta en el repositorio hermano
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
aprovisionamiento, planificacion ni migracion de Fabric. La demostracion local
de descubrimiento de Fabric es
`cargo run -p fabric-node -- manifest` desde el checkout de Fabric: escribe un
manifiesto pero no aprovisiona nada. Consulte
[Posicionamiento alpha y guia de prototipo](../releases/alpha-positioning.md)
para el limite exacto, la tabla de decisiones y los requisitos de la Fase 1.
