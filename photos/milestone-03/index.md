# Milestone 03 photos — Board and preview

[Plan](../../milestones/milestone-03/plan.md) · [Progress](../../milestones/milestone-03/progress.md) · [Reports](../../milestones/milestone-03/reports/)

All images are real eframe/wgpu game frames captured on 2026-10-15 with permission. No desktop or unrelated window was captured. Source: M03 working tree based on approved stable 861b8c6; capture files precede the implementation commit and are not claimed to be from a clean release. Native pixels/point 1.5 was reported by eframe, consistent with 150% display scaling; Windows settings were not changed. Camera zoom is 1× in these captures. No gameplay/private identities are present.

## Inspected capture register

| Image | Window logical points / image pixels | Effective pixels/point | Scenario and inspection |
| --- | --- | --- | --- |
| [001](001-default-board.png) | 1000×680 / 1500×1020 | 1.5 | Initial flat diagram; labels/glyphs lacked requested charm. Before evidence only. |
| [003](003-compact-board.png) | 480×420 / 720×630 | 1.5 | Initial compact view; terrain labels crowded tokens. Superseded. |
| [005](005-scaling-stress.png) | 800×650 / 1200×975 | 2.25 (egui zoom 1.5) | Label crowding confirmed at scale. Superseded. |
| [006](006-illustrated-sea-hud.png) | 1000×680 / 1500×1020 | 1.5 | Painterly terrain and full sea/HUD; user requested simpler cartoon art and ports without labels. Superseded. |
| [007](007-compact-sea-hud.png) | 480×420 / 720×630 | 1.5 | Earlier compact artwork/HUD; superseded. |
| [010](010-cartoon-graphic-ports.png) | 1000×680 / 1500×1020 | 1.5 | Simplified cartoon terrain and graphic boats. User subsequently removed bars and dark sea. |
| [011](011-cartoon-compact.png) | 480×420 / 720×630 | 1.5 | Boats/terrain recognizable without labels; no clipping. Superseded chrome. |
| [012](012-cartoon-wide.png) | 1400×720 / 2100×1080 | 1.5 | Regular undistorted hexes and consistent artwork; superseded chrome. |
| [013](013-cartoon-scaling.png) | 800×650 / 1200×975 | 2.25 | Scaling stress, full island/boats clear. Superseded chrome. |
| [014](014-open-sea-3d-settlement.png) | 1000×680 / 1500×1020 | 1.5 | Light sea, no bars, shaded miniature settlement. Floating Board/slider later removed by user request. |
| [015](015-open-sea-compact.png) | 480×420 / 720×630 | 1.5 | Compact full island and settlement; superseded floating controls. |
| [016](016-open-sea-scaling.png) | 800×650 / 1200×975 | 2.25 | Light sea/roof geometry inspected; slider readable but subsequently removed. |
| [017](017-wheel-right-pan.png) | 1000×680 / 1500×1020 | 1.5 | Current default: sea fills frame, no bars/Board label/slider; graphic harbors and shaded settlement, no visible overlap or distortion. |
| [018](018-wheel-compact.png) | 480×420 / 720×630 | 1.5 | Current compact: entire island/ports visible, no persistent UI clutter, regular geometry. |

Intermediate captures 002, 004, 008 and 009 are retained in this directory but were not individually inspected; no acceptance claim is based on them. Their capture logs reported native pixels/point 1.5; 009 used egui zoom 1.5. They remain archived iterations rather than current screenshots.

## Remaining evidence

Final zoom/pan/context-menu native interaction checks, current wide/scaling capture and complete milestone release gate remain pending. The input regression uses egui events in tests and is distinct from a human interaction check. No non-Windows runtime behavior is inferred.
