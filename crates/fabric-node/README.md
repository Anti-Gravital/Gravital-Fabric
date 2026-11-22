# fabric-node (local prototype)

Local-only hardware discovery prototype for Gravital Fabric. This is a partial
slice of roadmap Phase 1, not an installable node agent and not a Fabric alpha.
It opens no socket, starts no daemon and requires no external service.

Authorized by `docs/rfc/RFC-0008-local-node-prototype.md`; the identifier design
is recorded in `docs/adr/0008-deterministic-local-node-identity.md`; the data
contract is `contracts/fabric-node-manifest/v1.schema.json`.

## Commands

From the repository root:

```bash
cargo run -p fabric-node -- manifest
cargo run -p fabric-node -- register --state /tmp/fabric-node-state.json
cargo run -p fabric-node -- --version
cargo run -p fabric-node -- --help
```

`manifest` writes the versioned JSON manifest to stdout. `register` writes a
state document containing that manifest plus `registered_at_unix` to the path
given after `--state` (default: `./fabric-node-state.json`) and prints a one-line
confirmation. The state file is written through a temporary file and renamed.
An existing state file at that path is replaced.

Exit codes: `0` success, `1` local I/O failure, `2` invalid command line.

## What is discovered

| Fact | Source |
| --- | --- |
| Operating system and architecture | `std::env::consts` |
| Logical CPU count | `std::thread::available_parallelism` |
| Total memory | `MemTotal` in `/proc/meminfo`, converted from kB |
| Block devices and sizes | `/proc/partitions` (1024-byte blocks) |
| Network interfaces | Names from `/sys/class/net`, MAC from `address`, state from `operstate` |
| Hostname | `/proc/sys/kernel/hostname` |
| Node identifier | Fixed FNV-1a over `machine-id`, `dbus` machine id or hostname |

Missing procfs or sysfs entries produce empty arrays, `null` memory and the
source `unavailable`. No address is obtained from a socket and no interface is
configured or modified. Interfaces are sorted by name so the output is
reproducible on a stable host.

## Limitations

- The identifier is deterministic and non-cryptographic. It is not an
  authentication credential; Phase 1 still requires cryptographic identity.
- This command does not register anything with a control plane, provision a
  container or VM, export an image, schedule a workload or persist volumes.
- Linux procfs and sysfs paths are used for discovery. On other systems the
  corresponding fields degrade to empty or `null` values.
- JSON is produced by a small fixed writer rather than a serialization crate;
  the schema is the contract and integration tests run the real binary.
- There are no third-party crate dependencies.

## Tests

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo test --workspace --locked --offline
```

Unit tests use synthetic meminfo, partitions and sysfs fixtures; integration
tests execute the built binary and check the emitted JSON and the state file.
