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

## 2026-10-16 screenshot-led composition, artwork and toolbar

Source: M04 working tree after 2ef2771, committed with these frames. Actual game-rendered frames, synthetic seats, existing frame-only permission. Native scale 1.5; egui zoom 1 except filenames containing scaled at 1.5 (effective 2.25). Before: 052/056/057. Private hands stay covered. Earlier work frames are retained, with final evidence 078–082. Intermediate 073–077 exposed edge-tight ships; camera fit corrected afterward.

| Image | Date | Viewport / image | Inspection |
| --- | --- | --- | --- |
| [058-full-layout](058-full-layout.png) | 2026-10-16 | 1000×680 / 1500×1020 | Intermediate composition/art iteration; superseded, not final acceptance evidence. |
| [059-full-layout-compact](059-full-layout-compact.png) | 2026-10-16 | 480×420 / 720×630 | Intermediate composition/art iteration; superseded, not final acceptance evidence. |
| [060-full-layout-scaled](060-full-layout-scaled.png) | 2026-10-16 | 1000×680 / 1500×1020 | Intermediate composition/art iteration; superseded, not final acceptance evidence. |
| [061-reference-composition](061-reference-composition.png) | 2026-10-16 | 1000×680 / 1500×1020 | Intermediate composition/art iteration; superseded, not final acceptance evidence. |
| [062-reference-composition-compact](062-reference-composition-compact.png) | 2026-10-16 | 480×420 / 720×630 | Intermediate composition/art iteration; superseded, not final acceptance evidence. |
| [063-reference-composition-scaled](063-reference-composition-scaled.png) | 2026-10-16 | 1000×680 / 1500×1020 | Intermediate composition/art iteration; superseded, not final acceptance evidence. |
| [064-reference-final](064-reference-final.png) | 2026-10-16 | 1000×680 / 1500×1020 | Intermediate composition/art iteration; superseded, not final acceptance evidence. |
| [065-reference-final-compact](065-reference-final-compact.png) | 2026-10-16 | 480×420 / 720×630 | Intermediate composition/art iteration; superseded, not final acceptance evidence. |
| [066-reference-final-scaled](066-reference-final-scaled.png) | 2026-10-16 | 1000×680 / 1500×1020 | Intermediate composition/art iteration; superseded, not final acceptance evidence. |
| [067-reference-final-error](067-reference-final-error.png) | 2026-10-16 | 1000×680 / 1500×1020 | Intermediate composition/art iteration; superseded, not final acceptance evidence. |
| [068-classic-ui](068-classic-ui.png) | 2026-10-16 | 1000×680 / 1500×1020 | Intermediate composition/art iteration; superseded, not final acceptance evidence. |
| [069-classic-ui-compact](069-classic-ui-compact.png) | 2026-10-16 | 480×420 / 720×630 | Intermediate composition/art iteration; superseded, not final acceptance evidence. |
| [070-classic-ui-scaled](070-classic-ui-scaled.png) | 2026-10-16 | 1000×680 / 1500×1020 | Intermediate composition/art iteration; superseded, not final acceptance evidence. |
| [071-classic-ui-error](071-classic-ui-error.png) | 2026-10-16 | 1000×680 / 1500×1020 | Intermediate composition/art iteration; superseded, not final acceptance evidence. |
| [072-classic-ui-wide](072-classic-ui-wide.png) | 2026-10-16 | 1440×820 / 2160×1230 | Intermediate composition/art iteration; superseded, not final acceptance evidence. |
| [073-reference-game](073-reference-game.png) | 2026-10-16 | 1000×680 / 1500×1020 | Inspected intermediate: top or left ships too close to the board edge; superseded by fit correction. |
| [074-reference-game-compact](074-reference-game-compact.png) | 2026-10-16 | 480×420 / 720×630 | Inspected intermediate: top or left ships too close to the board edge; superseded by fit correction. |
| [075-reference-game-scaled](075-reference-game-scaled.png) | 2026-10-16 | 1000×680 / 1500×1020 | Inspected intermediate: top or left ships too close to the board edge; superseded by fit correction. |
| [076-reference-game-error](076-reference-game-error.png) | 2026-10-16 | 1000×680 / 1500×1020 | Inspected intermediate: top or left ships too close to the board edge; superseded by fit correction. |
| [077-reference-game-wide](077-reference-game-wide.png) | 2026-10-16 | 1440×820 / 2160×1230 | Inspected intermediate: top or left ships too close to the board edge; superseded by fit correction. |
| [078-reference-fit](078-reference-fit.png) | 2026-10-16 | 1000×680 / 1500×1020 | Inspected final: left toolbar, bank/roster, covered hand and original reference-led artwork fit. |
| [079-reference-fit-compact](079-reference-fit-compact.png) | 2026-10-16 | 480×420 / 720×630 | Inspected compact: full island/ships visible; bank has five resource types and separate dev deck. |
| [080-reference-fit-scaled](080-reference-fit-scaled.png) | 2026-10-16 | 1000×680 / 1500×1020 | Inspected egui zoom 1.5: toolbar, card slots, actions and full ports fit. |
| [081-reference-fit-error](081-reference-fit-error.png) | 2026-10-16 | 1000×680 / 1500×1020 | Inspected rejection: error readable; no clipping of actions or island. |
| [082-reference-fit-wide](082-reference-fit-wide.png) | 2026-10-16 | 1440×820 / 2160×1230 | Inspected wide: full board/ships, proportional right panel, complete roster and action dock. |

## 2026-10-16–17 measured reference checkpoint

Working tree after a16c9de. Actual native wgpu frames, synthetic engine scenario, covered private hands. Native pixels/point 1.5. Source includes one official Colonist wool SVG from the public press-kit folder; other terrain remains generated. See report 010 and asset provenance. Exact-reference identity is **not achieved**. Native 2000 × 1135 physical-pixel captures allow direct comparison with the supplied reference, without resizing images.

| Image | Capture date | Requested size / actual image | Inspection |
| --- | --- | --- | --- |
| [083-measured-layout](083-measured-layout.png) | 2026-10-16 | 2000×1135 physical / 2000×1135 | Inspected first measured layout; board too large and top ships cropped; superseded. EgUI zoom 1; effective 1.5. |
| [084-measured-surface](084-measured-surface.png) | 2026-10-16 | 2000×1135 physical / 2000×1135 | Inspected corrected board scale and action surfaces; not exact artwork. EgUI zoom 1; effective 1.5. |
| [085-measured-canonical](085-measured-canonical.png) | 2026-10-16 | 2000×1135 physical / 2000×1135 | Intermediate layout/camera/capture iteration; superseded, not final acceptance evidence. EgUI zoom 1; effective 1.5. |
| [086-measured-compact](086-measured-compact.png) | 2026-10-16 | 480×420 logical / 720×630 | Intermediate layout/camera/capture iteration; superseded, not final acceptance evidence. EgUI zoom 1; effective 1.5. |
| [087-measured-scaled](087-measured-scaled.png) | 2026-10-16 | 2000×1135 physical / 3000×1703 | Intermediate egui zoom 1.5: physical-size bug produced 3000×1703; superseded by 091/099. EgUI zoom 1.5; effective 2.25. |
| [088-measured-error](088-measured-error.png) | 2026-10-16 | 2000×1135 physical / 2000×1135 | Intermediate layout/camera/capture iteration; superseded, not final acceptance evidence. EgUI zoom 1; effective 1.5. |
| [089-measured-toolbar](089-measured-toolbar.png) | 2026-10-17 | 2000×1135 physical / 2000×1135 | Inspected closer left-tool shapes; generated terrain still differs. EgUI zoom 1; effective 1.5. |
| [090-official-wool](090-official-wool.png) | 2026-10-17 | 2000×1135 physical / 2000×1135 | Inspected official wool integration and reference geometry; superseded by final neutral-pointer series. EgUI zoom 1; effective 1.5. |
| [091-physical-scaled](091-physical-scaled.png) | 2026-10-17 | 2000×1135 physical / 2000×1135 | Inspected official wool integration and reference geometry; superseded by final neutral-pointer series. EgUI zoom 1.5; effective 2.25. |
| [092-native-compact](092-native-compact.png) | 2026-10-17 | 480×420 logical / 720×630 | Inspected official wool integration and reference geometry; superseded by final neutral-pointer series. EgUI zoom 1; effective 1.5. |
| [093-native-error](093-native-error.png) | 2026-10-17 | 2000×1135 physical / 2000×1135 | Inspected official wool integration and reference geometry; superseded by final neutral-pointer series. EgUI zoom 1; effective 1.5. |
| [094-measured-neutral](094-measured-neutral.png) | 2026-10-17 | 2000×1135 physical / 2000×1135 | Intermediate capture; neutral filename preceded the final neutral-pointer implementation. Not final acceptance evidence. EgUI zoom 1; effective 1.5. |
| [095-measured-neutral-scaled](095-measured-neutral-scaled.png) | 2026-10-17 | 2000×1135 physical / 2000×1135 | Intermediate capture; neutral filename preceded the final neutral-pointer implementation. Not final acceptance evidence. EgUI zoom 1.5; effective 2.25. |
| [096-measured-neutral-compact](096-measured-neutral-compact.png) | 2026-10-17 | 480×420 logical / 720×630 | Intermediate capture; neutral filename preceded the final neutral-pointer implementation. Not final acceptance evidence. EgUI zoom 1; effective 1.5. |
| [097-measured-neutral-error](097-measured-neutral-error.png) | 2026-10-17 | 2000×1135 physical / 2000×1135 | Intermediate capture; neutral filename preceded the final neutral-pointer implementation. Not final acceptance evidence. EgUI zoom 1; effective 1.5. |
| [098-neutral-canonical](098-neutral-canonical.png) | 2026-10-17 | 2000×1135 physical / 2000×1135 | Inspected normal: measured layout, original wool SVG and covered hands; strict identity false. EgUI zoom 1; effective 1.5. |
| [099-neutral-scaled](099-neutral-scaled.png) | 2026-10-17 | 2000×1135 physical / 2000×1135 | Inspected egui zoom 1.5 (effective 2.25): correct 2000×1135 output, controls/roster fit. EgUI zoom 1.5; effective 2.25. |
| [100-neutral-compact](100-neutral-compact.png) | 2026-10-17 | 480×420 logical / 720×630 | Inspected compact: working compact layout retained; all player rows and actions visible. EgUI zoom 1; effective 1.5. |
| [101-neutral-error](101-neutral-error.png) | 2026-10-17 | 2000×1135 physical / 2000×1135 | Inspected rejection: selected target and error visible; private hand stays covered. EgUI zoom 1; effective 1.5. |

[Initial comparison report](090-reference-comparison.json) · [Final strict comparison report](098-reference-comparison.json). The --require-exact command exits 1. Only 648163 of 2270000 pixels match exactly; this is not a similarity score and includes different scene state. No pixel-identity acceptance is claimed.
