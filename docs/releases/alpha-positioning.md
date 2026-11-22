# Gravital Fabric - Alpha positioning and prototype guide

Status: positioning document, 2026-11-22. Not a master document.
Scope: what Gravital Fabric is today, and what an alpha tester can and cannot
exercise. English is canonical (`CLAUDE.md` section 8).

## 1. Honest position

Gravital Fabric is at **specification stage with one local-only prototype**.
There is no product implementation, no installable node agent and no
control-plane service. The prototype crate `fabric-node` discovers local
hardware, derives a stable non-cryptographic identifier and writes a JSON
manifest or a local state file; it is authorized by RFC-0008 for that slice
only. One versioned JSON Schema is published under `contracts/`. The offline
governance tool in `tools/kaizen-insights/` is also executable.

The runnable alpha that exists today belongs to **Anti-Gravital**, not to
Fabric. It is a local API prototype: scaffold a project from a template,
compile a DSL schema into artifacts and run a service on loopback. It exercises
the technology layer only. It does not discover a node, provision a machine,
schedule a workload or migrate anything.

The three layers, from the Blueprint and the README:

```
Gravital Cloud       commercial catalog (separate repository)
        |
Gravital Fabric      infrastructure fabric (this repository, specification only)
        |
Anti-Gravital        open-source technology foundation (separate repository,
                     testing alpha available)
        |
Physical or cloud    bare metal, workstations, racks, VPS, cloud VMs
```

Layer independence is a contract, not a future promise: Fabric ADR-0001 and
Anti-Gravital ADR-0016 forbid a crate dependency in either direction.

## 2. What is specified

| Artifact | Content |
| --- | --- |
| Blueprint v1.0 (canonical) | Vision, positioning, scope, Portable Workload, portability levels |
| Arquitectura Tecnica (canonical) | Twelve engines, three planes, contracts, data model, migration flow |
| Hoja de Ruta (canonical) | Phases 1 to 6, acceptance criteria and the phase gate |
| RFC-0001 | Scope and boundary with Anti-Gravital and the commercial catalog |
| RFC-0002 | Portable Workload manifest and the L1/L2/L3 levels |
| RFC-0003 | Image registry, volumes, snapshots, replication and durability states |
| RFC-0004 | Control plane, node autonomy, provider adapters, machine interface |
| RFC-0005 | Functions runtime with WebAssembly capabilities |
| RFC-0006 / RFC-0007 | Kaizen governance and the external loop |
| ADR-0001 to ADR-0007 | Accepted architectural and governance decisions |
| RFC-0008 and ADR-0008 | First local prototype crate, contract layout and deterministic local identity |
| `contracts/fabric-node-manifest/v1.schema.json` | First published contract: local node manifest as JSON Schema data |
| `docs/diagrams/` | Layered architecture and migration flow diagrams |

The twelve specified engines are `fabric-node`, `fabric-compute`,
`fabric-storage`, `fabric-network`, `fabric-scheduler`, `fabric-migration`,
`fabric-replication`, `fabric-runtime`, `fabric-security`, `fabric-control`,
`fabric-provider` and `fabric-metering`. Each has a contract in the
architecture. Only a local discovery slice of `fabric-node` has prototype code;
the other engines and the rest of `fabric-node` have no implementation.

## 3. What is NOT implemented

| Absent capability | Consequence for a tester |
| --- | --- |
| Only one local prototype crate and a Cargo workspace | A host can be discovered and a local manifest or state file produced; nothing can be installed or deployed as a product |
| Only the node manifest JSON Schema is published under `contracts/` | No generated types and no control-plane, compute or security contract exists yet |
| No installed node agent and no cryptographic node identity | The prototype identifier is deterministic and non-cryptographic; a host cannot register with Fabric |
| No `fabric-compute` | No container or virtual machine can be provisioned on a node |
| No volume manager, snapshot engine or replication engine | No node-independent data, even though RFC-0003 defines the contract |
| No control plane, inventory, scheduler or quotas | No multi-node administration |
| No migration or checkpoint code | No L1, L2 or L3 workload move can be demonstrated |
| No functions runtime | RFC-0005 defines it; no WebAssembly or native function runner exists |
| No machine-interface tools | RFC-0004 section 4.5 inherits seven prerequisites from Anti-Gravital RFC-0024; `deploy.*` tools stay unimplemented until Phase 5 |

RFC-0001 to RFC-0005 are accepted specifications. RFC-0008 authorizes a local
prototype only. Acceptance is not an implementation and must not be read as
one; real host output for the prototype is in
`docs/audits/fabric-node-prototype.md`.

## 4. Contract boundary with Anti-Gravital

Two accepted decisions govern the boundary:

- **Fabric ADR-0001** (no compile-time dependency with Anti-Gravital): no
  `fabric-*` crate depends on an `ag-*` crate. Both consume the same versioned
  contracts as data. An Anti-Gravital implementation may satisfy a Fabric
  contract as an optional, deployment-time adapter, never as a mandatory link.
- **Anti-Gravital ADR-0016** (open interfaces between AG and Fabric): no `ag-*`
  crate depends on a `fabric-*` crate either. Contracts are Protobuf for
  messages, OpenAPI for HTTP and JSON Schema for declarative manifests. Each
  product consumes generated types from those schemas.

| Boundary rule | Meaning in practice |
| --- | --- |
| Contracts are data, not imports | A shared schema can be exchanged without either product importing the other's code |
| No crate dependency in either direction | Either product can be installed, upgraded and replaced independently |
| Adapters are opt-in behind features | An operator selects an Anti-Gravital implementation for a Fabric component; it is never required |
| Export path is mandatory | Manifests, images, volumes, configuration and snapshots are exportable in documented formats without an account on either platform |
| Contract changes are versioned | A schema change needs an RFC in the product that owns the contract, not an implicit code change |

Consequence for a tester: the Anti-Gravital alpha produces a local service and
generated SDK artifacts from a DSL schema. Fabric may later consume the same
manifest as data under a versioned contract. No Fabric code reads or validates
that manifest today.

## 5. The three portability levels

The unit of portability is the declarative Portable Workload (RFC-0002,
ADR-0002): image, virtual hardware, volumes, networks, identity, policies and
an optional checkpoint. A disk image is one field of it, not the unit.

| Level | What moves | Guarantee | Status in Fabric today |
| --- | --- | --- | --- |
| L1 Image | Operating system, configuration and disks | Rebuildable on any supported host | Specified only |
| L2 Workload | Containers, dependencies, volumes and configuration | Rebuildable with compatible standards | Specified only |
| L3 Execution | Memory, virtual CPU and devices | Restorable only where the compatibility contract matches | Specified only, Phase 5 |

L3 is never a promise for arbitrary hardware. If compatibility fails, the
platform boots from the image, restores persistent data, records the downgrade
and reports it to the caller (ADR-0003). Tests must assert the downgrade path,
not only the happy path. No level has been demonstrated by Fabric code.

## 6. Prototype scenarios a tester CAN exercise today

These use **Anti-Gravital alone**. Preconditions: a Rust toolchain that meets
the Anti-Gravital MSRV, Git, a source checkout of the Anti-Gravital repository
and an install of `ag` from source. After the build, the walkthrough runs
offline and binds only to loopback. See
`docs/examples/anti-gravital-prototype.md` for the pointer to the external
walkthrough.

| Scenario | Command | What it proves | What it does NOT prove |
| --- | --- | --- | --- |
| Scaffold a project | `ag new my-api --template rest` | A project skeleton is created from a template | No node discovery or registration |
| Validate a schema | `ag schema lint --schema schema.ag` | The DSL schema parses and reports diagnostics | No Portable Workload manifest is produced or checked |
| Generate artifacts | `ag generate --schema schema.ag --output generated` | DSL compiles to Rust, SQL, TypeScript and OpenAPI artifacts | No Fabric contract or generated Fabric type is involved |
| Run locally | `cd my-api && ag dev` | A service starts on loopback (default `127.0.0.1:8080`) | No container or VM is provisioned on a node |
| Check the response | `curl http://127.0.0.1:8080/` | The generated service answers a local request | No network fabric, tenant isolation or quota applies |

These scenarios are useful for validating the technology layer and for
understanding the shape of a workload definition. They are not an infrastructure
prototype and must not be reported as Fabric working.

## 7. Scenarios that need Fabric code

| Capability | Required engine / contract | Roadmap phase | Specification | Status |
| --- | --- | --- | --- | --- |
| Install an agent and discover CPU, memory, disks and network | `fabric-node`, node identity | Phase 1 | RFC-0004 section 4.1 | Prototype only: local discovery and manifest on one host, no installed agent |
| Register a node with Fabric | `fabric-node`, control-plane inventory | Phase 1 | RFC-0004 | Prototype only: local state file, no control plane |
| Provision containers and virtual machines | `fabric-compute`, `fabric-runtime` | Phase 1 | RFC-0004, RFC-0005 | Not implemented |
| Export and import images with disk and configuration | `fabric-storage`, `fabric-migration` | Phase 1 | RFC-0002, RFC-0003 | Not implemented |
| Node-independent volumes, snapshots, automated restore | `fabric-storage`, replication | Phase 2 | RFC-0003 | Not implemented |
| Global inventory, scheduler, projects, quotas, network fabric | `fabric-control`, `fabric-scheduler`, `fabric-network` | Phase 3 | RFC-0004 | Not implemented |
| Functions, object storage and other catalog primitives | `fabric-runtime`, adapters | Phase 4 | RFC-0005 | Not implemented |
| Portable Workload migration, checkpoint and geographic replication | `fabric-migration`, `fabric-replication` | Phase 5 | RFC-0002, RFC-0003 | Not implemented |
| Capability-scoped infrastructure tools | Machine interface, shared contract | After prerequisites | RFC-0004 section 4.5, Anti-Gravital RFC-0024 | Blocked by seven prerequisites; no Fabric MCP code |

## 8. Decision table

| I want to... | Today with Anti-Gravital alone? | What is missing |
| --- | --- | --- |
| Discover local CPU, memory, disks and network as JSON | Yes, with the `fabric-node` local prototype | Cryptographic identity and an installed agent |
| Create an API project from a template | Yes | Nothing |
| Compile a DSL schema into Rust, SQL, TypeScript and OpenAPI | Yes | Nothing |
| Run a local service and call it on loopback | Yes | Nothing |
| Register a physical host as a Fabric node | No (local prototype state file only) | Cryptographic identity and control-plane registration (Phase 1) |
| Deploy a container or VM to that host | No | `fabric-compute` (Phase 1) |
| Keep data after a compute node is lost | No | Volumes and replication (Phase 2) |
| Schedule across nodes with projects and quotas | No | Control plane and scheduler (Phase 3) |
| Move a workload to another host or provider | No | Portable Workload package and migration (Phase 5) |
| Drive infrastructure through a machine interface | No | Seven RFC-0024 prerequisites and RFC-0004 section 4.5 tools |

## 9. Preconditions to start Phase 1

The roadmap gate says a phase may not start until the previous phase's
acceptance criterion is reproducible on real hardware. Phase 1 has no previous
implementation phase, so entering it requires the following, all before any
fabric crate is written:

1. An implementation RFC that adds the first `fabric-*` crate(s) and defines the
   `contracts/` layout, as required by RFC-0001 section 5 and the repository
   constitution. No feature without a document.
2. Versioned contracts published under `contracts/` (Protobuf, OpenAPI, JSON
   Schema and declarative manifests) with generated types consumable by both
   products, and no crate import in either direction (ADR-0001, ADR-0016).
3. Versioned contracts for the Phase 1 engines: `fabric-node`, `fabric-compute`
   and the local part of `fabric-security` (RFC-0004 section 4.1).
4. A defined deliverable: an agent installable on a compatible Linux host,
   hardware discovery, a cryptographic node identity, node registration,
   container and VM provisioning, and image export and import (roadmap Phase 1).
5. An acceptance procedure reproducible on two different physical machines:
   deploy an instance on each and recover its disk and configuration, recording
   hardware, operating system, tool versions, commit, methodology and number of
   runs (roadmap Gates).
6. Rust-only toolchain and CI for the first crate, with format, lint, tests,
   audit and deny checks configured (ADR-0005 and the repository constitution).
7. Security defaults in place: deny-by-default capability manifest, least
   privilege, no secrets in plans, audit records or machine-interface results
   (architecture section 8, RFC-0004 section 4.5).

### Prototype status against these preconditions (2026-11-22)

RFC-0008 authorizes the local prototype only. Against the list above: item 1 is
met for the prototype; item 2 is partly met (the contract layout and one JSON
Schema exist, generated types do not); item 4 exists only as local discovery
and a state file; item 6 has format, lint, tests, audit and deny executed
locally for the prototype, but no CI job for the crate yet; items 3, 5 and 7 are
not met. The prototype does not open the Phase 1 gate.

No start date is promised. Adding a crate, changing a contract or moving a phase
requires its own RFC and ADR where applicable.

## 10. Condition for Fabric's own alpha

Fabric may be labelled alpha only when the Phase 1 acceptance criterion has
been reproduced and published: an installable node agent on two different
physical machines that registers a node, provisions a container or VM, and
exports and imports an image with its disk and configuration, with the evidence
required by the roadmap gate (hardware, operating system, tool versions, commit,
methodology and number of runs). Until that evidence exists, the correct label
for this repository is "specification stage with a local-only prototype". The
Anti-Gravital testing alpha is not a Fabric alpha and must not be presented as
one.

## 11. References

- `docs/master/GRAVITAL-FABRIC-Blueprint-v1.0.md`
- `docs/master/GRAVITAL-FABRIC-Arquitectura-Tecnica.md`
- `docs/master/GRAVITAL-FABRIC-Hoja-de-Ruta.md`
- `docs/rfc/RFC-0001-scope-and-boundaries.md` to
  `docs/rfc/RFC-0005-functions-runtime.md`
- `docs/rfc/RFC-0008-local-node-prototype.md`
- `docs/adr/0001-no-compile-time-dependency-with-anti-gravital.md`
- `docs/adr/0003-honest-portability-levels.md`
- `docs/adr/0008-deterministic-local-node-identity.md`
- `docs/audits/fabric-node-prototype.md` — real host evidence for the prototype
- Anti-Gravital `docs/adr/0016-open-interfaces-between-ag-and-fabric.md` (read-only)
- `docs/examples/anti-gravital-prototype.md`
