# Asset attribution

The foundation preview's hexagonal emblem is original procedural egui drawing in crates/desktop/src/main.rs (2026-10-15). No external artwork or sound is included. The project has not selected a public license.

The desktop uses eframe/egui's bundled default fonts and native wgpu renderer. Cargo.lock pins third-party Rust dependencies. The packaging helper includes dependency license metadata and available license/notice files (including bundled-font notices) under third-party/. These dependencies retain their respective licenses; no license is inferred for project-authored code.

Future entries must record path, title, author, source, license, modifications, and required notices for original and third-party assets. Dependency licenses must also be preserved in release distribution where required.

photos/ contains development evidence and is not an asset source. Do not reuse extracted third-party artwork, logos, sound, or proprietary code.
