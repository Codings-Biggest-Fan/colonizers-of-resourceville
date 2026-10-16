# Milestone 01 — Native foundation evidence

Date: 2026-10-15 (America/Los_Angeles). These are actual wgpu rendered frames, not generated mockups. Only the application's own framebuffer was captured. Native title bars are outside these frame images.

| Image | Build/revision | Dimensions | Scaling | Scenario and inspection |
| --- | --- | --- | --- | --- |
| [001 native foundation](001-native-foundation.png) | c5b4a95-equivalent capture code, before theme correction | 1500 × 1020 px; 1000 × 680 logical client | Windows 150%, egui zoom 1 | Inspected: low text contrast; failed visual check, retained as before evidence. |
| [002 compact before](002-native-compact.png) | Same as 001 | 720 × 630 px; 480 × 420 logical client | Windows 150%, zoom 1 | Inspected: close control fell below visible scroll area. |
| [003 zoom before](003-native-zoom.png) | Same as 001 | 1080 × 900 px; 720 × 600 logical client | Windows 150%, zoom 1.5; effective 225% | Before evidence; superseded by 008. |
| [004 contrast and footer](004-native-foundation-contrast.png) | UI changes subsequently committed in 328c7ab; captured with uncommitted edits over c5b4a95 | 1500 × 1020 px | Windows 150%, zoom 1 | Inspected: readable title, emblem, honest scope, close control fixed in footer. |
| [005 compact footer](005-native-compact-footer.png) | Intermediate edits over c5b4a95 | 720 × 630 px | Windows 150%, zoom 1 | Inspected: footer visible; oversized decoration still clipped in scroll area, corrected in 007. |
| [006 zoom footer](006-native-zoom-footer.png) | Intermediate edits over c5b4a95 | 1080 × 900 px | Windows 150%, zoom 1.5 | Inspected: footer visible; decoration overflow prompted compact layout. |
| [007 compact responsive](007-native-compact-responsive.png) | UI subsequently committed in 328c7ab | 720 × 630 px | Windows 150%, zoom 1 | Inspected: full title, preview scope, version, and Close preview fit without overlap; decorative emblem omitted in compact layout. |
| [008 zoom responsive](008-native-zoom-responsive.png) | UI subsequently committed in 328c7ab | 1080 × 900 px | Windows 150%, zoom 1.5 | Inspected: full content and footer fit with wrapping; no clipped decoration. |

Capture logs reported native pixels per point 1.5. The additional zoom check changes egui zoom, not Windows display scaling. Other OS display scaling values have not been tested. The native window was also inspected with accessibility: Close preview and the title-bar Close are exposed as buttons. No gameplay, multiplayer, or cloud evidence is claimed.

[Progress](../../milestones/milestone-01/progress.md) · [Reports](../../milestones/milestone-01/reports/)

## Clean native release build — final window check

Captured and inspected on 2026-10-15 from the clean Windows x86-64 optimized binary built at 066b4401fec1d17ad0f19e1537a3fcd9804167f0. The user explicitly authorized this computer access. The stable captures below contain only the selected game window. Transient resize/menu captures returned unreliable bounds and were not retained.

| Image | Archived dimensions | Scenario and inspection |
| --- | --- | --- |
| [009 clean native window](009-clean-native-window.jpg) | 1003 × 711 px JPEG | Inspected original procedural title-bar icon, exact product title, readable content, emblem, and visible footer. |
| [010 clean native maximized](010-clean-native-maximized.jpg) | 1707 × 1019 px JPEG | Inspected settled maximized native window: layout fills available space, emblem remains proportional, content/footer do not overlap. |

Computer Use reports the above logical screenshot sizes; physical window dimensions are not independently recorded for these JPEGs. The Windows session was previously measured at DPI 144 (150%); no OS scaling change was made. This is distinct from 001–008, which are full-resolution rendered-frame captures with reported native pixels/point. Keyboard Tab focused Close preview, Enter activated it, and a subsequent window inventory confirmed the game closed. Native restore was observed returning the title-bar control to Maximize after recovery from a transient minimized-window report.
