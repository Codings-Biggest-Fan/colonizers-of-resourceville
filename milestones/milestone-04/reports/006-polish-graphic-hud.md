# 006 — Polish: graphic sidebar and action dock

Date: 2026-10-16 (America/Los_Angeles). M04 remains Verifying.

## Flaws and correction

The user requested a sidebar and bottom bar, then rejected the initial text-heavy/stat-heavy iterations as bland. The final direction is graphics first (the user's 5/95 rule), with a compact game-first composition informed by the [public Colonist press screenshots](https://colonist.io/press-kit). This is layout reference only; no competitor assets, marks, source, private APIs or product names are embedded in the application.

Replaced the floating test window with native docked panels. A narrow right player rail uses original rounded colored house portraits and a visible active-player outline, plus actual dice pips. Bottom controls use original road, settlement, city, development-card stack, trade arrows, dice/play and finish-arrow graphics. Fit/reset have graphical controls. Visible player/structure/resource counters, dice sum and house numerals were removed; tile production numbers remain. Names, control explanations and costs live in accessible names and hover tooltips. One short phase cue and actionable errors remain in the dock.

Roads, settlements, cities, start, supplied dice, end-turn, fit and reset use the real existing pipeline. Development-card and trade controls are disabled with explicit M05 availability tooltips; their rules have not been implemented or claimed. No fake hands, chat, room data, scores, currencies or controls for absent gameplay are rendered. Authoritative rules/state formats did not change. F4/context menu remains available, and the HUD is visible by default. Public ownership remains inspectable in text, so color is not the sole identification mechanism.

This request explicitly supersedes the earlier no-bars instruction for the current M04 HUD. It does not approve beta/stable promotion or M05 implementation.

## Validation

- scripts/check.ps1 passed formatting and all-target/all-feature workspace Clippy with warnings denied.
- Six desktop tests passed: existing camera/input, transformed hit targets, seed recovery and native action bridge, plus a new normal/compact/scaled viewport regression proving the dock reserves board space and starts real setup state.
- User explicitly permitted this native frame-only check. Inspected final captures 043–046: actual wgpu default production, compact setup, scaled production and visible rejection/recovery. Native scale 1.5; 045 has egui zoom 1.5 (effective 2.25). No Windows DPI setting change or full human input session is claimed.
- Earlier captures 025–042 preserve the interim designs; they are not current acceptance screenshots. Final 043–046 are the current inspected evidence. Before evidence is the floating-window capture 024.
- Initial UTF-8, test-context and Clippy issues were corrected before the final passing gates. No dependency/tool installation.

## Limits and handoff

The 5/95 instruction is applied as a graphic-first design target, not a measured pixel ratio. M04 seven resolution, trades, cards, victory, bots, saves and online remain later scope. Manual multi-screen privacy and nonlocal interactive rendering remain unverified. This correction is on milestone-04/base-turn-engine and nightly only; beta stays at approved f5548fd and stable/main are unchanged. No release or new milestone work.
