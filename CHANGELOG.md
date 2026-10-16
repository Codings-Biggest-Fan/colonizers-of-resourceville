# Changelog

## M04 testing preview — 2026-10-15

- Deterministic base setup/ordinary-turn engine, controlled dice, finite bank, atomic validation, legal actions and building restrictions/piece limits.
- Native synthetic-seat turn test (F4/context menu), real pieces/targets, current phase/player and local errors, with no private hands displayed.
- Flat cargo ships, generic question marks and contrasting road/house outlines; light sea and camera retained. Native frames archived; tests and release gates tracked in M04 progress.
- M03 merged into stable through PR #10. M04 remains unapproved and Verifying; seven resolution and complete rules arrive at M05, offline games at M06. No release/tag due.

## M03 approved — 2026-10-15

- M02 polish approved and merged into stable through PR #8; main mirrors 861b8c6. No release due.
- M03 adds stable board topology, seeded base layouts and interactive native board inspection/camera controls. Original cartoon terrain, graphic harbors, simple surrounding sea and compact HUD passed acceptance with six-platform build/package validation.
- No playable turns, bots, saves or rooms; M03 implementation and polish approved for beta/stable through PRs #9/#10. See [progress](milestones/milestone-03/progress.md).


## Unreleased — Milestone 02 rules and configuration contract

### Added — 2026-10-15

- M02 passed local and six-target build/package gates; user merged PR #7 into beta at 3f32f09. Final evidence is recorded in M02 report 002. Stable/main unchanged; no release due before M06.

- Versioned, strictly serialized configuration, family/scenario/seat modes, deterministic validation, derived custom markers, and non-playable capability reporting.
- Readiness invalidation, immutable acceptance, atomic rejected edits, wire/save envelopes and bounded decoding, stable topology IDs, recipient privacy contracts and tests.
- Public guest pregame audit with observed defaults, locked options, explicit uncertainties, dispositions and milestone assignments.
- Pinned 2025 official sources with hashes, complete core scenario and development/progress-card inventories, combination policy, future fixtures and migration/privacy boundaries.
- User observation: fog tiles reveal when roads/settlements are built nearby; M12 implements reveal with verified adjacency/ship behavior, M17 adds fog map authoring.
- Added final planned M21 for a separate website and completed wiki/docs; expanded record validation/wiki navigation through M21. No website implementation or deployment is authorized by this planning update.
- Reconciled M01 merged status and current single-branch/release-cadence guidance. Native UI unchanged; no release/tag or playable game claimed.

## Unreleased — Milestone 01 foundation

### Added — 2026-10-15

- Five-crate native Rust workspace with pinned toolchain/lockfile, original wgpu preview, and loopback Axum/Tokio liveness server.
- PowerShell development, lint, test, rendered-frame capture, and package helpers.
- GitHub builds/packages for Windows/Linux/macOS x86-64 and ARM64; local testing remains Windows-first.
- Standing Catppuccin, settings, low-setup friend-play, privacy, copy, and Impeccable design rules; README main image below the title and five-image screenshot grid.
- Building/game-guide/workflow updates and inspected native-window evidence. No gameplay or milestone release is claimed.
- Corrected Unix ZIP executable permissions after inspecting a real GitHub-built artifact; all six actual GitHub artifacts passed independent hash/architecture/permission checks.

## Unreleased — Milestone 00 initial setup awaiting review

### Added — 2026-10-15

- Expanded the game description and README with a milestone roadmap, linked documentation screenshots, build/setup instructions, and channel guidance; refreshed wiki preview and repository About metadata.
- Current 20-milestone roadmap and accepted implementation plan.
- Per-milestone plans, progress records, report folders, and screenshot indexes.
- Architecture, rules, pregame configuration, compatibility, development, and deployment documents.
- Rust/native workflow, recordkeeping requirements, and user-directed Polish mode in AGENTS.md.
- Read-only prerequisite checking and milestone structure/link validation helpers.
- Published the planning structure to the public GitHub repository Codings-Biggest-Fan/colonizers-of-resourceville on main.

Added M00, nightly/beta/stable branch policy, per-milestone PR approval and GitHub Releases, wiki publication, building guide, and a Game Instructions source folder. Original setup reports moved to M00 with historical content preserved.

No v0.0.0 or v0.0.1 release has shipped. Rust implementation, CI, runtime screenshots, and release verification are pending.

