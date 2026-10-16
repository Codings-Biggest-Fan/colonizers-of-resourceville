# 005 — Polish: softer terrain and rounded houses

Date: 2026-10-16 (America/Los_Angeles). Scoped visual correction; M04 remains Verifying.

## User flaws and cause

The user reported strange-looking tile visuals and houses that looked too realistic/sharp. The previous atlas repeated oversized, strongly shaded trees, grain bundles, mesas and pyramid-like peaks under hex clipping. The house contour had hard polygon corners and architectural cutouts; the old city contour also contained a concave roof valley passed to a convex fill.

## Correction

Created original terrain-v4.png by editing the existing original atlas through the built-in image-generation tool. Smaller, flatter cartoon terrain motifs, softer ridges, planted grain rows and calmer centers improve the actual hex presentation. Preserved six terrain identities, token positions, stable topology, ports, sea and camera controls. The historical textures remain in assets/.

Rounded every house corner and roof peak with sampled quadratic curves; simplified the door and removed the extra city window. Cities pair two small convex rounded house shapes, retaining a distinct silhouette without a concave polygon fill. Preserved cream/dark separation, player colors and owner numbers. No gameplay, protocol, configuration, interaction or persistence behavior changed.

## Validation and evidence

- User explicitly permitted game-rendered-frame-only captures for this check. No native desktop/window-helper capture was used.
- Inspected [021 normal](../../../photos/milestone-04/021-soft-terrain-houses.png), [022 compact](../../../photos/milestone-04/022-soft-compact.png), [023 all-terrain contrast fixture](../../../photos/milestone-04/023-soft-contrast.png), and [024 scaled engine replay](../../../photos/milestone-04/024-soft-scaled-production.png). All are actual eframe/wgpu output. The contrast fixture is not a legal game.
- Before evidence is preserved in M04 010/014/020. Normal/compact numbers, ports and piece silhouettes remain readable; the scaled test retains its existing movable overlay limitation. Native scale 1.5; egui zoom 1 except 024 at 1.5. No claim of changing Windows DPI.
- scripts/check.ps1 passed formatting and all-target/all-feature workspace Clippy. Cached/offline dependencies were used after interrupting the initial package-cache wait; no tools installed.
- All five desktop regression tests passed with all features, including transformed targets, camera input, seed recovery and the native action bridge.
- README main image/grid, photo metadata, asset attribution, DESIGN, progress, roadmap and changelog updated. No rules tests added for this presentation-only correction.

## Delivery and limitations

Continue on milestone-04/base-turn-engine and nightly. Beta remains the approved PR #11 baseline at f5548fd; stable/main unchanged. No polish approval, stable PR, release, M05 work or indefinite Polish mode is implied. Cross-platform native rendering, manual input walkthrough and remaining M04 acceptance work remain unverified by these visual checks.
