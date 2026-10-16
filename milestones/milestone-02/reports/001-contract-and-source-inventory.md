# 001 — Configuration contract and source inventory

Date: 2026-10-15 (America/Los_Angeles). Branch: milestone-02/rules-contract, based on origin/stable 25692ac. User request: finish M02; later correction adds fog-map reveal knowledge.

## Changes

Added pure rules configuration validation and acceptance model; protocol/save versions and strict bounded decoding; public/player projections; source-backed card/scenario inventories, public guest UI audit, compatibility and delivery assignments. No native UI or game engine changes. No cloud resources accessed/provisioned. M01 completion records reconciled through a new report; prior reports preserved. Single-branch approval rules and no-release-before-M06 cadence updated where in scope.

## Research evidence

Public guest browser inspection authorized explicitly in this chat. Inspected Mode/Map/Settings and bot/public mode choices through normal UI. Recorded session defaults, locks and unobserved behavior, not parity. No match started or purchase/account sign-in performed. Guest setup abandoned by closing tab. No guest identity or room code retained. Fog-map nearby road/settlement reveal is attributed to the user; exact trigger semantics deferred.

Official 2025 source PDFs fetched from linked publisher index, extracted using pypdf, SHA-256 pinned in docs/rules-sources.json. Selected science p13 and freight p20 pages rendered with bundled Poppler and inspected. Initial PyMuPDF rendering attempt failed because the optional module was absent; switched to existing Poppler without installing tools. Web PDF fetches exceeding service size limits were retrieved directly from the same official URLs. PDFs and artwork stay in ignored local research, not shipped assets.

## User-directed roadmap additions

The user explicitly requested fog behavior in a milestone and a final M21 for website/wiki/docs. Added M12 reveal tasks/tests, M17 fog authoring, and planned M21 records/photo index/roadmap/wiki navigation. M02 remains active; no later implementation or website deployment started.

## Actual checks

- Focused rules/protocol tests passed: initial 8, then 10 with inventory and two-recipient projection checks. No runtime/network/private-hand UI testing inferred.
- Final formatting, all-target/all-feature Clippy, optimized workspace build and all 13 workspace tests passed (12 new tests plus the existing server test). All 22 record sets and 614 links passed; single-M21 check passed after correcting scalar-array Count handling.
- Initial Clippy rejected find/is_none in a test; corrected to contains and reran successfully. Packaging correctly refused an uncommitted tree; commit first, then package so BUILD.txt identifies real source. Initial local gate transcript is retained locally; committed package/CI evidence follows in the next report.
- Added M21 to checker/wiki ranges, fixed the observed single-milestone scalar-array failure, and normalized changed Markdown to UTF-8. No original reports rewritten.

## Limitations and next steps

No games, saves, bots, cloud, rooms or native settings. All runtime capabilities remain disabled. Later milestones own full rule effects, event-card face distribution, larger expansion layouts, hidden reveal geometry, combination execution and shoulder-surfing checks. Complete local and GitHub gates, publish TESTING PREVIEW docs, open/attach beta PR, then stop for explicit review. Do not merge/promote/tag/release or start M03 automatically.
