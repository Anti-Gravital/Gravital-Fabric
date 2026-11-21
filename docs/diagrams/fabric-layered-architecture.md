# Fabric layered architecture

> Last reviewed: 2026-11-21. Master chapter: Architecture, section 2.

```mermaid
flowchart TB
    subgraph Commercial[Separate repository]
        Catalog[Cloud product catalog]
    end
    subgraph Contracts[Versioned open contracts]
        Schema[Protobuf / OpenAPI / JSON Schema]
    end
    subgraph Fabric[Gravital Fabric]
        Control[fabric-control: inventory, scheduler, reconciliation]
        Node[fabric-node: discovery and identity]
        Compute[fabric-compute: VM and container]
        Runtime[fabric-runtime: native and WebAssembly]
        Storage[fabric-storage: images, volumes, snapshots]
        Replication[fabric-replication]
        Network[fabric-network]
        Security[fabric-security]
        Metering[fabric-metering]
        Provider[fabric-provider adapters]
    end
    subgraph Physical[Customer resources]
        Bare[Bare metal and racks]
        Work[Workstations]
        VPS[VPS and dedicated servers]
        Cloud[Cloud VMs and external clusters]
    end
    Catalog --> Schema
    Control --> Schema
    Node --> Control
    Compute --> Control
    Storage --> Replication
    Provider --> Bare
    Provider --> VPS
    Provider --> Cloud
    Node --> Work
    Security -.-> Compute
    Security -.-> Network
    Metering -.-> Control
```

The control plane is not hosted inside any single provider. A node keeps
running assigned workloads when the link to `fabric-control` is lost.
