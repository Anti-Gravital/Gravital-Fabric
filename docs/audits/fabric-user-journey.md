# Audit: real-user journey through the fabric-node prototype

Status: evidence note, 2026-11-22. Prototype only; the roadmap Phase 1 gate
remains open. This is not a Fabric alpha.

Scope: a second tester installed nothing, cloned no further repository and used
only the checkout plus a Rust toolchain. The tester followed the commands in the
root `README.md` and `crates/fabric-node/README.md` as written, timed them,
captured verbatim output, validated the JSON against the published contract and
checked determinism and repeat registration. One reproducible defect was found
and reported as Fabric issue #3. The original reproduction below is kept as
historical evidence; the defect was subsequently fixed and verified in wave 11
on branch `community/fabric-fix-11` (see "Issue #3 fix verification" below).

## Honest position

Gravital Fabric is a specification with one local-only prototype. The
`fabric-node` crate discovers local hardware, derives a deterministic
non-cryptographic identifier and writes JSON to stdout or a caller-selected
local state file. It opens no socket, starts no daemon and contacts no control
plane. There is no installable node agent, no product release and no
Provisioning, image export, scheduler or multi-node capability. The runnable
alpha that exists belongs to Anti-Gravital, not to Fabric. See
`docs/releases/alpha-positioning.md` and `docs/audits/fabric-node-prototype.md`.

## Environment

| Item | Value |
| --- | --- |
| Run time (UTC) | 2026-11-22T13:18Z to 13:21Z |
| Kernel and architecture | Linux x86_64 (same development host as the first audit) |
| Rust compiler | rustc 1.99.0 (b940084d7 2026-09-28) |
| Cargo | cargo 1.99.0 (5f94df478 2026-08-27) |
| Code commit | `910c3a080cb3fbcbda3e3835619ac9b8396988af` |
| Branch / worktree | `community/fabric-install` (isolated QA worktree) |
| Build target | private, disk-backed target dir outside the repository; `CARGO_INCREMENTAL=0`, `CARGO_PROFILE_TEST_DEBUG=0` |
| Network access by the binary | none; only procfs, sysfs and std APIs |
| JSON validator | Python `jsonschema` 4.19.2, draft 2020-12 |

The first build was attempted on the `/tmp` tmpfs, which has a filesystem quota
on this host; it failed with `Disk quota exceeded (os error 122)` while writing
a Cargo fingerprint. This is a host constraint, not a product defect. A private
target dir on the disk-backed filesystem was used for all measurements below.

## How the product was obtained

A real user today can only build from source. There is no installer, no
prebuilt binary, no `cargo install` target and no container image published by
this repository. From the checkout the only documented entry point is:

```bash
cargo run -p fabric-node -- manifest
cargo run -p fabric-node -- register --state ./fabric-node-state.json
```

Both commands work without network access because the workspace has no
third-party dependencies.

## Command-by-command journey

### Cold build (private target)

```
cargo build -p fabric-node --offline
```

Wall time 1.74 s, peak RSS 155836 KB, first compile of the single crate.

### Command 1: manifest (documented form, with `--offline`)

```
cargo run -p fabric-node --offline -- manifest
```

Wall time 0.39 s, peak RSS 18492 KB (includes Cargo startup and the
`cargo run` file-lock wait). Direct binary execution, warm: 0.02 s and about
2.4 MB RSS. stdout was 3716 bytes, 194 lines. Verbatim stdout:

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
      { "name": "loop0", "size_bytes": 1674694656 },
      "..." : "40 block-device objects in total, order follows /proc/partitions"
    ],
    "network_interfaces": [
      { "name": "enp3s0", "mac": "84:69:93:67:b0:f5", "state": "down" },
      { "name": "lo", "mac": "00:00:00:00:00:00", "state": "unknown" },
      { "name": "wlp0s20f3", "mac": "70:a8:d3:94:83:a1", "state": "up" }
    ]
  }
}
```

The abbreviated block-device section above is for readability only. The full
194-line stdout was captured verbatim and is byte-identical to the manifest in
`docs/audits/fabric-node-prototype.md`: 40 block devices, 3 network interfaces,
12 logical CPUs and 32729972736 bytes of total memory. The node identifier is
the same deterministic value reported in that audit. No hostname, MAC address
or identifier authenticates anything.

### Command 2: register (documented form, with `--offline`)

```
cargo run -p fabric-node --offline -- register --state /tmp/fabric-node-state.json
```

Wall time 0.15 s, peak RSS 18736 KB including Cargo. Direct binary execution,
warm: 0.01 s and about 2.5 MB RSS. Verbatim stdout:

```
registered 1372fd34-4732-1ba1-f6cc-2e65cbef1917 at /tmp/fabric-node-state.json (unix 1795352188)
```

In this journey the caller-selected path was a temporary file
(`unix 1795353533`, equal to 2026-11-22T13:18:53Z); the line above is the exact
form from the first audit. The state file was 4184 bytes, 198 lines, with
top-level keys `state_version` (`1.0.0`), `registered_at_unix` and `manifest`.
The nested manifest is identical to the `manifest` stdout. Only that path was
written; no daemon, socket, container or service was started.

### Other documented commands

| Command | Exit | Result |
| --- | --- | --- |
| `cargo run -p fabric-node -- --version` | 0 | `fabric-node 0.1.0` |
| `cargo run -p fabric-node -- --help` | 0 | usage block, 10 lines |
| `cargo run -p fabric-node --` (no command) | 0 | same usage block |
| `cargo run -p fabric-node -- manifest extra` | 2 | `fabric-node: manifest takes no arguments` |
| `cargo run -p fabric-node -- register --state` | 2 | `fabric-node: --state needs a path` |
| `cargo run -p fabric-node -- register --state ""` | 2 | `fabric-node: --state needs a non-empty path` |
| `cargo run -p fabric-node -- register --bogus x` | 2 | `fabric-node: unknown register option: --bogus` |
| `cargo run -p fabric-node -- bogus-command` | 2 | `fabric-node: unknown command: bogus-command` |
| `cargo run -p fabric-node -- register --state /nonexistent-dir/state.json` | 1 | `fabric-node: cannot write /nonexistent-dir/.state.json.tmp: No such file or directory (os error 2)` |

All observed exit codes match the documented contract: 0 success, 1 local I/O
failure, 2 invalid command line. The exact commands in the README, with and
without `--offline`, behave identically. The table above records the pre-fix
run at commit `910c3a0`; after the wave-11 fix the nonexistent-directory case
still exits 1, but the temporary path in the message is unique to the writer
(pid, sequence and nanoseconds) and the prefix is `cannot create temporary
state file`.

## Output inspection

### Schema validity

Both the `manifest` stdout object and the nested `manifest` inside the state
file validate against `contracts/fabric-node-manifest/v1.schema.json` (JSON
Schema draft 2020-12) using `jsonschema.validate`. Result: `VALID` in both
cases.

The registration state document as a whole does not validate against that
schema:

```
INVALID against published manifest schema: Additional properties are not allowed
('manifest', 'registered_at_unix', 'state_version' were unexpected)
```

This is expected, not a runtime bug. The only published schema describes the
discovery manifest; there is no schema for the state document. See the gap
below.

### Determinism

Two consecutive `manifest` invocations produced byte-identical stdout
(`cmp` clean). The identifier, CPU count, memory, the 40 block devices and the
3 sorted network interfaces are stable on this host across runs. Block-device
order is not sorted by name (documented behaviour); it follows
`/proc/partitions` and may change after a reboot or a storage change.

### Is `register` idempotent?

No. Re-running `register` to the same path replaces the file and changes only
`registered_at_unix`; here the value changed from `1795353533` to
`1795353547` while the nested manifest stayed identical. This is the
documented behaviour ("an existing state file at that path is replaced"), so
it is not a defect, but a user expecting a stable "register once" artifact
will see a different file on every run.

## What works today

- Build and run the only crate from a checkout, offline, with a stable toolchain.
- Discover OS, architecture, logical CPU count, total memory, block devices
  and network interfaces on a real Linux host.
- Derive a stable, documented, non-cryptographic local identifier.
- Emit JSON that validates against the published versioned schema.
- Write a local registration state file atomically via a unique per-invocation
  temporary file and rename, to a caller-selected path (unique naming added in
  wave 11; see the fix verification below).
- --version, --help, usage, and documented exit codes.

## What a user would expect but cannot do yet

These are honest limitations, already recorded in
`docs/releases/alpha-positioning.md`. They are not implemented and must not be
presented as working:

- Install a node agent. There is no installer, binary release or service unit;
  only `cargo run` from a source checkout.
- Register with a control plane. `register` writes a local file; no inventory
  or network endpoint exists.
- Authenticate a node. The identifier is a deterministic hash, not a
  credential; Phase 1 still requires cryptographic identity.
- Provision a container or VM, export or import an image, create volumes or
  snapshots, schedule a workload, or migrate a workload between hosts.
- Validate the registration state document with a published contract. Only the
  discovery manifest has a JSON Schema under `contracts/`.
- Idempotently register. Every invocation rewrites the timestamp.
- Achieve the Phase 1 acceptance criterion: deploy an instance on two different
  physical machines and recover its disk and configuration with recorded
  hardware, OS, tool versions, commit, methodology and number of runs.

## Defect found with reproduction

### Issue #3 - concurrent register to the same state path fails spuriously

`crates/fabric-node/src/register.rs::write_state` uses one fixed temporary
filename, `.<file_name>.tmp`, in the destination directory. Concurrent
invocations targeting the same `--state` path share that temp file. One
process renames it away while another is about to rename it, so the second
rename fails with ENOENT and that caller exits 1, losing its write.

Reproduction with the built binary:

```sh
BIN=/path/to/target/debug/fabric-node
D=$(mktemp -d)
for i in $(seq 1 32); do
  "$BIN" register --state "$D/state.json" >"$D/out.$i" 2>"$D/err.$i" &
done
wait
grep -l . "$D"/err.*
```

Observed failures across trials (parallel writers to one path):

| Trial | Writers | Failures |
| --- | --- | --- |
| 1 | 24 | 2 |
| 2 | 32 | 1 |
| 3 | 32 | 1 |
| 4 | 32 | 1 |

Verbatim stderr from a failing writer:

```
fabric-node: cannot rename state file to /tmp/.../state.json: No such file or directory (os error 2)
```

The final state file remains valid JSON, but one or more callers observe a
spurious failure. Reported as
[Anti-Gravital/Gravital-Fabric issue #3](https://github.com/Anti-Gravital/Gravital-Fabric/issues/3)
with labels `bug` and `good first issue`. Not fixed by this audit, which changed
no source code. Suggested direction: a unique per-invocation temporary name and
a concurrency integration test.

### Issue #3 fix verification (wave 11, 2026-11-22)

Fix landed on branch `community/fabric-fix-11` (descriptor
`docs/kaizen/pr-drafts/community-fabric-fix-11.md`). `write_state` no longer
uses `.<file_name>.tmp`. It builds a name containing the process id, an atomic
in-process sequence, nanoseconds and a retry index (`<file>.<pid>.<seq>.
<nanos>.<attempt>.tmp`) in the destination directory, opens it with
`create_new` (atomic O_CREAT|O_EXCL) and retries with a fresh name on a name
collision (at most 64 attempts). The document is written to that handle and
moved onto the destination with the same atomic `fs::rename`. Every cleanup
path removes only the path this writer exclusively created, so a failing
writer can never delete another writer's temporary file. The JSON state
format is byte-for-byte unchanged: `state_version`, `registered_at_unix` and
the nested `manifest` are produced by the same `to_json` function.

A regression test in `crates/fabric-node/tests/cli.rs`,
`concurrent_registers_to_one_state_path_all_succeed`, spawns 32 real
`fabric-node` processes at once against one `--state` path, asserts every
child exits 0, checks the final file is a whole JSON object with balanced
braces and the required fields, and asserts no `.tmp` file is left behind.
No sleeps or timing assumptions are used. The same reproduction as the
original report was rerun against the rebuilt binary.

| Evidence | Real result |
| --- | --- |
| Regression test, 32 concurrent processes | all 32 exit 0; final state whole JSON; no temp leftovers |
| Regression test repeated 10 times | 10 passed, 0 failed |
| Real binary, 5 trials x 32 concurrent writers | 0 failing invocations in every trial |
| `cargo fmt --all -- --check` | passed |
| `cargo check --workspace --locked --offline` | passed |
| `cargo clippy --workspace --all-targets --locked --offline -- -D warnings` | passed, no warnings |
| `cargo test --workspace --locked --offline` | passed: 16 unit + 7 integration tests, 0 failed |
| `scripts/kaizen check` | all green: descriptors, content policies, workflow scopes, format, lint, tests, debt 0 <= 0 |
| `git diff --check` | clean |

Build target for these measurements: private, disk-backed target dir outside
the repository (shared with the wave-11 agents); `CARGO_INCREMENTAL=0`,
`CARGO_PROFILE_TEST_DEBUG=0`. Same host and toolchain as the environment table
above. The Phase 1 gate and the Fabric alpha position are unchanged by this
local I/O fix.

## Top improvements for Fabric Phase 1

Ordered by what a tester hits first, not by implementation order:

1. Unique temporary files in `register` (issue #3) plus a concurrent-process
   test, so repeated registrations cannot fail spuriously. Addressed in wave 11;
   see the fix verification above.
2. An installable agent with a documented install path, rather than only
   `cargo run` from a checkout. This is the roadmap Phase 1 deliverable.
3. Cryptographic node identity and real control-plane registration; supersede
   the deterministic prototype hash.
4. A published JSON Schema for the registration state document, so the actual
   output of `register` is a verifiable contract like the manifest is.
5. `fabric-compute` provisioning behind explicit adapters, with native
   operation still working without Docker or a hypervisor.
6. Image export and import with disk and configuration, then the two-machine
   acceptance evidence required by the roadmap gate.

None of these changes the document hierarchy: a new crate, a contract change
or a phase change needs its own RFC and ADR where applicable.

## Verification of this audit

| Check | Real result |
| --- | --- |
| `scripts/kaizen check` | all green: descriptors, content policies, workflow scopes, format, lint, tests, debt 0 <= 0 |
| `git diff --check` | clean |
| Emoji and AI-attribution scans on changed files | no emoji; no tool-attribution phrase |
| `cargo fmt --all -- --check` | passed |
| `cargo clippy --workspace --all-targets --locked --offline -- -D warnings` | passed, no warnings |
| `cargo test --workspace --locked --offline` | passed: 16 unit tests and 6 integration tests, 0 failed |

No source code was changed by this audit, so the production binary behaviour
described above is the shipped prototype behaviour at commit `910c3a0`.

## References

- `docs/audits/fabric-node-prototype.md` — first real-host evidence.
- `docs/releases/alpha-positioning.md` — honest scope and tester decision table.
- `crates/fabric-node/README.md` — documented commands and limitations.
- `contracts/fabric-node-manifest/v1.schema.json` — the only published contract.
- `docs/master/GRAVITAL-FABRIC-Hoja-de-Ruta.md` — Phase 1 and the phase gate.
- Fabric issue #3 — the reproducible concurrency defect found in this journey.
