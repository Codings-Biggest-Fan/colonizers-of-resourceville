# 008 — Light HUD layout refinement

Date: 2026-10-16 (America/Los_Angeles). Source: M04 working tree after b57d806; implementation and evidence committed together.

## Request and correction

The user requested a closer familiar game layout in light mode after finding the prior HUD unappealing. Rebuilt the presentation with Catppuccin Latte surfaces, compact horizontal player rows on the right, public activity below the roster at normal sizes, and a shorter bottom action bar aligned right. Original artwork, branding and native Rust implementation remain. Layout reference: [public gameplay press kit](https://colonist.io/press-kit); no source or assets copied.

Public resource counts and remaining road/settlement/city stock remain visible. Development-card counts remain unavailable and card/trade controls disabled pending M05. Activity records only accepted public actions, is capped at 12 entries and clears on a new test. No private hands or dice outcomes are added to the log. Compact/scaled views put dice below the heading and omit activity to preserve board space.

## Validation

- Formatting and workspace/all-target/all-feature Clippy passed through scripts/check.ps1.
- Eight desktop tests passed, including accepted-action-only bounded history and existing responsive board-space/input tests.
- Actual game-frame captures inspected: 052 normal, 055 rejected action, 056 compact and 057 scaled. Intermediate 053/054 exposed clipped dice; corrected in 056/057. Before: 047/050/051.
- Native frame-only testing used the user's existing permission. No unrelated windows captured. No physical keyboard/mouse interaction session performed.
- Compiler incremental hard-link fallback warnings used file copying; checks completed successfully.

## Scope and next steps

Presentation and public test-harness history only; no M05 gameplay, persistence format, beta/stable/main promotion or release. M04 remains Verifying with broader acceptance tracked in progress.md. Continue from user visual feedback; do not advance the roadmap.
