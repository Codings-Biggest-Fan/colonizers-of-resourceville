# Native Rust Colonist Remake

An independently implemented Windows-first hex-board strategy game project.

**Current state:** planning structure established; no Cargo workspace or playable application yet. Milestone 01 is In progress. No release has shipped, no application screenshot exists, and no cloud resources have been created.

## Start here

- [Current roadmap](ROADMAP.md)
- [Accepted implementation plan](docs/implementation-plan.md)
- [Milestone 01 plan](milestones/milestone-01/plan.md) and [current progress](milestones/milestone-01/progress.md)
- [Agent and contributor rules, including Polish mode](AGENTS.md)
- [Development guide](docs/development.md) and [helper scripts](scripts/README.md)
- [Screenshot evidence](photos/README.md) and [milestone record conventions](milestones/README.md)

## Planned architecture

Rust rules/protocol/bots, native egui/eframe + wgpu desktop, and a separate Axum/Tokio server backed by a new Supabase Auth/Postgres project. Offline solo comes before online play. Hosts choose human/bot seats; the initial 2–4-seat support expands to 2–8.

The roadmap includes base rules, Cities & Knights, Seafarers, their supported combinations, Traders & Barbarians, Explorers & Pirates, public casual games, and ranked base queues. All are planned, not currently implemented.

## Check the planning structure

From PowerShell, including outside this directory:

```powershell
& 'C:\Users\asharma\Documents\colonist\scripts\milestone.ps1' -All
& 'C:\Users\asharma\Documents\colonist\scripts\bootstrap.ps1'
```

The bootstrap helper checks prerequisites without installing anything. Cargo build/run commands will be documented when the workspace is implemented; none are claimed to work yet.

No public product name or project license is selected. Use original code and assets; see [attribution](assets/ATTRIBUTION.md).
