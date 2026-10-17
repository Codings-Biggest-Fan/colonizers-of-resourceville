# Product context

The Colonizers of Resourceville is a Windows-first native Rust hex-board resource strategy game, with original code/art/branding. Native egui/eframe/wgpu desktop; separate Rust server. No browser UI. See AGENTS.md and ROADMAP.md for authoritative scope and channel policy.

M03 serves local board inspection, not a playable match. The user wants an inviting cartoon board whose terrain and harbor graphics speak without on-board labels, surrounded by a simple sea. The board has no permanent chrome. Wheel zoom and right-drag pan control the camera; a right-click context menu keeps seed/setup controls available. Friends, bots, turns, saves and rooms remain later milestones. Do not fabricate hands, resource counts, scores, dice, players or turn controls to fill a game HUD.

## Platform

Native desktop Rust, Windows-first; CI builds Windows/Linux/macOS x86-64 and ARM64. No installed web runtime or webview. Native keyboard focus, resizing and actual game-window captures are required evidence. Ask before UI access; Escape ends that session.

M04 adds a native synthetic-seat turn-engine test through F4/context menu. It performs actual setup, production and building transitions with supplied dice, public hand counts and hidden hands. Complete offline games, bots, saves, friends and rooms remain later work. Keep this harness truthful and separate from a supported shared-device game mode.

Current M04 HUD direction (2026-10-16 user request): graphic-first right sidebar and bottom action dock, replacing historical chrome-free/floating-window guidance. Show only functional implemented actions; development-card/trade controls are visibly disabled future affordances. No private hands, fake scores or chat.

Current light HUD direction (2026-10-16): Catppuccin Latte supersedes the earlier dark rail. Use horizontal player rows with public card counts, a short bottom action dock and accepted public activity at normal sizes. Compact/scaled views place dice below the heading and hide activity. Keep original artwork, hidden hands private and unimplemented card/trade controls disabled. Appearance settings/persistence are not implemented by this correction.

Current screenshot-led M04 direction (2026-10-16) supersedes earlier layout notes: full-height right activity and unavailable chat region, five-resource bank plus unavailable development deck, compact opponent rows and active seat last; bottom covered hand/build dock with dice above actions; top-left native back/guide/audio/options/support tools. Audio/support/card/trade remain disabled until implemented. Generated terrain v5 uses larger classic resource symbols; desert has no resource/card. Intentional active-seat hand reveal automatically covers on focus loss, new test or accepted end-turn. Keep captures covered and branding original. Report 009 records provenance and limits.

Current user acceptance is exact visual identity with the supplied reference (2026-10-17). Approximation and green code checks do not satisfy it. Preserve native Rust and original product identity, actual M04 behavior/privacy and honest unavailable features; record unresolved fidelity and asset-download gates. Report 010 tracks the open goal.
