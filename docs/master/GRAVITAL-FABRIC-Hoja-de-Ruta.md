# Gravital Fabric - Hoja de Ruta

Status: canonical. Defines what may be built and when.

The order is deliberate: first make workloads portable and persistent, then
add a multi-node control plane, then turn those capabilities into products.
Commercial services do not begin before the infrastructure primitives exist.

## Phase 1 - Portable infrastructure runtime

Priority: maximum.

- Install an agent on any compatible Linux host.
- Discover CPU, memory, disks and network; create a cryptographic node
  identity.
- Provision containers and virtual machines.
- Register the node with Fabric.
- Export and import images.

**Acceptance:** deploy an instance on two different physical machines and
recover its disk and configuration.

## Phase 2 - Persistent storage and snapshots

- Node-independent volumes.
- Consistent snapshots and versioning.
- Replication between nodes.
- Automated restore.
- ZFS and Ceph adapters.

**Acceptance:** lose a compute node and recover the instance on another node
without losing confirmed data.

## Phase 3 - Control plane and multi-node

- Global inventory.
- Scheduler.
- Projects and tenants.
- Network fabric.
- Resource control and quotas.

**Acceptance:** deploy and administer resources on several nodes from one
control plane.

## Phase 4 - Cloud products

- Compute.
- Object storage.
- Containers.
- Serverless functions.
- Scheduled jobs.
- Managed clusters.

**Acceptance:** each product can be created, metered, limited, deleted and
billed through APIs.

## Phase 5 - Migration and replication

- Portable Workload package.
- Migration between hosts.
- Restore between providers.
- Compatible checkpoint and restore.
- Geographic replication.

**Acceptance:** demonstrate workload migration and persistence between two
heterogeneous environments, documenting which states are recoverable.

## Phase 6 - Commercial provider

- Portal.
- Public API.
- SDK and CLI.
- Plans, quotas and billing.
- Per-product service levels.
- Customer and support management.

This phase lives in the separate commercial repository. Only contracts and
adapters are published here.

## Gates

A phase may not start until the previous phase's acceptance criterion is
reproducible on real hardware, with hardware, operating system, tool versions,
commit, methodology and number of runs recorded. No start date is promised.
