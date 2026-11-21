# RFC-0003 - Persistent Storage Fabric and durability contract

- Status: accepted
- Decision date: 2026-11-21
- Author: Angel Nereira (BDFL), Gravital Labs
- Target phase: Phase 2
- Affected modules/crates: `fabric-storage`, `fabric-replication`
- Predecessor RFC: RFC-0002
- Comment period: waived by BDFL decision.

## 1. Motivation

Portability is only useful if data survives the node. Persistence must be a
service of the fabric, not a feature of a virtual machine. This RFC separates
the four storage responsibilities and defines what durability means.

## 2. Problem

1. A volume tied to a compute node disappears with the node.
2. A snapshot taken without coordination can capture an inconsistent state.
3. Replication is often confused with backup or high availability.
4. No durability contract means users cannot know which writes survived a
   sudden power loss.

## 3. Alternatives considered

**A. VM-local disks with periodic copies.** Rejected: node loss loses data.

**B. One mandatory distributed filesystem.** Rejected as the default: it would
make a specific engine a requirement for all users.

**C. Four storage services with replaceable engines (chosen).** The platform
owns the contract; the engine is an adapter.

## 4. Proposed design

### 4.1 Components

- **Image Registry** — versioned images and templates, with digests.
- **Volume Manager** — node-independent volumes with attachment and release.
- **Snapshot and Checkpoint Engine** — consistent capture, versioning and
  restore.
- **Replication Engine** — replica of images, volumes and snapshots between
  nodes, zones and providers.

### 4.2 Engines as adapters

ZFS, Ceph and other engines are selected per deployment behind a trait. An
S3-compatible API is offered as an interface, not as a mandatory product. No
storage engine is implemented from scratch.

### 4.3 Durability contract

Three states are distinguished and exposed to the caller:

| State | Meaning |
| --- | --- |
| Confirmed | Written to stable storage and, if configured, replicated |
| In transit | Accepted by the node but not yet on stable storage |
| In memory | Not yet handed to the storage layer |

A successful write response means confirmed. The platform never promises to
preserve data that was only in memory or in transit when power failed.
Snapshots are coordinated with workloads and tagged with the level of
consistency achieved.

## 5. Implementation plan

1. Define volume and snapshot contracts.
2. Implement the native volume adapter and the ZFS adapter.
3. Add replication with explicit consistency levels.
4. Add the Ceph adapter behind a feature.

## 6. Risks

| Risk | Probability | Impact | Mitigation |
| --- | --- | --- | --- |
| Inconsistent snapshots | Medium | High | Coordination protocol and consistency tag. |
| Engine-specific behavior leaks into contracts | Medium | Medium | Contract tests run against every enabled adapter. |
| Replication storms | Medium | Medium | Rate limits and incremental transfer. |

## 7. Impact

Defines Phase 2 and the persistence plane of the architecture.

## 8. Rollback

An engine can be disabled by feature; volumes remain exportable in the
documented neutral format.

## 9. References

- ADR-0004.
- `GRAVITAL-FABRIC-Blueprint-v1.0.md` section 6.
