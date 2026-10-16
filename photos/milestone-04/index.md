# Milestone 04 native evidence

[Plan](../../milestones/milestone-04/plan.md) · [Progress](../../milestones/milestone-04/progress.md) · [Report](../../milestones/milestone-04/reports/002-engine-implementation.md)

Captured 2026-10-15 (America/Los_Angeles), from the M04 working tree committed with these frames as d8441e7; approved M03 stable 7a0f859 is included. Native eframe/wgpu output only. Native display scale was 1.5 pixels/point; all captures use egui zoom 1 except 006/013/017/020, which use zoom 1.5 (effective 2.25). No claim of changing Windows DPI. Logical viewport 1000×680 produces 1500×1020 pixels; compact 480×420 produces 720×630.

Setup, production and rejection scenarios replay actual engine actions with synthetic seats. Contrast scenarios deliberately place sample pieces over all terrains and are not legal game positions. No private hands render. Runtime frames are not evidence of a full human input walkthrough. The user permitted game-window testing/captures. Only engine-rendered frames are stored here.

| Image | Viewport / image | Scenario and inspection |
| --- | --- | --- |
| [001-flat-ports.png](001-flat-ports.png) | 1000×680 / 1500×1020 | Initial flat ports, sample outline readable. |
| [002-setup-targets.png](002-setup-targets.png) | 1000×680 / 1500×1020 | Setup legal rings and active seat; earlier expanded panel. |
| [003-production.png](003-production.png) | 1000×680 / 1500×1020 | Engine placement/production; before ownership numerals. |
| [004-rejected-action.png](004-rejected-action.png) | 1000×680 / 1500×1020 | Rejected settlement explanation; earlier expanded panel. |
| [005-compact-ports.png](005-compact-ports.png) | 480×420 / 720×630 | Compact flat port/question-mark recognition. |
| [006-scaled-setup.png](006-scaled-setup.png) | 1000×680 / 1500×1020 | Earlier expanded setup at egui zoom 1.5. |
| [007-terrain-piece-contrast.png](007-terrain-piece-contrast.png) | 1000×680 / 1500×1020 | All-terrain renderer contrast fixture, including fields/mountains. |
| [008-compact-setup.png](008-compact-setup.png) | 480×420 / 720×630 | Earlier compact setup; panel covers part of board. |
| [009-zoomed-out.png](009-zoomed-out.png) | 1000×680 / 1500×1020 | Camera zoom 0.7, production; ships/pieces distinguishable. |
| [010-final-layout.png](010-final-layout.png) | 1000×680 / 1500×1020 | Current layout preview with sample pieces. |
| [011-numbered-production.png](011-numbered-production.png) | 1000×680 / 1500×1020 | Ownership numerals readable; earlier expanded panel. |
| [012-final-compact-test.png](012-final-compact-test.png) | 480×420 / 720×630 | Compact production before count-section reduction. |
| [013-final-scaled-test.png](013-final-scaled-test.png) | 1000×680 / 1500×1020 | Scaled production before count-section reduction. |
| [014-compact-contrast.png](014-compact-contrast.png) | 480×420 / 720×630 | Compact contrast fixture; light/dark outlines separate all terrains. |
| [015-compact-controls.png](015-compact-controls.png) | 480×420 / 720×630 | Duplicate compact panel iteration before successful count reduction. |
| [016-final-production.png](016-final-production.png) | 1000×680 / 1500×1020 | Duplicate production panel iteration before count reduction. |
| [017-scaled-controls.png](017-scaled-controls.png) | 1000×680 / 1500×1020 | Duplicate scaled panel iteration before count reduction. |
| [018-reduced-compact-controls.png](018-reduced-compact-controls.png) | 480×420 / 720×630 | Final compact: count section collapsed, actions fit; board partly covered by movable panel. |
| [019-reduced-production-controls.png](019-reduced-production-controls.png) | 1000×680 / 1500×1020 | Final production: numbered ownership, compact native control window. |
| [020-reduced-scaled-controls.png](020-reduced-scaled-controls.png) | 1000×680 / 1500×1020 | Final egui zoom 1.5: controls/text fit; movable panel covers some targets. |
Before/after: compare M03 [017](../milestone-03/017-wheel-right-pan.png) / [018](../milestone-03/018-wheel-compact.png) with M04 010/014 for ships and outline geometry. Compare M04 012/013 with 018/020 for reduced panel overlap. A movable overlay still covers targets at compact/scaled sizes; drag/collapse/close it to reach them. Human drag/keyboard-only placement and multi-screen privacy remain unverified.
