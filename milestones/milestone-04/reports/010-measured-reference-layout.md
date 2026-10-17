# 010 — Measured reference layout; exact-match goal remains open

Date: work continued 2026-10-16–17 (America/Los_Angeles). Source: working tree after a16c9de. This is a development checkpoint, not completion of the user's exact-match goal.

## Goal and evidence baseline

The user requires: “Make it look exactly the same 1 to 1 apples 2 apples no singular difference.” The supplied reference is 2000 × 1135. Earlier work changed authoritative state and produced inspected frames, so the previous goal turn was progress. Current state, not earlier aesthetic claims, governs this checkpoint.

## Implemented progress

- Replaced approximate desktop HUD placement with a measured native layout module. At the reference dimensions: sidebar 451 pixels, board-right boundary x=1549, bottom dock 110 pixels, opponent rows 107 pixels, active row 168 pixels, bank 109 pixels; large dice and turn strip sit over lower-right sea. Narrow windows retain the compact layout.
- Sampled reference sea RGB 7/103/166, cream panel gradients and muted opponent surfaces. Added water-ring outlines, rounded aqua build controls, hourglass end-turn icon and closer left-tool silhouettes. These are measured/recreated visuals, not an assertion that all artwork is exact.
- Added Open Sans from Google Fonts (public site body font observed) and Lilita One for board numerals (visually matched candidate; original game's numeral-font identity unverified). Both unmodified font files and their SIL OFL notices are embedded/recorded, and packaging preserves their notices.
- Added a pinned pure-Rust SVG renderer. The individually downloaded official tile_wool.svg now supplies pasture tiles and the normal HUD's wool card image. Transparency, green fill and raster dimensions have a regression test. It is Colonist artwork, not project-original art; provenance and SHA256 are recorded under assets/reference/.
- Preserved engine validation, public count/private hand boundaries, supplied dice and reset controls. Nonfunctional phases are not clickable turn actions. Dice editing and test reset are available through Board options.
- Added physical-pixel frame capture. It now produces 2000 × 1135 even at egui zoom 1.5. Captures use a neutral pointer to avoid incidental desktop-cursor hover changes. This is real game rendering, not a screenshot backdrop.
- Added a read-only same-size RGB comparison script. It does not resize, align or edit either image. --require-exact is a strict identity gate, separate from successful report generation.

## Official asset access and denial

The user specifically approved inspection of the official Google Drive folder linked from the public press kit, then granted broad ongoing access. Inspection confirmed public tile/shore/UI asset files. One wool SVG downloaded successfully before a later full-pack attempt. The browser's full-pack download gate denied authority for takeout-download-drive.usercontent.google.com and reported declined approval. The pack was not acquired; no alternate download route was attempted. The Drive preparation UI no longer showed an active transfer. There is no currently live pack-download job to wait on.

No permissive redistribution license was verified in the inspected material. Provenance states that limitation without calling the art project-original or asserting affiliation. No release or approved-channel promotion occurs in this checkpoint.

## Verification and limitations

- Eleven desktop tests passed, including measured canonical panel geometry, SVG decoding/transparency, camera/hit transforms, accepted-action history and private-hand covering.
- Workspace formatting/all-target/all-feature Clippy passed. Initial missing named-font setup in synthetic contexts and two style lints were repaired. Compiler hard-link fallback warnings copied files; no remaining code lint failures.
- Native frame evidence: 098 canonical, 099 egui zoom 1.5, 100 compact, 101 rejected action. Intermediate 083–097 are preserved and individually labeled in the photo index. 087 exposed a physical-capture zoom-size bug and is superseded.
- Strict reference comparison is recorded separately in the photo archive. Pixel identity is false; scene contents differ as well as artwork. The metric is not a similarity percentage and cannot prove rules/interaction correctness.
- Actual mouse/keyboard interaction through the complete native UI and nonlocal runtimes remain unverified. Existing synthetic egui tests cover action/camera behavior. The user's ongoing computer permission does not change tool availability.

## Completion audit: not achieved

| Requirement | Current evidence | Status |
| --- | --- | --- |
| Identical reference dimensions | Native 2000 × 1135 captures, including scaled capture | Proven for those captures |
| Reference panel geometry and basic palette | Layout test, sampled colors and inspected same-size rendering | Concrete progress; individual pixels still differ |
| Identical terrain/shore/ship/piece artwork | Only official wool acquired; other art remains generated/procedural | Incomplete |
| Identical bank/cards/avatars/action/award icons | Positions recreated; several glyphs remain approximations | Incomplete |
| Identical typography, borders, gradients and states | Open Sans plus candidate numeral font; runtime checks | Incomplete/unverified |
| Identical supplied scene contents | Real M04 scenario has different map, pieces, counts and logs | Not proven; comparison includes these differences |
| All visible reference functions | M05/online capabilities remain unavailable, with honest disabled states | Not implemented by this M04 visual task |
| No singular visible difference | Strict comparison is false | Not achieved |

Keep the full goal active. Do not substitute this checkpoint or green code checks for exact visual completion. Next work needs the remaining exact artwork plus continued measured comparison; the denied full-pack download cannot be bypassed. M04 remains Verifying. No M05 start, beta/stable/main promotion, tag or release.
