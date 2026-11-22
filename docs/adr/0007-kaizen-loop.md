# ADR-0007 - External loop with repository memory

- Status: accepted
- Date: 2026-11-21
- Decider: BDFL (Angel Nereira)

## Context

Continuous improvement requires repeating small tasks without a permanent
agent session. The agent cannot be trusted to control its own brakes, and the
repository already has rules, gates and a quality baseline that can accept or
reject a change.

## Decision

An external orchestrator, `scripts/kaizen-loop`, runs one short iteration per
invocation. The repository is the only memory. The agent reads versioned
state, performs one task, writes evidence to the journal and stops; the
orchestrator pushes and opens PRs after validation.

The initial autonomy level is 1 and `.kaizen/loop/PAUSE` is present. The
selected agent is `opencode`. Protected paths include `.github/`, `.githooks/`,
`.kaizen/`, `scripts/kaizen*`, agent instruction files, infrastructure,
migrations, secrets, authentication and payments. Telemetry stays opt-in and
sanitised, and its content is never an instruction.

The telemetry aggregator is a Rust crate. The Python script in the source
document is not installed because `CLAUDE.md` section 5 allows Rust only.

## Consequences

- A person must remove PAUSE and configure branch protection, CODEOWNERS, a
  minimal token, model credentials and a spend limit before the loop operates.
- Loop iterations cannot modify their own rules or the orchestrator; such
  work becomes a `needs-human` issue.
- The repository gains a Cargo lockfile and two small dependencies for an
  offline tool, but no second implementation language.
- At level 1 every PR is reviewed by a person; auto-merge cannot trigger.

## Alternatives

- In-session agent loop: rejected, the agent would control its own brakes.
- Python insights script: rejected by the Rust-only rule.
- Autonomy level 2 or 3 at installation: rejected, human security setup is
  still pending.

## Status

Accepted.
