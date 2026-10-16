# 007 — HUD card and piece counts

Date: 2026-10-16 (America/Los_Angeles). Scoped M04 correction; status remains Verifying.

The user requested visible card/development-card counts and other useful counts after the graphic-first HUD refinement. Added original small resource/development-card pictograms below each player portrait. Resource totals come directly from the public engine view. Development-card totals display an em dash and a tooltip explaining M05 availability: M04 has no development-card state, so no numeric value is fabricated. Hidden card types and private points remain undisclosed.

Build icons now show the active player's remaining road/settlement/city pieces, derived from authoritative placed pieces and the existing limits. Values update with setup and player changes; no fake score, deck balance or victory tally was introduced. Kept the graphic rail and icon-first controls. Compact rail rows were tightened after inspection so fit/reset stay visible.

Validation: scripts/check.ps1 passed formatting and all-feature workspace Clippy. Seven desktop tests passed, including a new regression that derives stock from real setup transitions (15/5/4 before setup; 13/3/4 after two placements), unknown/unimplemented counts, active-player state and the prior geometry/camera/bridge coverage. No rules/state format changes or new dependencies.

Inspected native frames 047–051 (normal, compact and scaled); 048/049 preserve the first spacing iteration, while 050/051 show the final compact/scaled rail fit. All use the game's own wgpu frame under the existing HUD capture permission with synthetic seats. Native scale 1.5; egui zoom 1.5 only for 049/051. No full manual input, multiplayer privacy, OS-DPI change or nonlocal runtime claim.

Changes stay on milestone-04/base-turn-engine and nightly. Beta remains the approved f5548fd baseline, stable/main unchanged. Development cards/trades remain M05; no milestone advancement, release or promotion authorized.
