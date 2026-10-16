# Native board design

M03 direction follows the user's 2026-10-15 corrections: expressive but simplified cartoon terrain, graphic ports without on-board labels, a simple sea surrounding the whole island, wheel zoom and right-drag pan without permanent chrome. Product identity stays The Colonizers of Resourceville.

## Surface and hierarchy

Operate/Experience: the island dominates. No top/bottom bars, Board label or zoom slider. The sea fills the viewport. Mouse wheel zooms, right-drag pans and left-click inspects. A right-click context menu keeps seed generation, sample-piece visibility and Fit accessible without permanent chrome. The native window retains the product title. No fabricated game-state controls, marketing blocks or decorative badges.

## Semantic tokens

Catppuccin Mocha background #1e1e2e, surface #313244, input #181825, text #cdd6f4, secondary #bac2de, blue focus/selection #89b4fa, error #f38ba8. Original illustration colors are content colors, distinct from interface theme. Sea uses plain light blue #7ec7df; shore separates sea and island. Terrain silhouettes and hover/selection names supplement color. Appearance settings and all flavor/accent preferences are not exposed in M03.

[Official palette guidance](https://github.com/catppuccin/catppuccin/blob/main/docs/style-guide.md) informs UI semantics; readability takes priority. Native controls use shared text/surface/focus tokens. Number tokens use cream with dark ink and probability dots; red 6/8 also have explicit numbers/dots.

## Artwork and states

Original generated atlas textures are embedded in the binary and mapped into actual topology hexes; no generated mockup is used as runtime evidence. Harbors use cargo boats; ratios are disclosed by hover/selection. Asset provenance is in assets/ATTRIBUTION.md. Zoom is uniform; no stretching by viewport aspect ratio. Visible focus border and keyboard target cycling complement pointer inspection. Invalid seeds preserve typed input and the previous board. No animation is needed for frequent camera or inspection actions.

Sample settlements use original miniature geometry with a front gable, receding side wall, two shaded roof planes, recessed door and ground shadow. They are preview pieces, not placed by a playable game engine.

## Verification

Inspect default, compact, wide, scaling stress and zoom/pan captures. Earlier flat/painterly images are before evidence, not the final visual standard. Capture only the game window. Record actual scaling; egui zoom is not proof of switching Windows DPI.

## M04 native expansion

The board remains the dominant surface. A closable, draggable native Turn engine test window owns actual phase/actions and public resource counts; no permanent top/bottom bars. F4/context menu opens it. Selected-target actions validate through the rules engine; errors stay in the panel and input is preserved. Explicit supplied dice identify this as a test harness. Private hands never render. Flat side-view port ships use geometric cargo icons; generic ports use a drawn question mark. Roads/houses use cream outer separation, dark outlines and player color. Ownership is also available in text inspection and numbered settlements. Appearance preferences remain future scope.
