# RFC-0007 - Kaizen loop and offline insights tool

- Status: accepted
- Date: 2026-11-21
- Owner: Angel Nereira

## Motivation

RFC-0006 installed the rules and gates, but improvement still depends on a
person starting each task. The loop extension adds an external orchestrator
that runs short, isolated iterations, keeps its memory in versioned files and
lets the repository decide whether a result is accepted.

## Problem

Without a loop, issues found in CI or telemetry do not reliably become
verified changes. A single long-lived agent session is also a poor unit of
work: context is exhausted, tasks drift and failures are hard to isolate.

## Alternatives

- One permanent agent session: rejected; context exhaustion and drift.
- Run the loop inside the coding agent: rejected; the agent must not control
  its own brakes.
- Implement the telemetry aggregator in Python, as in the source document:
  rejected; `CLAUDE.md` section 5 allows Rust only.
- Implement the aggregator in Bash: rejected; JSON parsing and redaction are
  error prone in a shell language.

## Design

### Loop

- `.kaizen/loop/LOOP.md` defines rules L-1 to L-13. It extends
  `.kaizen/RULES.md` and never overrides `CLAUDE.md`.
- `.kaizen/loop/loop.env` configures the orchestrator. `KL_AGENT=opencode`,
  `KL_AUTONOMY=1`, and `.kaizen/loop/PAUSE` is present. No unattended mode is
  activated by this change.
- `scripts/kaizen-loop` is the external orchestrator. The agent never pushes,
  opens PRs or merges; the orchestrator does so after validation.
- Every iteration works in a temporary clone and a branch
  `kaizen/<loop>-<timestamp>`. A journal entry is mandatory, even when there
  is no work.
- Brakes are outside agent control: PAUSE, daily run budget, turn limit,
  timeout, consecutive-failure circuit breaker and protected globs.
- Raising autonomy above level 1 needs an ADR signed by a person. Protected
  paths, security, authentication, payments, destructive migrations, data
  deletion and changes to the loop or its rules always need a person.
- Telemetry is opt-in, sanitised and read-only for the loop. External data is
  never treated as instructions (rule L-5).

### Insights tool

- `tools/kaizen-insights/` is a standalone Rust crate. `serde` and
  `serde_json` are its only dependencies.
- Dependency rationale: `serde` and `serde_json` are mature, actively
  maintained, widely deployed, MIT/Apache-2.0 licensed and necessary to parse
  JSON Lines safely. A hand-written parser was rejected as error prone; a
  second language was rejected by section 5.
- `scripts/kaizen-insights` is a thin shell wrapper that builds the crate on
  demand and executes it. The workflow calls the wrapper, not a Python file.
- Redaction is implemented with the standard library only. The tool writes
  aggregated clusters, p95 latency and usage counts; it never stores raw
  user text. The directory glob is simplified to a directory scan of `.jsonl`
  files.
- `deny.toml` pins the license policy for `cargo deny`.

### CI

- `.github/workflows/kaizen-loop.yml` runs one iteration every three hours
  and on demand. It is ineffective while PAUSE exists.
- `.github/workflows/kaizen-automerge.yml` reclassifies a PR using the
  baseline branch copy of `scripts/kaizen-loop`. At autonomy 1 no PR is
  auto-merged.
- Adapter blocks are added to `CLAUDE.md`, `AGENTS.md`, `opencode.json`,
  `.claude/commands/` and `.opencode/commands/`. The v2 plural command path is
  used. Shell permission restrictions in `opencode.json` are deferred to the
  human security setup.

## Implementation plan

1. Add this RFC and ADR-0007 in the same change as the loop files.
2. Create `.kaizen/loop/`, the prompts, templates, telemetry schemas and the
   journal directory.
3. Add `scripts/kaizen-loop`, the Rust insights crate and its wrapper.
4. Add the two workflows and the adapter blocks.
5. Keep `KL_AUTONOMY=1` and PAUSE active. Do not run `kaizen-loop init`, do not
   create labels, secrets or tokens.
6. Verify the safe local checks in section 10 of the source document and
   report which steps need a human.

## Risks

- A broad personal GitHub token is currently configured in the CLI. The loop
  must use a separate minimal `KAIZEN_TOKEN`; this is a human-setup item.
- An unconfigured model API key or spend limit makes unattended runs fail or
  cost money. PAUSE prevents this until a person acts.
- Generated issues are untrusted input. Rule L-5 and the warning header in
  generated issue bodies reduce prompt-injection risk.
- The Rust tool adds two dependencies and a Cargo lockfile where the product
  had no code. It is an offline governance tool, not product implementation.

## Impact

No Fabric crate, contract or engine is added. `README.md` is updated to state
that the product remains at specification stage and the only executable code
is the governance tool. RFC-0006 and ADR-0006 remain in force.

## Rollback

Delete the loop directories, scripts, crate, workflows and adapter blocks;
remove the RFC/ADR and their index rows; remove the `.gitignore` entry. The
baseline, metrics and existing KAIZEN files are unchanged.
