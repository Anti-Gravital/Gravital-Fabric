# Audit: first local fabric-node prototype

Status: evidence note, 2026-11-22. Prototype only; Phase 1 gate remains open.

Scope: real execution of the `fabric-node` prototype on the development host,
with verbatim output. This note proves local discovery, a stable local
identifier, JSON manifest emission and a local registration state file. It does
not prove provisioning, image export/import, control-plane registration or the
two-machine Phase 1 acceptance criterion.

## Environment

| Item | Value |
| --- | --- |
| Run time (UTC) | 2026-11-22T12:56:28Z |
| Kernel and architecture | Linux 7.0.0-44-generic x86_64 |
| Rust compiler | rustc 1.99.0 (b940084d7 2026-09-28) |
| Cargo | cargo 1.99.0 (5f94df478 2026-08-27) |
| Toolchain | stable-x86_64-unknown-linux-gnu |
| Code commit | `909c30e97cf5e421bf5f3467f21a7426e3069861` |
| Cargo environment | `CARGO_TARGET_DIR=/home/angel-nereira/Github-local-angelnereira/shared-cargo-target`, `CARGO_INCREMENTAL=0`, `CARGO_PROFILE_TEST_DEBUG=0` |
| Network access used by the binary | none; only procfs, sysfs and std APIs were read |

The identifier below is derived from the local machine id by the fixed
algorithm in ADR-0008. It is not a secret and not a credential. No MAC address,
hostname or identifier is used here to authenticate anything.

## Command 1: manifest

Command:

```
cargo run --quiet -p fabric-node --offline -- manifest
```

Verbatim stdout (40 block-device entries and 3 network interfaces):

```json
{
  "manifest_version": "1.0.0",
  "node": {
    "node_id": "1372fd34-4732-1ba1-f6cc-2e65cbef1917",
    "identity_source": "machine-id",
    "os": "linux",
    "arch": "x86_64",
    "hostname": "angel-nereira-HP-ProBook-440-14-inch-G9-Notebook-PC"
  },
  "hardware": {
    "cpu_count": 12,
    "memory_total_bytes": 32729972736,
    "memory_source": "proc-meminfo",
    "block_devices": [
      {
        "name": "loop0",
        "size_bytes": 1674694656
      },
      {
        "name": "loop1",
        "size_bytes": 1674694656
      },
      {
        "name": "loop2",
        "size_bytes": 4096
      },
      {
        "name": "loop3",
        "size_bytes": 539365376
      },
      {
        "name": "loop4",
        "size_bytes": 544944128
      },
      {
        "name": "loop5",
        "size_bytes": 58200064
      },
      {
        "name": "loop6",
        "size_bytes": 66883584
      },
      {
        "name": "loop7",
        "size_bytes": 77574144
      },
      {
        "name": "nvme0n1",
        "size_bytes": 512110190592
      },
      {
        "name": "nvme0n1p1",
        "size_bytes": 1127219200
      },
      {
        "name": "nvme0n1p2",
        "size_bytes": 510980521984
      },
      {
        "name": "loop8",
        "size_bytes": 70049792
      },
      {
        "name": "loop9",
        "size_bytes": 70033408
      },
      {
        "name": "loop10",
        "size_bytes": 63565824
      },
      {
        "name": "loop11",
        "size_bytes": 63639552
      },
      {
        "name": "loop12",
        "size_bytes": 20930560
      },
      {
        "name": "loop13",
        "size_bytes": 14929920
      },
      {
        "name": "loop15",
        "size_bytes": 274960384
      },
      {
        "name": "loop16",
        "size_bytes": 17301504
      },
      {
        "name": "loop14",
        "size_bytes": 297193472
      },
      {
        "name": "loop17",
        "size_bytes": 20078592
      },
      {
        "name": "loop18",
        "size_bytes": 20074496
      },
      {
        "name": "loop20",
        "size_bytes": 172830720
      },
      {
        "name": "loop19",
        "size_bytes": 13320192
      },
      {
        "name": "loop21",
        "size_bytes": 99823616
      },
      {
        "name": "loop23",
        "size_bytes": 644382720
      },
      {
        "name": "loop22",
        "size_bytes": 96141312
      },
      {
        "name": "loop24",
        "size_bytes": 1581056
      },
      {
        "name": "loop25",
        "size_bytes": 1581056
      },
      {
        "name": "loop26",
        "size_bytes": 182878208
      },
      {
        "name": "loop27",
        "size_bytes": 421507072
      },
      {
        "name": "loop29",
        "size_bytes": 12537856
      },
      {
        "name": "loop28",
        "size_bytes": 19296256
      },
      {
        "name": "loop30",
        "size_bytes": 12161024
      },
      {
        "name": "loop31",
        "size_bytes": 19746816
      },
      {
        "name": "loop32",
        "size_bytes": 46882816
      },
      {
        "name": "loop33",
        "size_bytes": 847872
      },
      {
        "name": "loop34",
        "size_bytes": 34091008
      },
      {
        "name": "loop35",
        "size_bytes": 233730048
      },
      {
        "name": "loop36",
        "size_bytes": 233693184
      }
    ],
    "network_interfaces": [
      {
        "name": "enp3s0",
        "mac": "84:69:93:67:b0:f5",
        "state": "down"
      },
      {
        "name": "lo",
        "mac": "00:00:00:00:00:00",
        "state": "unknown"
      },
      {
        "name": "wlp0s20f3",
        "mac": "70:a8:d3:94:83:a1",
        "state": "up"
      }
    ]
  }
}
```

The block-device list is not sorted by name; it follows the order of
`/proc/partitions` on this host. Network interfaces are sorted by name.

## Command 2: register

Command:

```
cargo run --quiet -p fabric-node --offline -- register --state /tmp/fabric-node-state.json
```

Verbatim stdout:

```
registered 1372fd34-4732-1ba1-f6cc-2e65cbef1917 at /tmp/fabric-node-state.json (unix 1795352188)
```

The state file parsed as JSON with top-level keys `state_version`,
`registered_at_unix` and `manifest`. `state_version` is `1.0.0` and the nested
manifest is identical to the manifest output above. Only that caller-selected
path was written; no daemon, socket, container or system service was started.

## Verification results

| Check | Real result |
| --- | --- |
| `cargo fmt --all -- --check` | passed |
| `cargo check --workspace --locked --offline` | passed |
| `cargo clippy --workspace --all-targets --locked --offline -- -D warnings` | passed, no warnings |
| `cargo test --workspace --locked --offline` | passed: 16 unit tests and 6 integration tests, 0 failed |
| `python3 jsonschema.validate(manifest, contracts/fabric-node-manifest/v1.schema.json)` | passed |
| `cargo audit --file Cargo.lock --no-fetch` | exit 0, no vulnerabilities |
| `cargo deny check` | exit 0: advisories, bans, licenses and sources ok; two pre-existing unmatched-license warnings in `deny.toml` |
| `cargo metadata --no-deps --offline --manifest-path tools/kaizen-insights/Cargo.toml` | passed; workspace exclusion keeps the standalone tool usable |
| `scripts/kaizen check` | all green: descriptors, content policies, workflow scopes, format, lint, tests, debt 0 <= 0 |
| `git diff --check` | clean |
| Emoji and attribution scans on changed files | no emoji; no tool-attribution phrase |

The zero-debt line is not a relaxation: `.kaizen/baseline.json` remains at
`debt_max: 0`. The check script was corrected so an empty grep result counts as
zero instead of aborting under `pipefail` (RFC-0008 section 4.4).

## What this proves and what it does not

Proves: a Rust node prototype discovers CPU count, total memory, block devices
and network interfaces on a real Linux host; it derives a reproducible local
identifier without secrets; it emits JSON that validates against the published
versioned schema; it writes a local registration state file.

Does not prove: cryptographic node identity, control-plane registration,
container or VM provisioning, image export/import, volumes, snapshots,
replication, scheduling, metering or the roadmap Phase 1 acceptance criterion.

Phase 1 remains partial. The gate requires deploying an instance on two
different physical machines and recovering its disk and configuration, with
hardware, OS, tool versions, commit, methodology and number of runs recorded.
That has not been attempted here. This prototype is not a Fabric alpha and the
Anti-Gravital alpha is not a Fabric alpha.

## Exact next Phase 1 steps

1. Publish an RFC for cryptographic node identity and control-plane
   registration; supersede the deterministic prototype identifier with a real
   credential established at registration (architecture section 8, RFC-0004).
2. Publish versioned contracts for `fabric-compute` and the local part of
   `fabric-security`, then implement provisioning behind explicit adapters, with
   native operation still working without Docker or a hypervisor (RFC-0004
   sections 4.1 and 4.3).
3. Define the image/volume/snapshot contract and implement export and import of
   a workload with its disk and configuration (RFC-0002, RFC-0003).
4. Replace the local state file with a real registration client that queues
   changes while the control plane is unreachable (RFC-0004 section 4.2).
5. Reproduce the acceptance criterion on two different physical machines and
   record hardware, OS, tool versions, commit, methodology and number of runs.
6. Only after that evidence exists, reconsider the Phase 1 box and any Fabric
   alpha label. Do not close the gate from this note.
