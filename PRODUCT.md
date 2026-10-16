# Product context

The Colonizers of Resourceville is a Windows-first native Rust hex-board resource strategy game, with original code/art/branding. Native egui/eframe/wgpu desktop; separate Rust server. No browser UI. See AGENTS.md and ROADMAP.md for authoritative scope and channel policy.

M03 serves local board inspection, not a playable match. The user wants an inviting cartoon board whose terrain and harbor graphics speak without on-board labels, surrounded by a simple sea. The board has no permanent chrome. Wheel zoom and right-drag pan control the camera; a right-click context menu keeps seed/setup controls available. Friends, bots, turns, saves and rooms remain later milestones. Do not fabricate hands, resource counts, scores, dice, players or turn controls to fill a game HUD.

## Platform

Native desktop Rust, Windows-first; CI builds Windows/Linux/macOS x86-64 and ARM64. No installed web runtime or webview. Native keyboard focus, resizing and actual game-window captures are required evidence. Ask before UI access; Escape ends that session.

M04 adds a native synthetic-seat turn-engine test through F4/context menu. It performs actual setup, production and building transitions with supplied dice, public hand counts and hidden hands. Complete offline games, bots, saves, friends and rooms remain later work. Keep this harness truthful and separate from a supported shared-device game mode.
