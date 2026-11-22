# Gravital Fabric

[English](#english) | [Espanol](#espanol)

## English

Gravital Fabric is the open-source infrastructure fabric of the Gravital
platform. It turns a set of machines, storage devices and networks owned by a
customer into one manageable, replicable and migratable infrastructure.

It is a separate project from [Anti-Gravital](https://github.com/Anti-Gravital/Anti-Gravital).
Anti-Gravital is the open-source technology layer: framework, DSL, CLI, SDK,
security and runtimes. Gravital Fabric is the infrastructure layer that
provisions, connects, administers and moves workloads. A commercial catalog of
services is built on both layers in a separate repository.

### The three layers

```
Gravital Cloud       commercial catalog (separate repository)
        |
Gravital Fabric      infrastructure fabric (this repository)
        |
Anti-Gravital        open-source technology foundation
        |
Physical or cloud    bare metal, workstations, racks, VPS, cloud VMs
```

The layers are independent. Fabric does not require Anti-Gravital, and
Anti-Gravital components can be used on infrastructures that do not run
Fabric. Contracts are versioned open data; no crate in either project imports
a crate from the other.

### What Fabric does

- Discovers and registers nodes: CPU, memory, storage, network and
  virtualization capabilities.
- Provisions compute as virtual machines, containers and sandboxed functions.
- Keeps volumes, snapshots and images independent of the compute node.
- Replicates data and workload definitions between nodes, regions and
  providers.
- Schedules workloads, enforces quotas and meters consumption.
- Exposes a capability-scoped machine interface so a connected agent can
  operate the infrastructure with only the grants it was given.

### Portability levels

| Level | What moves | Compatibility |
| --- | --- | --- |
| L1 Image | Operating system, configuration and disks | Very high |
| L2 Workload | Containers, dependencies, volumes and configuration | Very high with compatible standards |
| L3 Execution | RAM, virtual CPU and execution state | Conditional on hardware and runtime |

L3 is never a promise for arbitrary hardware. When a checkpoint cannot be
restored, the instance boots from its image and restores persistent data.

### The guiding principle

> Infrastructure belongs to the customer. Gravital Fabric administers,
> connects, optimizes and moves their resources. No workload is technically
> prisoner of a server, a data center or a cloud provider.

A user can export an instance, its images, volumes, configuration, snapshots
and data in documented formats without an active account on the platform.

### Status

Specification stage with one local-only prototype. The first executable product
code is the `fabric-node` prototype crate: it discovers local CPU, memory, block
devices and network interfaces, derives a stable non-cryptographic local
identifier and writes a JSON manifest or a local state file. It opens no socket
and needs no external service. The offline governance tool at
`tools/kaizen-insights/` is also executable. The source of truth remains
`docs/master/`; decisions are in `docs/rfc/` and `docs/adr/`.

Roadmap Phase 1 is partial: no cryptographic identity, provisioning,
control-plane registration, image export/import or two-machine acceptance
evidence exists. This is not a Fabric alpha. See the
[prototype evidence](docs/audits/fabric-node-prototype.md) and the
[alpha positioning guide](docs/releases/alpha-positioning.md).

From a checkout, the prototype runs with:

```bash
cargo run -p fabric-node -- manifest
cargo run -p fabric-node -- register --state ./fabric-node-state.json
```

For alpha testers: the [alpha positioning and prototype guide](docs/releases/alpha-positioning.md)
states what can be exercised today with the Anti-Gravital alpha and what still
needs Fabric code. The [external prototype pointer](docs/examples/anti-gravital-prototype.md)
links to that walkthrough. The Anti-Gravital alpha is not a Fabric alpha.

Rust is the only implementation language. Licence: Apache-2.0.

---

## Espanol

Gravital Fabric es la capa de infraestructura de codigo abierto de la
plataforma Gravital. Convierte un conjunto de maquinas, almacenamiento y redes
propiedad del cliente en una infraestructura unica, administrable, replicable y
migrable.

Es un proyecto separado de [Anti-Gravital](https://github.com/Anti-Gravital/Anti-Gravital).
Anti-Gravital es la capa tecnologica abierta: framework, DSL, CLI, SDK,
seguridad y runtimes. Gravital Fabric es la capa que aprovisiona, conecta,
administra y traslada workloads. El catalogo comercial se construye sobre ambas
capas en un repositorio separado.

### Las tres capas

```
Gravital Cloud       catalogo comercial (repositorio separado)
        |
Gravital Fabric      infraestructura (este repositorio)
        |
Anti-Gravital        base tecnologica de codigo abierto
        |
Fisico o cloud       bare metal, estaciones, racks, VPS, VMs cloud
```

Las capas son independientes: Fabric no requiere Anti-Gravital y los
componentes de Anti-Gravital pueden usarse sin Fabric. Los contratos son datos
abiertos versionados; ningun crate de un proyecto importa un crate del otro.

### Que hace Fabric

- Descubre y registra nodos: CPU, memoria, almacenamiento, red y
  virtualizacion.
- Aprovisiona compute como maquinas virtuales, contenedores y funciones
  aisladas.
- Mantiene volumenes, snapshots e imagenes independientes del nodo de compute.
- Replica datos y definiciones entre nodos, regiones y proveedores.
- Planifica workloads, aplica cuotas y mide consumo.
- Expone una interfaz de maquina por capacidades para que un agente conectado
  opere la infraestructura solo con los permisos otorgados.

### Niveles de portabilidad

| Nivel | Que se traslada | Compatibilidad |
| --- | --- | --- |
| L1 Imagen | SO, configuracion y discos | Muy alta |
| L2 Workload | Contenedores, dependencias, volumenes y configuracion | Muy alta con estandares compatibles |
| L3 Ejecucion | RAM, CPU virtual y estado de ejecucion | Condicionada al hardware y runtime |

L3 nunca es una promesa para cualquier hardware. Si un checkpoint no puede
restaurarse, la instancia arranca desde su imagen y recupera los datos
persistentes.

### Principio rector

> La infraestructura pertenece al cliente; Gravital Fabric administra, conecta,
> optimiza y traslada sus recursos. Ningun workload queda prisionero de un
> servidor, un centro de datos o un proveedor cloud.

El usuario puede exportar su instancia, imagenes, volumenes, configuracion,
snapshots y datos en formatos documentados, sin necesidad de cuenta activa.

### Estado

Fase de especificacion con un prototipo local. El primer codigo ejecutable del
producto es el crate `fabric-node`: descubre CPU, memoria, dispositivos de
bloque e interfaces de red locales, deriva un identificador local determinista
no criptografico y escribe un manifiesto JSON o un archivo de estado local. No
abre sockets ni necesita servicios externos. La herramienta de gobernanza
sin conexion en `tools/kaizen-insights/` tambien es ejecutable. La fuente de
verdad sigue siendo `docs/master/`; las decisiones estan en `docs/rfc/` y
`docs/adr/`.

La Fase 1 de la hoja de ruta es parcial: no hay identidad criptografica,
aprovisionamiento, registro en un plano de control, exportacion de imagenes ni
evidencia de aceptacion en dos maquinas. Esto no es un alpha de Fabric. Consulte
la [evidencia del prototipo](docs/audits/fabric-node-prototype.md) y la
[guia de posicionamiento alpha](docs/releases/alpha-positioning.md).

Desde un checkout, el prototipo se ejecuta con:

```bash
cargo run -p fabric-node -- manifest
cargo run -p fabric-node -- register --state ./fabric-node-state.json
```

Para quienes prueban el alpha: la [guia de posicionamiento alpha y prototipo](docs/releases/alpha-positioning.md)
indica que se puede ejecutar hoy con el alpha de Anti-Gravital y que requiere
mas codigo de Fabric. El [puntero al prototipo externo](docs/examples/anti-gravital-prototype.md)
enlaza ese recorrido. El alpha de Anti-Gravital no es un alpha de Fabric.

Rust es el unico lenguaje de implementacion. Licencia: Apache-2.0.
