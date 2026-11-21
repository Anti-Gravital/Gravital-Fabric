# RFC-0002 - Portable Workload and portability levels

- Status: accepted
- Decision date: 2026-11-21
- Author: Angel Nereira (BDFL), Gravital Labs
- Target phase: Phase 1 and Phase 5
- Affected modules/crates: `fabric-workload`, `fabric-migration`, `fabric-compute`
- Predecessor RFC: RFC-0001
- Comment period: waived by BDFL decision.

## 1. Motivation

Moving a disk is not the same as moving a running machine. A portability
contract that promises everything will either fail in production or hide its
failures. Users need a unit they can export, inspect and restore, and a clear
statement of which parts of an instance survive a move.

## 2. Problem

1. An image alone does not capture volumes, identity, network or policy.
2. A memory snapshot depends on CPU features, hypervisor, virtual devices and
   kernel version; it cannot be restored on arbitrary hardware.
3. Without a declarative unit, every provider adapter invents its own format.

## 3. Alternatives considered

**A. Disk image as the unit.** Rejected: loses policy, identity and volumes.

**B. Provider-native snapshot as the unit.** Rejected: locks the workload to a
provider and its format.

**C. Declarative Portable Workload with explicit levels (chosen).** Describes
everything needed to rebuild the instance and records which state was
captured.

## 4. Proposed design

### 4.1 The unit

A `Portable Workload` is a versioned manifest with:

- image reference and digest;
- virtual hardware request (CPU, memory, devices);
- volume references and their persistence contracts;
- network attachment and cryptographic identity;
- policies, dependencies and capability grants;
- optional checkpoint reference with a compatibility contract.

### 4.2 Levels

| Level | Content | Guarantee |
| --- | --- | --- |
| L1 | Image, configuration and disks | Rebuildable on any supported host |
| L2 | Containers, dependencies, volumes, configuration | Rebuildable with compatible standards |
| L3 | Memory, virtual CPU and devices | Restorable only where the compatibility contract matches |

L3 is optional. If compatibility fails, the platform boots from the image and
restores persistent data, records the downgrade and reports it to the caller.

### 4.3 Package format

A package is a signed directory or archive with a manifest, image layers,
volume metadata, checkpoint data when present, and a checksum manifest. It is
portable without an account on the platform.

## 5. Implementation plan

1. Define the manifest JSON Schema.
2. Implement L1 export and import (Phase 1).
3. Add L2 support through container standards (Phase 1).
4. Add L3 with a compatibility contract (Phase 5).

## 6. Risks

| Risk | Probability | Impact | Mitigation |
| --- | --- | --- | --- |
| L3 oversold as universal | Medium | High | Levels in every API result and in the CLI; downgrade is explicit. |
| Package too large to transfer | Medium | Medium | Deduplicated layers and incremental replication. |
| Checkpoint contains secrets | Low | High | Encrypt packages; never log checkpoint contents. |

## 7. Impact

Defines the central data model of the platform. No existing API changes.

## 8. Rollback

The manifest is versioned; an incompatible change creates a new version and
importers keep supporting the previous one.

## 9. References

- ADR-0002, ADR-0003.
- `GRAVITAL-FABRIC-Arquitectura-Tecnica.md` section 5.
