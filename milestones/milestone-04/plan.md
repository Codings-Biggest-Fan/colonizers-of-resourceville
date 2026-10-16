# Milestone 04 — Base turn engine

Target release: **v0.0.4**. Status: Verifying. Refined against the accepted M03 topology and existing configuration contracts.

[Roadmap](../../ROADMAP.md) · [Progress](progress.md) · [Reports](reports/) · [Screenshots](../../photos/milestone-04/index.md)

## Goal and dependencies

Implement deterministic base-game setup and ordinary building turns.

Requires milestone 03 to pass its release gate. Reuse the existing rules/protocol contracts and relevant previously released systems; do not bypass unresolved earlier gates.

## Scope and implementation tasks

- [x] Implement explicit setup and turn phases with snake placement order and starting resource allocation.
- [x] Implement dice production and finite-bank shortage behavior.
- [x] Implement road, settlement, and city costs, connectivity, distance checks, and piece limits.
- [x] Expose legal actions and validate complete transitions before changing state.
- [x] Drive the native board from engine state with active-player and phase cues.

## Board visual requirements — requested 2026-10-15

Carry these M03 follow-up requirements into M04 implementation; this planning update does not start M04 or change the approved M03 renderer.

- [x] Use simple flat 2D ship graphics for every port, replacing the current shaded perspective boats. Keep each resource-specific port visually identifiable through its cargo/resource graphic.
- [x] Use a clear question-mark graphic for every generic 3:1 port (three of any resource for one). Keep the existing trade meaning; do not introduce a question-mark rule variant or permanent text labels on the board.
- [x] Give houses/settlements and roads much stronger contrast against every underlying terrain. Use distinct light/dark separation and readable silhouettes, not only a change of player color. Preserve readable overlap between houses, roads, tile boundaries and target highlights.
- [x] Preserve the simple light sea, no permanent top/bottom bars, no Board label or zoom slider, wheel zoom and right-drag pan unless the user requests another change.

Validate actual rendered graphics at default and compact sizes, zoomed out and at representative scaling. Inspect settlements and roads over every terrain, including the blue-gray mountains and bright fields; confirm resource-specific ships and generic question marks remain recognizable without hover. Keep trade details available through inspection. Capture before/after evidence with permission; a passing build alone does not establish contrast or visual recognition.

## Interfaces, state, persistence, and UI

Rules accepts explicit actions and controlled random outcomes, returns new state/events or actionable errors, and has no UI, network, database, or async dependency.

## Tests and acceptance checks

- [x] Exercise full setup order and placement restrictions.
- [x] Verify production, shortages, spending, resource conservation, and piece counts.
- [x] Prove rejected actions do not mutate game state and scripted turns replay deterministically.

## Screenshot evidence

Capture setup placement, legal-target highlighting, production state, and a rejected-action explanation.

Record actual captures and metadata in the linked photo index. Explain nonvisual work and absent captures truthfully.

## Compatibility and migration

Use the milestone-02 contract; add documented version changes if the state shape must change.

## Release gate and handoff

Deterministic setup and ordinary-turn tests pass; rejected actions are atomic.

Update progress, append actual implementation reports, index inspected photos, update ROADMAP.md/CHANGELOG.md, and complete evidence review before marking Complete. Then summarize the release and stop for the next request. User-requested Polish mode pauses advancement.
## Branch, user testing, and GitHub release

Use one milestone-04/base-turn-engine branch for implementation and polish. nightly follows pushed development; the implementation PR targets beta and remains unmerged for explicit user approval. beta retains its last approved baseline while new work is unapproved. Polish continues on that same branch; a stable PR requires separate explicit polish approval, after which main mirrors stable. No release/tag is due before M06. Do not start M05 without the next request. See [delivery workflow](../../docs/github-workflow.md).

## Refined implementation boundary — 2026-10-15

The user requested M04 implementation. Reuse `Board` stable IDs and `GameConfig` without changing their existing wire formats. Add engine version 1 as an in-memory authoritative state, explicit controlled dice, actor validation, public projection and seat-scoped hands. No save or network state format is introduced. Accept only the standard Heartland subset and casual two-seat setup; reject unsupported runtime options. A seven enters a pending robber phase and cannot be skipped; M05 owns its resolution, trading, cards, awards and victory. Native controls are explicitly a turn-engine test, not a shared-device game mode. Keep private hands off the board and out of diagnostic output.

Branch: `milestone-04/base-turn-engine`, incorporating approved M03 stable and subsequent M03/M04 planning records. Source and automated checks proceed independently of permission-gated native window checks.
