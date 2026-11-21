# RFC-0006 - Kaizen continuous governance

- Status: accepted
- Date: 2026-11-21
- Owner: Angel Nereira

## Motivation

The repository is at specification stage. It already has a documentation-first
constitution, RFC and ADR directories, and issue/close discipline, but no
automatic mechanism enforces those rules. Without a ratchet and repeatable
checks, quality can silently degrade once implementation starts.

## Problem

There is no versioned per-branch descriptor, no quality baseline, no automated
policy check and no periodic metric collection. The rules in `CLAUDE.md` are
binding but are not executed by any tool.

## Alternatives

- Keep relying on manual review only: rejected, it does not satisfy the
  principle that an unautomated rule is not enforced.
- Adopt an external hosted governance service: rejected, it would add a
  dependency and conflict with the portability principle.
- Install Kaizen as the authoritative rule set and reduce `CLAUDE.md`:
  rejected; `CLAUDE.md` remains the technical constitution.

## Design

Install a local Kaizen layer subordinate to `CLAUDE.md`:

- `.kaizen/RULES.md` is the canonical rule text for agents, but `CLAUDE.md`
  governs any conflict. All adapters point to the rule file.
- `scripts/kaizen` provides `init`, `start`, `check`, `baseline`, `metrics` and
  `retro`. It is a Bash CLI, not a second implementation language for Fabric
  crates (ADR-0005 is unchanged).
- `.githooks/commit-msg` validates subject length, emoji and AI-tool
  attribution; `.githooks/pre-push` runs `kaizen check`.
- Two GitHub Actions workflows autofill the PR body from the versioned
  descriptor and run the checks, plus record metrics on merge to `main`.
- Policies mirror `CLAUDE.md` section 7: 256-character commits, no emojis and
  no AI-tool attribution. These are enforced, not optional here.
- Generated rules and templates are bilingual, English canonical followed by
  a Spanish mirror, as required by `docs/INDEX.md`.
- Adapters are installed for Claude Code, Codex (`AGENTS.md`) and OpenCode.
  Adapter files are integration configuration that names tools; they are not
  work attribution and do not violate the ban on AI-tool evidence in authored
  content.
- Project test, lint and format commands stay empty: there is no crate yet.
  They are filled in by the first implementation RFC that adds a crate.
- The policy patterns in `.githooks/commit-msg` and `scripts/kaizen` name the
  forbidden strings so the hook can detect them. They are enforcement rules,
  not attribution of work to a tool.

## Implementation plan

1. Add this RFC and its ADR in the same change as the installed files.
2. Create `.kaizen/`, `scripts/kaizen`, `.githooks/` and the `docs/kaizen/`
   templates and data directory.
3. Add the GitHub workflows and PR template.
4. Merge a `Kaizen` section into `CLAUDE.md` and add the agent adapters.
5. Enable hooks with `scripts/kaizen init`, generate the baseline with
   `scripts/kaizen baseline` and record the first metrics line.
6. Verify with section 6 of the Kaizen installation document.

## Risks

- A `pre-push` hook can block pushes while the repository has no code; the
  checks are cheap and empty commands are skipped, so this is acceptable.
- Fork PRs have a read-only token and PR autofill will fail there; this is
  documented in the workflow.
- Two ADR directories could be confused; `docs/kaizen/adr/` is restricted to
  governance decisions and states this in its template.

## Impact

New governance files only. No Fabric crate, contract, engine or runtime is
changed. ADR-0001 to ADR-0005 remain in force.

## Rollback

Remove the Kaizen files and the `Kaizen` section from `CLAUDE.md`, unset
`core.hooksPath`, and revert the two workflows and the PR template. No data
migration is needed.
