# Development

**Current phase:** M00 initial setup. Documentation CI is provided; Cargo workspace, native UI, native-build CI, and runtime helpers remain pending in [M01](../milestones/milestone-01/progress.md).

## Prerequisites

Windows-first Rust/MSVC development requires Rust tools and the compatible Microsoft C++ linker/build prerequisites. Check using scripts/bootstrap.ps1. The helper does not install anything; missing tools return a nonzero status with guidance. Pin exact versions when scaffolding.

No build, test, application, or Docker command is advertised as runnable until its implementation exists.

## Available structure checks

```powershell
& 'C:\path\to\hearthline\scripts\milestone.ps1' -All
& 'C:\path\to\hearthline\scripts\milestone.ps1' -Number 1
```

The checker validates record structure/local Markdown links and optional Complete status. It cannot prove test outcomes, screenshot contents, or runtime correctness.

## Implementation workflow

Read AGENTS.md, ROADMAP.md, and the active plan/progress. Refine the active plan against actual code. Keep changes scoped, preserve unrelated edits, run meaningful checks, inspect native UI, and append a numbered report.

Future helper contracts are listed in [scripts/README.md](../scripts/README.md). Screenshots use [photos conventions](../photos/README.md). Report failures honestly.

## Polish workflow

Explicit Polish mode pauses milestone advancement. Maintain the user's flaw checklist, fix underlying causes, inspect before/after output when visual, and append a polish report. Resume roadmap implementation only when requested. [Full rules](../AGENTS.md#polish-mode).

## Build, review, and player documentation

Use [Building](building.md) for channel selection and guarded future Cargo commands. [GitHub workflow](github-workflow.md) requires one milestone branch/PR, nightly/beta alignment, and user approval before stable/release updates. [Game Instructions](game-instructions/README.md) is the player-guide source folder published in the wiki. Generate preview docs with scripts/wiki.ps1; publication is separate and explicit.
