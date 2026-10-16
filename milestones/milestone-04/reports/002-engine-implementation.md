# 002 — M04 engine implementation

Date: 2026-10-15 (America/Los_Angeles). Status: Verifying; no promotion/release authorized.

## Changes and decisions

The user requested M04. Fetched history confirms M03 merged through PR #10, stable 7a0f859. Created milestone-04/base-turn-engine from the planning head and merged stable, preserving all history. Refined the plan against actual Rust topology/configuration. No delegation, installation, services or changes to beta/stable/main.

Rules now own in-memory engine v1 with stable IDs, validated config/seat order, snake setup and starting resources, injected dice and explicit phases. Construction checks resource costs, connectivity/distance, blocked roads and reusable piece caps. Clone-and-commit keeps all errors atomic. Shortages resolve per resource: multiple recipients get none when short; a sole recipient receives what remains. The cached pinned base PDF hash matches docs/rules-sources.json (0d81a40c1d4eb5eb438a62f80d71cdaab5f00404722c3cfa0b106d79b635ebc6); implementation was checked against its extracted rules, not guessed from memory.

The engine deliberately stops on seven for M05. No trading/cards/awards/victory, bots/saves/rooms or gameplay transport. Config/protocol/save versions remain unchanged; engine v1 has no serialized authoritative payload. Public views exclude typed hands; authoritative state is neither Debug nor Serialize. This API boundary is not evidence of authentication or multi-screen privacy.

The native context menu/F4 opens a closable testing window controlling synthetic seats with supplied dice. Actual engine state drives pieces/phase/targets/errors. Private hands never render. Flat ships and geometric cargo/question-mark graphics replace the perspective atlas. Light/dark outlines separate pieces from terrain. Terrain art and light sea/camera are preserved.

## Checks and evidence

- cargo fmt --all and all-target/all-feature Clippy were run; initial lint errors were fixed before final gates.
- 28 workspace tests passed, including 96 seeded setup/replay cases, 120 ordinary turns replayed, shortages/conservation, costs, piece caps, opponent blocking, atomic rejection, unsupported options, existing topology/config/privacy fixtures and the native bridge/camera suite.
- Optimized workspace build passed before final text/mark refinement; final script gates are recorded in the handoff update.
- The user explicitly permitted game-window interaction/captures. The window helper returned foreground content when the game was occluded; none of that output was stored as evidence. Stopped helper captures and used only eframe's own rendered-frame output. Native captures are documented in the [photo register](../../../photos/milestone-04/index.md).
- Capture scenarios use the real engine API, except the clearly identified contrast renderer fixture. These frames do not prove a human input walkthrough. Compact panel overlaps are documented; the panel can move/collapse/close.
- No new Impeccable runtime installed; read existing PRODUCT/DESIGN and relevant native task guidance directly.

## Remaining and next steps

Finish final formatting/Clippy/tests/optimized build, package/hash checks and six-target CI; archive actual outcomes. Publish a labeled TESTING PREVIEW wiki and open/attach an unmerged implementation PR targeting beta. Keep Verifying until evidence/manual review is complete. Full human input/keyboard placement, multiple-screen privacy and nonlocal runtime remain unverified. Stop at M04; no releases before M06.

## Final local gate

After the count-section/piece-mark refinement: scripts/check.ps1 -Release passed formatting, all-target/all-feature Clippy with warnings denied and optimized workspace build. scripts/test.ps1 passed all 28 tests. scripts/milestone.ps1 -All passed 22 record sets, 149 Markdown files and 688 local links. This structure result does not assert visual or runtime truth. Twenty native frames are archived, including earlier iterations; final 018/019/020 show the reduced count section and readable ownership marks. Native scale was 1.5, with egui zoom 1.5 only where explicitly noted. Full native keyboard/input walkthrough remains pending rather than inferred from those frames.
