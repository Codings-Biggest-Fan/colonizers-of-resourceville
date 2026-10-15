# 001 — Planning structure and contributor rules

Date: 2026-10-15 (America/Los_Angeles)  
Milestone: 01 — Foundation  
Target release: v0.0.1  
Session scope: structure, roadmap, plans, evidence organization, and AGENTS.md first.

## Changes

- Established the native Rust product direction and the full 20-release roadmap.
- Created all 20 milestone plans, progress records, report folders, and photo indexes.
- Added architecture, rules-source policy, pregame catalogue groups, compatibility, development, deployment, and overall implementation documentation.
- Reserved the five crate directories without pretending a Cargo workspace exists.
- Added attribution/migration destinations and a helper-script guide.
- Added read-only prerequisite checking and a record/local-link validator.
- Updated AGENTS.md with one-milestone-at-a-time execution, evidence requirements, and explicit user-directed Polish mode.

## Verification record

The structure/link validator and prerequisite checker are scheduled for the closeout check immediately after file creation. Results will be recorded in the subsequent numbered validation report; no passing result is claimed in this entry.

Earlier targeted environment inspection did not find rustc/cargo on PATH, a default Cargo bin directory, or the queried standard Visual Studio build-tool location. No installation has been performed.

## Screenshots

[Photo index](../../../photos/milestone-01/index.md). No screenshot exists: this session changes documentation/scripts and there is no native application window. The milestone-01 native-window screenshot requirement remains pending.

## Decisions and limitations

The user's latest instruction prioritizes structure/roadmap/plans first and adds the contributor rules and Polish mode. This session does not implement Rust gameplay, application scaffolding, CI, server packaging, or runtime capture. No cloud resources were created and no release shipped.

Plans for later milestones are initial scoped handoffs, not final unverified schema or rule decisions. Reference audit and pinned rules remain M02 work.

## Exact next steps

1. Run the record/link validator from outside the repository, check both helper scripts parse, and verify pending milestones fail the Complete-status gate.
2. Record actual check results in 002-structure-validation.md.
3. Keep M01 In progress; continue toolchain verification, Cargo/native scaffolding, CI, remaining helpers, and native screenshot evidence when the user requests implementation to continue.
