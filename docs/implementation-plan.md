# Accepted implementation plan

Established: 2026-10-15. This document preserves the agreed direction. [ROADMAP.md](../ROADMAP.md) is the current status; individual milestone plans are implementation handoffs.

## Goal and architecture

Build a Windows-first native Rust Colonist-inspired strategy game with offline solo/bots, private guest rooms, later public casual/ranked play, and original assets. Start with 2–4 seats and expand to 2–8. Hosts select human/bot/open seats before starting.

Use pure Rust rules, shared serializable protocol types, restricted-observation bots, egui/eframe + wgpu desktop, and a separate Axum/Tokio server. Supabase Auth/Postgres with SQLx provides online identity/storage in a new organization and project independent of Scout App. Keep guest refresh credentials in Windows credential storage and all server/database secrets out of desktop builds.

Offline saves use explicit versions and atomic replacement. Online commands are serialized per game, revision checked, deduplicated, persisted with their receipts before acknowledgment, and projected per recipient. Restore pending decisions, offers, deadlines, bot state, and committed revision after reconnect/restart.

## Gameplay and pregame contract

Cover base rules, Cities & Knights, Seafarers, their supported combinations, Traders & Barbarians, and Explorers & Pirates with pinned core scenarios and officially documented combinations. Do not imply arbitrary mixtures are compatible. Standalone games, promotional material, historical editions, browser builds, and non-Windows release packages are excluded.

One versioned GameConfig drives pregame controls, server validation, presets, and saves. Gameplay changes clear readiness; starting freezes configuration. Include room/seat settings, rules/scenarios/maps, generation and preview, victory/dice/robber/discard/trade policy, timing/turn order, expansion settings, audience/recovery policy, and saved/shared presets. See [pregame catalogue](pregame-options.md).

Selectable controls must change real behavior. Reject invalid combinations with reasons. Board seeds never expose future hidden draws. Casual two-seat play, official two-player rules, official 5–6 rules, and custom 7–8 rules are distinct.

## Milestone 00 — Initial setup

M00 separates repository/plans/docs/wiki/helpers and GitHub workflow from the original 20 implementation milestones. It targets source/docs-only v0.0.0 after user PR testing/approval. M01 remains native Rust foundation. [M00 plan](../milestones/milestone-00/plan.md).

Every milestone uses a dedicated branch and PR against stable. nightly is latest work, beta is the active unapproved PR head, and stable is the most recent approved/merged PR. Wait for explicit user approval before merging, publishing its GitHub Release, or advancing. [Workflow](github-workflow.md).

## Release sequence

1. **v0.0.1 — Foundation:** Establish a reproducible native Rust workspace and the permanent planning, evidence, and release structure. [Implementation plan](../milestones/milestone-01/plan.md).
2. **v0.0.2 — Rules and configuration contract:** Turn the complete pregame and expansion scope into a source-backed, versioned implementation contract. [Implementation plan](../milestones/milestone-02/plan.md).
3. **v0.0.3 — Board and preview:** Deliver a geometrically correct, interactive native board and a truthful local pregame preview. [Implementation plan](../milestones/milestone-03/plan.md).
4. **v0.0.4 — Base turn engine:** Implement deterministic base-game setup and ordinary building turns. [Implementation plan](../milestones/milestone-04/plan.md).
5. **v0.0.5 — Complete base rules:** Complete the base-game mechanics required to finish a valid match. [Implementation plan](../milestones/milestone-05/plan.md).
6. **v0.1.0 — Offline playable:** Release complete offline solo games against bots for 2–4 seats. [Implementation plan](../milestones/milestone-06/plan.md).
7. **v0.1.1 — Supabase foundation:** Create isolated online identity and durable storage foundations. [Implementation plan](../milestones/milestone-07/plan.md).
8. **v0.2.0 — Private online lobby:** Let friends create and join a consistent native online lobby. [Implementation plan](../milestones/milestone-08/plan.md).
9. **v0.3.0 — Reliable online games:** Make base-game multiplayer authoritative, durable, and recoverable. [Implementation plan](../milestones/milestone-09/plan.md).
10. **v0.4.0 — Base pregame completeness:** Make all planned base-game configuration controls functional and verifiable. [Implementation plan](../milestones/milestone-10/plan.md).
11. **v0.5.0 — Cities & Knights:** Deliver complete Cities & Knights games including all core cards and bot choices. [Implementation plan](../milestones/milestone-11/plan.md).
12. **v0.6.0 — Seafarers:** Deliver the pinned Seafarers core scenarios with bots and correct nautical rules. [Implementation plan](../milestones/milestone-12/plan.md).
13. **v0.7.0 — Combined expansions:** Support documented Seafarers plus Cities & Knights combinations. [Implementation plan](../milestones/milestone-13/plan.md).
14. **v0.8.0 — Five to eight players:** Scale the implemented game families to supported larger player counts. [Implementation plan](../milestones/milestone-14/plan.md).
15. **v0.9.0 — Traders & Barbarians:** Implement the pinned core Traders & Barbarians variants and scenarios. [Implementation plan](../milestones/milestone-15/plan.md).
16. **v0.10.0 — Explorers & Pirates:** Implement Explorers & Pirates as its own rules family with every pinned core scenario. [Implementation plan](../milestones/milestone-16/plan.md).
17. **v0.11.0 — Maps and configuration completion:** Reconcile the full settings catalogue and finish map/preset authoring and supported combinations. [Implementation plan](../milestones/milestone-17/plan.md).
18. **v0.12.0 — Public casual platform:** Add public discovery and social features without weakening private-room boundaries. [Implementation plan](../milestones/milestone-18/plan.md).
19. **v1.0.0-rc.1 — Ranked and operations:** Prepare competitive play and demonstrate production operation and recovery. [Implementation plan](../milestones/milestone-19/plan.md).
20. **v1.0.0 — Release:** Ship a verified Windows release with complete scope and evidence accounting. [Implementation plan](../milestones/milestone-20/plan.md).

## Permanent records and helper scripts

Maintain root README/ROADMAP/CHANGELOG/AGENTS, docs/, five reserved crate directories, all 20 milestone plans/progress/reports folders, matching photos/milestone-NN/index.md files, scripts/, supabase/migrations/, and assets/ATTRIBUTION.md.

Reports are append-only numbered implementation evidence; never fabricate completion. Photo indexes record actual native captures, capture date, build identity, dimensions, scaling, scenario, and inspection. Nonvisual work explains why no screenshot applies. Polish fixes need before/after evidence when possible.

PowerShell helpers cover bootstrap, development, checks, tests, capture, packaging, and milestone validation. Resolve paths independently of current directory, propagate errors, and never silently install/provision/delete/capture unrelated windows.

## Verification and execution

Each release gate combines meaningful subsystem tests, formatting/Clippy/builds where available, actual native visual inspection when affected, separate-client checks for networking, privacy tests, and saved evidence. Per-card/scenario coverage is mandatory for expansions.

Implement one milestone at a time. After its gate passes, summarize evidence/limitations and stop for the next request. When explicitly asked for Polish mode, pause advancement and fix reported flaws until asked to resume. See [agent instructions](../AGENTS.md#polish-mode).

No release dates are promised. Confirm actual organization/hosting costs before provisioning. Current scaffolding work does not imply a shipped v0.0.1.

## Public documentation

Publish PR-reviewed source to the GitHub Wiki with clearly labeled unapproved previews, then republish approved docs from stable after approval. Include [Building](building.md) and the [Game Instructions folder](game-instructions/README.md), linked from README. Keep source records and photo evidence versioned.
