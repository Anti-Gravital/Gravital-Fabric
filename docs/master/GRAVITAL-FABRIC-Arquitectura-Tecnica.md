# Gravital Fabric - Arquitectura Tecnica

Status: canonical. Companion to `GRAVITAL-FABRIC-Blueprint-v1.0.md`.

## 1. Engine model

Fabric is a set of engines with contracts independent of their implementations.

| Engine | Responsibility |
| --- | --- |
| `fabric-node` | Hardware discovery, node identity and registration |
| `fabric-compute` | Virtual machines, containers and resource assignment |
| `fabric-storage` | Volumes, images, snapshots and blocks |
| `fabric-network` | Virtual networks, firewall, routing and tenant isolation |
| `fabric-scheduler` | Placement and distribution of workloads |
| `fabric-migration` | Export, transfer and restore of workloads |
| `fabric-replication` | Replication between nodes, regions and providers |
| `fabric-runtime` | Rust, WebAssembly and container runtimes |
| `fabric-security` | Identities, isolation and policy enforcement |
| `fabric-control` | Global state and reconciliation |
| `fabric-provider` | Adapters for external providers |
| `fabric-metering` | Consumption, quotas and billing records |

Every engine exposes a versioned contract. A provider adapter implements the
contract for a specific environment without changing the engine.

## 2. Planes

- **Control plane:** inventory, policy, scheduling and coordination. It is not
  required to live inside any provider.
- **Data plane:** the node agent and its local runtime. A node continues to
  execute its assigned workloads if the control plane is temporarily
  unreachable, reconciling when connectivity returns.
- **Persistence plane:** volumes, images, snapshots and replication. It is
  independent of compute nodes so a node can be replaced without data loss.

## 3. Contracts

- Protobuf for service-to-service messages and public APIs.
- OpenAPI for HTTP surfaces.
- JSON Schema for declarative manifests.
- Every contract is versioned and published as data under `contracts/`.
- Generated types are consumed by both Fabric and Anti-Gravital. There is no
  Rust crate dependency between the two projects (ADR-0001).

## 4. Data model

| Entity | Meaning |
| --- | --- |
| Node | A physical or virtual machine with discovered capabilities |
| Tenant | An isolated owner of workloads and resources |
| Workload | A declarative Portable Workload instance |
| Volume | A persistent, node-independent block device |
| Image | A versioned operating system or workload template |
| Snapshot | A consistent point-in-time capture of a volume |
| Checkpoint | A captured execution state, with a compatibility contract |
| Plan | A proposed set of changes with an identifier and an apply step |
| AuditRecord | Time, identity, command, plan identifier and result |

## 5. Migration flow

1. **Execute** on node A.
2. **Checkpoint** memory, virtual CPU and compatible devices; coordinate
   volumes for a consistent point.
3. **Package** image, disks, state, metadata and dependencies.
4. **Transfer** to node B, another data center or another provider.
5. **Adapt** to the destination CPU, memory, storage, virtualization and
   network interfaces.
6. **Restore** volumes, policies and the compatible execution state.
7. **Resume** if the checkpoint level permits it; otherwise boot from the
   image and restore persistent data.

The chosen level (L1, L2 or L3) is recorded in the workload history.

## 6. Storage engines

ZFS, Ceph and other engines are replaceable adapters behind traits. The
project does not implement a filesystem or a distributed object store from
scratch. An S3-compatible API is offered as an interface, not as a mandatory
product. Engine selection and its trade-offs are recorded per deployment.

## 7. Runtime

Fabric runs Rust code natively and WebAssembly components under a capability
model with explicit limits. Containers are a runtime adapter for workloads
that need operating-system dependencies. Sandboxed functions are a workload
type, not a separate product stack.

## 8. Security

- Node identity is cryptographic and established at registration.
- Tenant isolation is enforced by the network and security engines.
- A mutating operation always has a plan identifier and an apply step.
- A connected agent receives explicit capabilities; the default is read-only.
- External services are adapters behind features; native operation is always
  available.
- Secrets never appear in plans, audit records or machine-interface results.

## 9. Observability

Every engine emits structured events and metrics. A workload carries its
migration level, placement decision and audit history so a failure can be
explained after the fact.

## 10. Diagrams

See `docs/diagrams/` for the layered architecture and the migration flow.
