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

## 2026-10-16 visual correction

Source: M04 working tree after 6319c2f, committed together with these captures. Permission explicitly granted for game-rendered-frame-only captures. No unrelated windows or private hands captured. Before evidence: [010 layout](010-final-layout.png), [014 contrast](014-compact-contrast.png), [020 scaled test](020-reduced-scaled-controls.png). Terrain v4 and rounded silhouette geometry replace the earlier graphics; no rules/control/topology change.

| Image | Capture date | Viewport / image | Scaling | Scenario and inspection |
| --- | --- | --- | --- | --- |
| [021 terrain/houses](021-soft-terrain-houses.png) | 2026-10-16 | 1000×680 / 1500×1020 | Native 1.5; egui 1 | Smaller, flatter terrain motifs leave clear token centers; house has visibly rounded peak/corners and retains outline contrast. |
| [022 compact](022-soft-compact.png) | 2026-10-16 | 480×420 / 720×630 | Native 1.5; egui 1 | Compact layout; terrain families, numbers, ports and rounded sample piece distinguishable. |
| [023 contrast fixture](023-soft-contrast.png) | 2026-10-16 | 1000×680 / 1500×1020 | Native 1.5; egui 1 | Renderer fixture, not legal gameplay; rounded houses and paired-house cities over all terrain. |
| [024 scaled production](024-soft-scaled-production.png) | 2026-10-16 | 1000×680 / 1500×1020 | Native 1.5; egui 1.5; effective 2.25 | Real engine replay; ownership markings, outlines and existing controls preserved. Movable panel still covers some board area. |

No claim of changing Windows display settings, manual input walkthrough or multiplayer/privacy acceptance.

## 2026-10-16 graphic HUD

Source: M04 working tree after 67e7026, committed together with these frames. User granted frame-only permission for the sidebar/bottom-bar check. The final direction follows their graphic-first correction; intermediate drafts remain archived. All captures are native eframe/wgpu, not mockups. Native scale 1.5. Scaled captures use egui zoom 1.5 (effective 2.25); all others use zoom 1. No OS DPI-change claim. Final acceptance images are 043–046; before is 024. None proves a complete human input session or multiplayer privacy.

| Image | Date | Viewport / image | Scenario / inspection |
| --- | --- | --- | --- |
| [025-sidebar-preview.png](025-sidebar-preview.png) | 2026-10-16 | 1000×680 / 1500×1020 | Inspected intermediate HUD iteration; superseded by 043–046 after user feedback. |
| [026-sidebar-turn.png](026-sidebar-turn.png) | 2026-10-16 | 1000×680 / 1500×1020 | Inspected intermediate HUD iteration; superseded by 043–046 after user feedback. |
| [027-sidebar-compact.png](027-sidebar-compact.png) | 2026-10-16 | 480×420 / 720×630 | Inspected intermediate HUD iteration; superseded by 043–046 after user feedback. |
| [028-sidebar-scaled.png](028-sidebar-scaled.png) | 2026-10-16 | 1000×680 / 1500×1020 | Inspected intermediate HUD iteration; superseded by 043–046 after user feedback. EgUI zoom 1.5. |
| [029-sidebar-rejection.png](029-sidebar-rejection.png) | 2026-10-16 | 1000×680 / 1500×1020 | Inspected intermediate HUD iteration; superseded by 043–046 after user feedback. |
| [030-piece-roster-hud.png](030-piece-roster-hud.png) | 2026-10-16 | 1000×680 / 1500×1020 | Inspected intermediate HUD iteration; superseded by 043–046 after user feedback. |
| [031-piece-roster-compact.png](031-piece-roster-compact.png) | 2026-10-16 | 480×420 / 720×630 | Inspected intermediate HUD iteration; superseded by 043–046 after user feedback. |
| [032-piece-roster-scaled.png](032-piece-roster-scaled.png) | 2026-10-16 | 1000×680 / 1500×1020 | Inspected intermediate HUD iteration; superseded by 043–046 after user feedback. EgUI zoom 1.5. |
| [033-action-dock-error.png](033-action-dock-error.png) | 2026-10-16 | 1000×680 / 1500×1020 | Intermediate comparison capture; not inspected for final acceptance. |
| [034-game-hud-preview.png](034-game-hud-preview.png) | 2026-10-16 | 1000×680 / 1500×1020 | Intermediate comparison capture; not inspected for final acceptance. |
| [035-compact-game-hud.png](035-compact-game-hud.png) | 2026-10-16 | 1000×680 / 1500×1020 | Inspected intermediate HUD iteration; superseded by 043–046 after user feedback. |
| [036-compact-game-hud-small.png](036-compact-game-hud-small.png) | 2026-10-16 | 480×420 / 720×630 | Inspected intermediate HUD iteration; superseded by 043–046 after user feedback. |
| [037-compact-game-hud-scaled.png](037-compact-game-hud-scaled.png) | 2026-10-16 | 1000×680 / 1500×1020 | Inspected intermediate HUD iteration; superseded by 043–046 after user feedback. EgUI zoom 1.5. |
| [038-compact-game-hud-error.png](038-compact-game-hud-error.png) | 2026-10-16 | 1000×680 / 1500×1020 | Inspected intermediate HUD iteration; superseded by 043–046 after user feedback. |
| [039-compact-game-preview.png](039-compact-game-preview.png) | 2026-10-16 | 1000×680 / 1500×1020 | Inspected intermediate HUD iteration; superseded by 043–046 after user feedback. |
| [040-final-action-dock.png](040-final-action-dock.png) | 2026-10-16 | 1000×680 / 1500×1020 | Intermediate comparison capture; not inspected for final acceptance. |
| [041-final-dock-error.png](041-final-dock-error.png) | 2026-10-16 | 1000×680 / 1500×1020 | Intermediate comparison capture; not inspected for final acceptance. |
| [042-final-compact-build.png](042-final-compact-build.png) | 2026-10-16 | 480×420 / 720×630 | Intermediate comparison capture; not inspected for final acceptance. |
| [043-graphic-hud.png](043-graphic-hud.png) | 2026-10-16 | 1000×680 / 1500×1020 | Final graphic rail/dock: original action icons; only short phase text; production and player selection clear. |
| [044-graphic-hud-compact.png](044-graphic-hud-compact.png) | 2026-10-16 | 480×420 / 720×630 | Final compact setup: all portraits and controls fit; board targets remain outside HUD. |
| [045-graphic-hud-scaled.png](045-graphic-hud-scaled.png) | 2026-10-16 | 1000×680 / 1500×1020 | Final egui zoom 1.5: graphic controls/portraits readable, no panel overlap. |
| [046-graphic-hud-error.png](046-graphic-hud-error.png) | 2026-10-16 | 1000×680 / 1500×1020 | Final rejected action: error visibly fits beneath graphic actions; no state change. |

## 2026-10-16 card/piece count follow-up

Source: M04 working tree after 1274e0b, committed together with these images. Existing HUD frame-only permission; actual wgpu output, synthetic seats. Native scale 1.5; egui zoom 1 except 049/051 at 1.5 (effective 2.25). Before is 043. All five captures inspected; 047/050/051 are final evidence. No hidden card types or private points displayed.

| Image | Date | Viewport / image | Inspection |
| --- | --- | --- | --- |
| [047 card counts](047-card-counts.png) | 2026-10-16 | 1000×680 / 1500×1020 | Public resource totals, unavailable development-card slot and real active-player piece stock readable. |
| [048 compact spacing](048-card-counts-compact.png) | 2026-10-16 | 480×420 / 720×630 | Before spacing adjustment: counters readable, fit/reset lower edge tight. Superseded by 050. |
| [049 scaled spacing](049-card-counts-scaled.png) | 2026-10-16 | 1000×680 / 1500×1020 | EgUI zoom 1.5; first count spacing, superseded by 051. |
| [050 compact fit](050-card-counts-compact-fit.png) | 2026-10-16 | 480×420 / 720×630 | Final compact rows: card slots, portraits and fit/reset fully visible; board remains separate. |
| [051 scaled fit](051-card-counts-scaled-fit.png) | 2026-10-16 | 1000×680 / 1500×1020 | Final egui zoom 1.5; counters/icons fit with preserved dock and no panel overlap. |

## 2026-10-16 light HUD follow-up

Source: M04 working tree after b57d806, committed with these frames. Actual game-rendered output, synthetic seats, existing frame-only permission. Native scale 1.5; egui zoom 1 except 054/057 at 1.5 (effective 2.25). Before: 047/050/051. All six inspected; final evidence 052/055/056/057.

| Image | Date | Viewport / image | Inspection |
| --- | --- | --- | --- |
| [052 normal light HUD](052-light-reference-layout.png) | 2026-10-16 | 1000×680 / 1500×1020 | Horizontal player rows, public counts/activity and short action bar readable. |
| [053 compact before correction](053-light-reference-compact.png) | 2026-10-16 | 480×420 / 720×630 | Dice row clips at right; superseded by 056. |
| [054 scaled before correction](054-light-reference-scaled.png) | 2026-10-16 | 1000×680 / 1500×1020 | EgUI zoom 1.5; dice row clips; superseded by 057. |
| [055 rejected action](055-light-reference-error.png) | 2026-10-16 | 1000×680 / 1500×1020 | Dark red actionable error fits in light dock. |
| [056 compact corrected](056-light-reference-compact-fit.png) | 2026-10-16 | 480×420 / 720×630 | Dice, player rows, counts and controls fit; board remains unobstructed. |
| [057 scaled corrected](057-light-reference-scaled-fit.png) | 2026-10-16 | 1000×680 / 1500×1020 | EgUI zoom 1.5; all HUD content fits; no board overlap. |
