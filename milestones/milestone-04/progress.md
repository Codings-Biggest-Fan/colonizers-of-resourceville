# Milestone 04 progress — Base turn engine

Status: Verifying  
Target release: v0.0.4 (no release due)  
Last updated: 2026-10-15 (America/Los_Angeles)

[Plan](plan.md) · [Reports](reports/) · [Photos](../../photos/milestone-04/index.md) · [Roadmap](../../ROADMAP.md)

## Implemented

- Deterministic setup, snake order, second-settlement starting resources and explicit roll/build/pending-robber phases.
- Supplied dice outcomes, finite-bank shortages, resource conservation, building costs, distance/connectivity, opponent blocking and all piece limits.
- Seat-specific legal choices and clone-and-commit rejected-action atomicity. Authoritative hands have no Debug/Serialize; public projection contains counts only.
- Native test harness drives board pieces, legal targets, active player, phase, supplied dice and errors. F4/context menu opens the closable panel; seed regeneration resets the test.
- Flat port ships, generic question marks and strong outlined player pieces. Light sea, wheel zoom and right-drag pan preserved.

## Verification

- 28 workspace tests passed, including 96 seeded setup/replay cases, 120 ordinary replay turns, shortages, costs, piece limits, blocking, invalid outcomes, atomicity, native bridge and existing camera/input/contract tests.
- Final scripts/check.ps1 -Release passed formatting, all-feature Clippy and optimized workspace build. scripts/test.ps1 passed 28 tests; record/link checks passed. See report 002.
- Native wgpu frames captured with permission; metadata and inspection are in the photo index. Synthetic scenarios are replayed through the rules engine; contrast fixtures are not legal games.
- M03 PR #10 is merged into stable (7a0f859), confirmed from fetched Git history. This branch merges that accepted baseline without losing later planning records.

## Remaining checks

- Six-target GitHub build/package review remains pending. Local Windows package CRC/sidecar/all 649 hashes passed; [PR #11](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/pull/11) was user-approved and merged into beta at f5548fd. See [handoff report](reports/003-testing-handoff.md).
- Full human/native input walkthrough, including panel drag/collapse at compact size and keyboard-only placement, remains unverified. Existing headless camera input and native bridge tests do not establish human usability.
- Multiple-people/screens privacy and non-Windows interactive runtime remain unverified. No online functionality or complete-game claim.

## Limits

Seven waits for M05; no trading, cards, awards, victory, bots, saves, rooms or cloud. Explicit test dice and synthetic seats are a developer harness, not a supported shared-device mode. Unsupported runtime options are rejected. No protocol/save shape changes. The current native sidebar and bottom dock reserve board space, replacing the older overlay. Development-card/trade icons are disabled until M05.

## Blockers and next step

No source implementation blocker. User approved the current implementation baseline for beta; refinement is deferred. Finish evidence/CI review and remaining native acceptance when work resumes. Do not promote stable/main or begin M05. No release/tag is due.

## Reports

[001 — Requested visual work](reports/001-visual-follow-up-requirements.md) · [002 — Engine implementation and checks](reports/002-engine-implementation.md)


[003 — Testing handoff](reports/003-testing-handoff.md).

[004 — Approved beta promotion](reports/004-approved-beta-promotion.md).

## Scoped visual correction — 2026-10-16

- [x] Replace crowded, shaded tile motifs with simpler flat cartoon terrain and safe edge margins.
- [x] Round settlement/city silhouettes and simplify architectural detail while retaining ownership/contrast.
- [x] Inspect native default/compact/contrast/scaling frames and run affected Rust checks.

This is the user's scoped visual correction, not an indefinite Polish mode or approval to change beta/stable/main. Native frame-only capture was explicitly permitted for this check. Gameplay rules, board topology and controls remain outside the correction.

[005 — Softer terrain and rounded houses](reports/005-polish-terrain-and-houses.md). Native frames 021–024 inspected; formatting/Clippy passed. All five desktop regression tests passed.

## Scoped HUD correction — 2026-10-16

- [x] Replace the old floating window with a native sidebar and bottom dock.
- [x] Apply user feedback: graphic-first controls, minimal visible text and no extra public counters/house numerals.
- [x] Include road/settlement/city/card/trade/dice/end-turn graphics; unavailable M05 controls stay disabled with truthful tooltips.
- [x] Inspect default, compact, scaled and rejected-action frames; pass formatting, Clippy and six desktop regressions.

[006 — Graphic HUD](reports/006-polish-graphic-hud.md). Current evidence: frames 043–046. This request supersedes earlier no-bars design guidance within M04, without approving promotion or starting M05.

## HUD count follow-up — 2026-10-16

- [x] Show each player's public resource-card total with graphical resource/dev-card slots.
- [x] Mark the development-card count unavailable (M05), preserving unknown versus zero.
- [x] Show authoritative remaining road/settlement/city stock on build icons.
- [x] Pass seven desktop regressions and formatting/Clippy; inspect default/compact/scaled output.

The user's latest request reintroduces these specific functional counts while preserving the graphic-first layout. [Report 007](reports/007-polish-card-and-piece-counts.md); final frames 047/050/051. No beta/stable promotion or M05 implementation.

## Light HUD follow-up — 2026-10-16

- [x] Apply Catppuccin Latte surfaces and compact horizontal player rows.
- [x] Shorten the bottom action dock; retain requested card/piece counts and truthful disabled controls.
- [x] Add bounded public accepted-action history without private details.
- [x] Inspect normal/error/compact/scaled native frames and pass eight desktop tests plus formatting/Clippy.

[Report 008](reports/008-polish-light-reference-layout.md). Final evidence: 052/055/056/057; intermediate 053/054 clipping corrected. No channel promotion or M05 implementation.

## Screenshot-led layout follow-up — 2026-10-16

- [x] Match supplied composition: activity/chat region, bank, opponent rows, active seat, bottom cards/build actions and left tools.
- [x] Replace interim flat glyphs with generated reference-led terrain art and curved-sail harbor ships; preserve non-producing desert.
- [x] Add intentional hand reveal with automatic covering, truthful unavailable feature states and native guide/options.
- [x] Pass nine desktop tests and formatting/Clippy; inspect normal, compact, scaled, error and wide native frames.

[Report 009](reports/009-polish-screenshot-layout-and-art.md). Final frames 078–082; intermediate 058–077 preserved. This supersedes report 008's arrangement. M04 remains Verifying, with no promotion or roadmap advancement.
