# Milestone 04 progress — Base turn engine

Status: Verifying  
Target release: v0.0.4 (no release due)  
Last updated: 2026-10-15 (America/Los_Angeles)

[Plan](plan.md) · [Reports](reports/) · [Photos](../../photos/milestone-04/index.md) · [Roadmap](../../ROADMAP.md)

## Implemented

- Deterministic setup, snake order, second-settlement starting resources and explicit roll/build/pending-robber phases.
- Supplied dice outcomes, finite-bank shortages, resource conservation, building costs, distance/connectivity, opponent blocking and all piece limits.
- Seat-specific legal choices and clone-and-commit rejected-action atomicity. Authoritative hands have no Debug/Serialize; public projection contains counts only.
- Native test harness drives board pieces, legal targets, active player, phase, supplied dice and errors. F4/context menu opens the closable panel; seed regeneration resets the test.
- Flat port ships, generic question marks and strong outlined player pieces. Light sea, wheel zoom and right-drag pan preserved.

## Verification

- 28 workspace tests passed, including 96 seeded setup/replay cases, 120 ordinary replay turns, shortages, costs, piece limits, blocking, invalid outcomes, atomicity, native bridge and existing camera/input/contract tests.
- Final scripts/check.ps1 -Release passed formatting, all-feature Clippy and optimized workspace build. scripts/test.ps1 passed 28 tests; record/link checks passed. See report 002.
- Native wgpu frames captured with permission; metadata and inspection are in the photo index. Synthetic scenarios are replayed through the rules engine; contrast fixtures are not legal games.
- M03 PR #10 is merged into stable (7a0f859), confirmed from fetched Git history. This branch merges that accepted baseline without losing later planning records.

## Remaining checks

- Six-target GitHub build/package validation, final package integrity and PR handoff.
- Full human/native input walkthrough, including panel drag/collapse at compact size and keyboard-only placement, remains unverified. Existing headless camera input and native bridge tests do not establish human usability.
- Multiple-people/screens privacy and non-Windows interactive runtime remain unverified. No online functionality or complete-game claim.

## Limits

Seven waits for M05; no trading, cards, awards, victory, bots, saves, rooms or cloud. Explicit test dice and synthetic seats are a developer harness, not a supported shared-device mode. Unsupported runtime options are rejected. No protocol/save shape changes. Compact/scaled panels cover part of the board; move/collapse/close them to access covered targets.

## Blockers and next step

No source implementation blocker. Finish evidence/CI review, hand off the unmerged beta PR for user testing and explicit approval. Do not advance beta/stable/main or begin M05. No release/tag is due.

## Reports

[001 — Requested visual work](reports/001-visual-follow-up-requirements.md) · [002 — Engine implementation and checks](reports/002-engine-implementation.md)

