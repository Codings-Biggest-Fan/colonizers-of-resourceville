# The Colonizers of Resourceville

An independently implemented Windows-first hex-board strategy game project.

[Public GitHub repository](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville) · [Documentation wiki](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/wiki) · [Building](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/wiki/Building) · [Game Instructions](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/wiki/Game-Instructions)

**Current state:** planning structure established; no Cargo workspace or playable application yet. Milestone 00 — Initial setup is ready for testing on its PR; Milestone 01 native foundation is planned. No release has shipped, no application screenshot exists, and no cloud resources have been created.

## Start here

- [Current roadmap](ROADMAP.md)
- [Accepted implementation plan](docs/implementation-plan.md)
- [Milestone 00 setup plan](milestones/milestone-00/plan.md) and [current progress](milestones/milestone-00/progress.md)
- [Branches, approval, and releases](docs/github-workflow.md) — nightly is latest work, beta is the unapproved PR, stable is the last approved/merged PR
- [Local Building guide](docs/building.md) and [Game Instructions folder](docs/game-instructions/README.md)
- [Agent and contributor rules, including Polish mode](AGENTS.md)
- [Development guide](docs/development.md) and [helper scripts](scripts/README.md)
- [Screenshot evidence](photos/README.md) and [milestone record conventions](milestones/README.md)

## Planned architecture

Rust rules/protocol/bots, native egui/eframe + wgpu desktop, and a separate Axum/Tokio server backed by a new Supabase Auth/Postgres project. Offline solo comes before online play. Hosts choose human/bot seats; the initial 2–4-seat support expands to 2–8.

The roadmap includes base rules, Citadels & Sentinels, Open Seas, their supported combinations, Caravans & Raiders, Voyagers & Corsairs, public casual games, and ranked base queues. All are planned, not currently implemented.

## Check the planning structure

From PowerShell, including outside this directory:

```powershell
& 'C:\path\to\colonizers-of-resourceville\scripts\milestone.ps1' -All
& 'C:\path\to\colonizers-of-resourceville\scripts\bootstrap.ps1'
```

The bootstrap helper checks prerequisites without installing anything. Cargo build/run commands will be documented when the workspace is implemented; none are claimed to work yet.

The working product name is **The Colonizers of Resourceville**; no project license is selected. Use original code and assets; see [branding](docs/branding.md) and [attribution](assets/ATTRIBUTION.md).
