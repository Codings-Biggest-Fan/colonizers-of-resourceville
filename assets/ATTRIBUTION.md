# Asset attribution

No competitor artwork, extracted assets, sounds, logos or copied interface are used. The project has not selected a public license.

| Path | Title / authorship | Source and date | License / modifications / notices |
| --- | --- | --- | --- |
| art/terrain-v1.png | Original painterly terrain atlas; OpenAI image generation directed for this project | Generated 2026-10-15, original output exec-6237d412-afe2-4c20-9c6c-a9f3875b56f0 | Project artwork; no public license selected. Unmodified PNG. Historical iteration, not embedded in final app. |
| art/terrain-v2.png | Original cartoon terrain iteration; OpenAI image generation | Generated edit of v1, exec-bd567a4b-c744-404f-b948-b7ba54423f19, 2026-10-15 | Project artwork; no public license selected. Unmodified PNG. Historical iteration. |
| art/terrain-v3.png | Simplified original cartoon terrain atlas; OpenAI image generation | Generated edit of v2 following direct user simplification feedback, exec-99cbdb8a-88c5-4ad5-880d-de863af2833a, 2026-10-15 | Project artwork; no public license selected. Unmodified PNG. Historical M03/M04 iteration; superseded by terrain-v4.png. |
| art/terrain-v4.png | Soft flat cartoon terrain atlas | Generated edit of v3 following user feedback, 2026-10-16, exec-ede82a80-f50f-4545-8cff-dc63773c2a83 | Original project artwork; no public license selected. Unmodified generated PNG, embedded in current desktop build. |
| art/ports-v1.png | Original resource cargo-boat atlas; OpenAI image generation directed for this project | Generated 2026-10-15, exec-13dbd110-c413-427e-aa89-e8fc358210fa | Project artwork; no public license selected. Unmodified transparent RGBA PNG. Historical M03 artwork; no longer embedded at M04. |
| ../crates/desktop/src/board_view.rs | Original board projection, sea, shoreline, number tokens, sample piece geometry | Authored in this repository, 2026-10-15 | Project code; no public license selected. No external art. |
| ../crates/desktop/src/main.rs | Original procedural hex window icon | Authored in this repository, 2026-10-15 | Project code; no public license selected. |

Generated art is not a runtime screenshot. Development evidence stays in photos/. Historical boat sprites retain their generated alpha. No image generation credentials or model service is used at game runtime; shipping textures are embedded for offline use.

The desktop uses eframe/egui's bundled default fonts and native wgpu renderer. Cargo.lock pins third-party dependencies. Packaging includes dependency license metadata and available license/notice files (including font notices) under third-party/. Dependencies retain their respective licenses; no license is inferred for project-authored code/art.

M04 flat ships, cargo icons, question marks, outlined roads and houses are original procedural geometry in crates/desktop/src/pieces.rs; no external artwork. The terrain atlas remains embedded.



M04 visual correction (2026-10-16): rounded settlement silhouettes and paired toy-house cities are original procedural geometry in pieces.rs. Terrain v4 reduces oversized shaded objects, leaves quieter number-token centers, and preserves six distinct terrain families. Historical assets are retained as evidence of prior iterations.
