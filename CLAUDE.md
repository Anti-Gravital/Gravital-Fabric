# CLAUDE.md - Technical constitution of Gravital Fabric

## 0. Source of truth

Documentation comes first. No functional code is written before the
architecture, the contracts and the phase that authorize it are documented in
`docs/master/`, `docs/rfc/` and `docs/adr/`. Code implements documentation;
documentation is never rewritten to justify improvised code.

## 1. Scope

Gravital Fabric provisions, connects, administers, replicates, migrates and
meters heterogeneous infrastructure. It does not become a VPS provider, a
hosting company, a container orchestrator or a replacement for Kubernetes,
Docker, Terraform or cloud provider consoles. It integrates with them through
adapters.

## 2. Boundary with Anti-Gravital

- No crate named `fabric-*` depends on a crate named `ag-*`.
- No crate named `ag-*` is required to install or operate Fabric.
- Contracts are versioned data: Protobuf, OpenAPI, JSON Schema and declarative
  manifests. Generated types are consumed on both sides.
- An Anti-Gravital component may implement a Fabric component as an optional
  adapter, never as a mandatory dependency.

## 3. Ownership and portability

Infrastructure belongs to the customer. Every resource, image, volume,
snapshot, manifest and configuration must be exportable in documented formats
without an account on the platform. Portability has three explicit levels
(L1 image, L2 workload, L3 execution); L3 is conditional and must never be
presented as universal.

## 4. Honest claims

No invented benchmarks, no exaggerated claims, no marketing language in
technical documents. Every benchmark records hardware, operating system,
Rust version, commit, configuration, methodology, number of runs and standard
deviation.

## 5. Engineering

- Rust only. No second implementation language without an RFC that amends this
  rule and a CI change that covers it.
- SOLID, DRY, KISS, Clean Architecture, composition over inheritance, explicit
  over implicit, zero-cost abstractions, secure defaults.
- Minimize dependencies. Every new dependency needs maturity, maintenance,
  security, performance and real necessity documented in an RFC.
- Avoid `unsafe`; justify any occurrence in an RFC and a code comment.
- No secrets in source. Validate input at the boundary. Least privilege.

## 6. Persistence and failure

- Compute is replaceable; data is not. Volumes, images, snapshots and
  replication are first-class services, not a VM feature.
- A node must keep running its workloads when the global control plane is
  temporarily unreachable.
- Replication, high availability, migration, scaling and disaster recovery
  are different capabilities and must be documented and measured separately.

## 7. Governance

- An RFC precedes any new crate, new engine, contract change, security change,
  performance target, roadmap change or CI change.
- An ADR records every accepted architectural decision.
- Technical debt and bugs are tracked as GitHub Issues with verifiable
  acceptance criteria, labels and evidence. An issue is closed only after the
  fix is verified; blocked issues stay open and labelled.
- English is the canonical language of code comments, identifiers and
  technical documentation. A Spanish mirror may follow the English section.
- No emojis anywhere in the repository.
- No commit message, branch name, pull request title, code comment or document
  may contain evidence of an AI tool. Work is attributed to people and to the
  organization.
- Commit messages and pull request titles are limited to 256 characters.

## 8. Verification

No change is complete until `cargo fmt`, `cargo clippy`, `cargo test`,
`cargo audit`, `cargo deny` and the relevant benchmarks pass, and until the
documentation affected by the change is updated in the same commit.

## 9. Kaizen adapter

This file remains the technical constitution. `.kaizen/RULES.md` is a
subordinate continuous-improvement adapter; where the two disagree, this file
governs. Before declaring a task complete, run `scripts/kaizen check` and
report the real result. Start a task with `scripts/kaizen start <branch>`. Never
bypass hooks or relax `.kaizen/baseline.json` without human approval (rules K-5
and K-10).

The external loop extends this constitution through `.kaizen/loop/LOOP.md`.
It is installed paused at autonomy level 1. Loop iterations are launched by
`scripts/kaizen-loop`; an agent handles only the task in the selected prompt
and never pushes, opens PRs or merges.
