# 009 — Screenshot-led game layout and graphics

Date: 2026-10-16 (America/Los_Angeles). Source: working tree after 2ef2771; implementation and evidence committed together.

## User correction

The user rejected the first light HUD as too different, supplied a full gameplay screenshot, requested much closer tiles/ships, and identified the top-left toolbar and the absence of a desert resource. This session supersedes report 008's composition.

## Changes

- Full-height right panel: public activity above a separate unavailable chat area, a graphical bank strip, compact opponent rows, and active synthetic seat last. Normal/wide rows show actual remaining roads, settlements and cities. Compact rows retain public resource/dev-card slots; piece stock remains on active build buttons.
- Bottom card area and ordered trade/road/settlement/city/development/end controls; dice and phase above the actions. Board fills more of its available area while reserving toolbar, sidebar and bottom dock geometry.
- New embedded terrain-v5 atlas, generated with the built-in image_gen tool from the user's visual reference. [Exact prompt and provenance](../../../assets/art/terrain-v5.prompt.md), [asset](../../../assets/art/terrain-v5.png). Rounded sails, golden hulls, explicit harbor ratios and softer faceted houses are repository-authored geometry. Earlier assets/evidence preserved.
- Top-left back, guide, audio, board-options and headset graphics. Back returns to board-only inspection (F4 restores HUD); guide and seed/fit options are implemented native windows. Audio/support are disabled with truthful tooltips.
- Bank uses the actual five-resource inventory plus a separate unavailable development deck. Desert is a non-producing terrain only. No desert card was introduced.
- Clicking the covered hand intentionally reveals only the active synthetic seat's actual resource types. Click again to cover; accepted turn change, new test or focus loss covers automatically. Runtime captures remain covered. This remains a synthetic-seat harness, not a supported shared-device mode.

## Reference and limits

The [public press kit](https://colonist.io/press-kit) and user screenshot informed composition. No asset redistribution license was verified in the official pages; no extracted artwork/source or competitor marks are shipped. New generated graphics are visual-reference-based project artwork, not a claim of legal clearance. Original product branding remains.

Chat, audio, support, dev-card rules, robber resolution, awards, scores and timers are not fabricated. Unavailable controls stay disabled; M05 and later gameplay remain outside this request. Light theme is fixed presentation, not implemented persistent appearance settings.

## Validation

- Nine desktop tests passed, including automatic private-hand covering on new test/turn change/focus loss and existing board-space, input and action regressions.
- Formatting and workspace/all-target/all-feature Clippy passed. Compiler incremental hard-link fallback warnings copied files; no code lint failures remain.
- Final native frame checks: 078 normal, 079 compact, 080 egui zoom 1.5, 081 rejected action, 082 wide. Frame metadata and intermediate 058–077 are in the [photo index](../../../photos/milestone-04/index.md).
- Existing explicit game-frame-only permission used. Synthetic seats; no unrelated window capture, physical input walkthrough or change to Windows DPI. Native scaling 1.5; egui scaled stress effective 2.25.
- Privacy cover has automated coverage. Opening guide/options through physical input, full keyboard traversal, audio/support and nonlocal runtime remain unverified.

## Handoff

Keep M04 Verifying; broader milestone gates remain pending. Publish development only to milestone-04/base-turn-engine and nightly. No beta/stable/main promotion, release or M05 start.
