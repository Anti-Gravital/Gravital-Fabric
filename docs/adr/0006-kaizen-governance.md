# ADR-0006 - Kaizen is a subordinate governance adapter

- Status: accepted
- Date: 2026-11-21
- Decider: BDFL (Angel Nereira)

## Context

The repository needs automated enforcement of its documentation-first rules,
a ratchet for quality metrics and per-branch descriptors. A governance system
was installed, but the repository already has a technical constitution in
`CLAUDE.md` and an RFC/ADR process defined in section 7.

## Decision

`CLAUDE.md` remains the technical constitution and governs all conflicts.
`.kaizen/RULES.md` is a subordinate adapter that defines the continuous-
improvement rules once for every agent. RFC-0006 authorises the CI and hook
changes. Content policies enforce the existing ban on emojis and AI-tool
attribution, and the 256-character limit for commit subjects and PR titles.
Generated rules and templates are bilingual with English canonical.

## Consequences

- Rules are defined once and referenced by every adapter.
- A change to governance rules requires a PR against `.kaizen/RULES.md`; an
  architectural change still requires `docs/adr/`.
- Two ADR directories exist with separate scopes: `docs/adr/` for architecture
  and `docs/kaizen/adr/` for quality exceptions.
- Adapter configuration names agent tools for integration purposes; this is
  not work attribution.

## Alternatives

- Make RULES.md the single constitution: rejected, it would duplicate and
  potentially override the existing technical constitution.
- Skip CI and hooks until the first crate exists: rejected, the checks are
  cheap and establish the discipline before implementation starts.

## Status

Accepted.
