# 001 — Board implementation and visual corrections

Date: 2026-10-15 (America/Los_Angeles). Branch: milestone-03/board-preview, based on approved M02 stable merge 861b8c6. Status: In progress; not a completed milestone or approved implementation.

## Actual work

Implemented integer-lattice board topology and reproducible generator v1 in rules, with protocol ID re-exports preserving tuple serialization. Base board has 19 tiles, 54 vertices, 72 edges, constrained number inventory and nine coastal ports. Added seed validation, selection, camera transforms and native original terrain/boat art. No gameplay enabled.

User corrections replaced flat diagrams/painterly detail with simple cartoon artwork and unlabeled cargo-boat ports. Sea is now plain light blue. Removed top/bottom bars, then removed the temporary Board button and zoom slider. Wheel zooms and secondary-button drag pans; primary click selects. Setup/fit are available in a right-click context menu. Sample settlement uses original roof/front/side geometry and shading.

## Validation performed

- Workspace tests: 20 passed, including 2,051 seeded boards, deterministic golden fixture, graph/port checks, transformed hit targets, atomic seed errors and camera limits.
- New egui event regression sends actual secondary-button down/move/up plus positive/negative wheel events: pan delta and zoom directions passed. This is headless UI behavior, not a claimed human native interaction.
- Formatting and all-target/all-feature Clippy passed. Final optimized workspace build passed (13.91 seconds for the final desktop rebuild). Record/link check passed: 22 milestone sets, 145 Markdown files and 647 local links.
- Eframe capture helper produced only the game frame. Current 017/018 were inspected at 1500×1020 and 720×630 with native pixels/point 1.5. Earlier 016 egui zoom 1.5 was inspected at effective pixels/point 2.25; it is not proof of changing Windows scaling. Full metadata and superseded before images: [photo index](../../../photos/milestone-03/index.md).
- Computer Use reported physical Escape stops twice, including after a renewed authorization; no later Computer Use calls were made. The user gave fresh permission for this session's game-window capture. Captures used the application's own frame helper. Direct native input checks remain pending.

## Limitations and next steps

Finish optimized/package clean-checkout checks, native camera/context-menu checks and current wide/scaling evidence, six-target GitHub build/package/artifact validation, testing-preview wiki and beta review PR. Keep M03 unapproved, beta at its approved M02 baseline and stable/main at approved 861b8c6. No release/tag before M06; no M04. All generated artwork provenance is in assets/ATTRIBUTION.md; generated assets are not runtime evidence.
