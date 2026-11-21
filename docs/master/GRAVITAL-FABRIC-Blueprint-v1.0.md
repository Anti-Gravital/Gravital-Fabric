# Gravital Fabric Blueprint v1.0

Status: canonical. Supersedes no earlier document (first version).

## 1. Vision

Gravital Fabric makes infrastructure portable. A customer owns machines,
storage and networks; Fabric turns them into a distributed infrastructure that
can be administered, replicated, migrated and taken elsewhere without asking
a provider for permission.

The project is not a VPS provider and not a hosting company. It is the open
infrastructure layer of the Gravital platform. Commercial services are built on
top in a separate repository.

## 2. Positioning

```
Commercial catalog        Gravital Cloud products (separate repository)
Infrastructure platform   Gravital Fabric (this repository)
Technology foundation     Anti-Gravital (separate repository)
Physical resources        bare metal, workstations, racks, VPS, cloud VMs
```

The two open projects are independent. Fabric does not require Anti-Gravital.
Anti-Gravital components may implement Fabric contracts as optional adapters.
The boundary is open, versioned data, not Rust imports.

## 3. Scope

In scope:

- node discovery and cryptographic node identity;
- workstations, servers, racks, VPS and cloud VMs as provider adapters;
- compute as virtual machines, containers and sandboxed functions;
- image registry, persistent volumes, snapshots and checkpoints;
- replication between nodes, regions and providers;
- global control plane with local node autonomy;
- scheduling, quotas, metering and audit records;
- private networks, firewall rules, gateways and tenant segmentation;
- capability-scoped machine interface, shared with Anti-Gravital.

Out of scope unless an RFC amends this statement:

- building a general-purpose container orchestrator;
- replacing Docker, Kubernetes, Terraform or a cloud console;
- implementing storage engines, hypervisors or TLS libraries from scratch;
- user-interface rendering or hosting a commercial portal in this repository;
- promising universal live migration of arbitrary execution state.

## 4. Portable Workload

The unit of portability is a declarative `Portable Workload`: image,
configuration, volumes, networks, policies and, optionally, an execution
checkpoint. It is not a disk image.

A portable package contains:

- the operating system image and its version;
- virtual hardware configuration;
- volumes and their persistence contract;
- cryptographic identity and network configuration;
- captured execution state, when the checkpoint permits it;
- policies, dependencies and declarative manifests.

## 5. Portability levels

| Level | What moves | Contract |
| --- | --- | --- |
| L1 Image | OS, configuration and disks | Always offered |
| L2 Workload | Containers, dependencies, volumes and configuration | Offered with compatible standards |
| L3 Execution | RAM, virtual CPU and devices | Conditional; never guaranteed on arbitrary hardware |

If hardware or runtime compatibility fails, the workload boots from its image
and restores persistent data. The system records exactly which level was used.

## 6. Persistence

Persistence is a first-class service with four components:

- **Image Registry** — versioned images, templates and artifacts.
- **Volume Manager** — node-independent persistent volumes.
- **Snapshot and Checkpoint Engine** — consistent capture, versioning and
  restore.
- **Replication Engine** — replica of images, volumes and snapshots across
  nodes, zones and providers.

Compute nodes are replaceable; data survives the loss of a node.

## 7. Product catalog (separate repository)

The commercial layer consumes Fabric contracts and offers Compute, Storage,
Functions, Automation, Network and Managed Clusters. Product names do not use
third-party trademarks; such names are confined to adapter labels. Clusters
and virtual machines are different products because their isolation and
billing contracts differ.

## 8. Principles

1. The customer owns the infrastructure and can export everything.
2. No workload is prisoner of a server, data center or provider.
3. Portability claims are measurable and level-specific.
4. Integrate with dominant tools; do not replace them.
5. Native operation works without any external service; adapters are opt-in.
6. Rust only, with least privilege and secure defaults.
7. Documentation is the contract; code follows it.
