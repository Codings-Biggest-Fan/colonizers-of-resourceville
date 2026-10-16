# Milestone 03 progress — Board and preview

Status: Complete
Target version: v0.0.3 (no release due)
Last updated: 2026-10-15 (America/Los_Angeles)

[Plan](plan.md) · [Reports](reports/) · [Photos](../../photos/milestone-03/index.md) · [Roadmap](../../ROADMAP.md)

## Implemented at 4ccbf26

- Stable shared topology, seeded base terrain/numbers and coastal ports; protocol IDs re-export rules types with unchanged wire representation.
- Native board inspection, seed error recovery, cursor zoom, pan, fit, keyboard selection, resize and scale-aware hit targets.
- Simplified original cartoon terrain, graphic cargo-boat ports without on-board labels, light blue sea, miniature 3D-looking sample settlement and no permanent UI bars/buttons/slider. Wheel zoom and secondary-button drag pan; context menu retains setup/fit.

## Verification

- 20 workspace tests and all-feature Clippy passed, including 2,051 seeded board cases, deterministic fixture and transformed hit targets. Includes egui input-event regression proving right-drag and wheel zoom in/out.
- Final optimized workspace build passed; all six clean-checkout GitHub builds/tests/packages passed. Local package CRC, sidecar SHA-256 and all 649 file hashes verified; All six artifacts, architectures, Unix permissions, 3,894 file hashes and both Linux container configurations verified; evidence archived.
- Eighteen real native frames captured at native pixels/point 1.5, including 480×420, 1400×720 and egui zoom 1.5. Final default/compact frames 017/018 inspected; previous iterations retained as before evidence. Scaling stress 016 inspected before final control removal.
- Native UI access was stopped with physical Escape. User subsequently authorized a new game-window-only check and capture session. No unrelated windows captured.

## Flaw checklist from user corrections

- [x] Replace flat color/symbol diagram with original terrain imagery.
- [x] Surround board with sea; later corrections removed the initial HUD/zoom controls.
- [x] Simplify artwork to a charming cartoon style.
- [x] Remove on-board port labels; use cargo boats and hover/selection details.
- [x] Reconcile inspected default/compact plus earlier wide/scaling frames, transformed-hit tests and egui camera input regression with user visual approval. Separate human native input session/final zoom-pan screenshot not completed; see acceptance limitations.

- [x] Remove bars, Board label and slider; wheel zoom and right-drag pan.
- [x] Lighten sea and give sample settlement shaded roof/side geometry.

## Remaining gates and limitations

User approved implementation and polish on 2026-10-15. [Beta PR #9](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/pull/9) and [stable PR #10](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/pull/10) created. Acceptance gates passed and TESTING PREVIEW wiki HTTP/navigation checks passed. Promotion follows through the approved PRs; see [acceptance report](reports/002-acceptance-and-approved-promotion.md). No playable turns, bots, saves, accounts or rooms. No M04 or release work. This is scoped feedback during M03, not indefinite Polish mode.
