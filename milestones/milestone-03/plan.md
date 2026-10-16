# Milestone 03 — Board and preview

Target version: v0.0.3 (planning identifier; no release before M06). Status: Complete.

[Roadmap](../../ROADMAP.md) · [Progress](progress.md) · [Reports](reports/) · [Screenshots](../../photos/milestone-03/index.md)

## Goal and dependencies

M02 gates passed and the user approved polish and starting M03 on 2026-10-15. PR #8 promotes M02 to stable at 861b8c6; main mirrors it. Build only a deterministic base board and interactive local preview. No turn engine, playable setup, bots, saves or rooms.

## Refined scope against actual code

- Move the existing protocol topology ID types to rules and re-export them without changing their wire representation.
- Integer lattice topology: radius-two island, 19 tiles, 54 vertices, 72 edges, reciprocal adjacency, 30 coast edges.
- Generator version 1 and u64 board-only seed; 4 forest/pasture/fields, 3 hills/mountains, one desert. Standard 18 number tokens, no adjacent 6/8; nine coastal ports with distinct endpoints. Stable sorted IDs independent of seed and rendering. No game randomness generated or exposed.
- Native Catppuccin Mocha controls and light-blue sea surface, original terrain marks, numbered tokens, port connectors, optional clearly labelled sample pieces.
- Seed validation with preserved input and unchanged board on error; regenerate, inspect tiles/edges/vertices, hover/select, bounded cursor-anchored zoom, pan, fit, resize and keyboard access.
- Only implemented board controls; no selectable expansion/seat/rules controls that suggest gameplay works. Appearance settings remain later scope.

## Interfaces and compatibility

Rules owns pure topology/generation; desktop owns projection, hit testing, camera and transient selection. Protocol re-exports existing tuple IDs with unchanged serialization. Board is generated, not deserialized from untrusted data. Future generator layout changes require a new generator version. No existing save/map format migrated or playable capability enabled.

## Tests and acceptance

Topology reciprocity, shared intersections, coast/ports, graph connectivity, inventories and number constraints across seeded fixtures; deterministic golden output and extreme seeds. Desktop projection/inverse and hit targets across aspect ratios, scales and pan/zoom; meaningful egui input simulation for controls when feasible. Formatting, Clippy, workspace tests, release build/package, records/wiki, clean checkout and all six CI targets. Native evidence: default, compact, wide, zoom/pan and scaling stress. Permission required for UI checks under AGENTS.md; no invented screenshots or runtime claims.

## Delivery

One branch milestone-03/board-preview; nightly follows development. User approved implementation and polish on 2026-10-15. PR #9 targets beta; PR #10 targets stable. Promote after gates and mirror stable to main. No release/tag, M04, website or cloud work. Keep progress/reports current; only mark Complete after all gates and approval.

## User visual corrections

Use simplified cartoon terrain and cargo-boat ports without on-board text. Sea is plain and light. Remove top/bottom bars, permanent Board label and zoom slider. Mouse wheel zooms; right-drag pans; left-click inspects. Keep setup and Fit in a right-click context menu. Sample settlements must show shaded roof and side walls. These direct corrections supersede the earlier permanent HUD direction.

## Acceptance reconciliation

User approved implementation and polish; [acceptance report](reports/002-acceptance-and-approved-promotion.md) records all six build/package/artifact gates, local checks, inspected geometry and native-input limitations. Completion does not claim a human native interaction session or nonlocal runtime testing. M04 remains Planned.
