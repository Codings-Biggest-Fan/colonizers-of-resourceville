# Development

**Current phase:** planning and directory structure. Cargo workspace, native UI, CI, and most runtime helpers are still pending in [M01](../milestones/milestone-01/progress.md).

## Prerequisites

Windows-first Rust/MSVC development requires Rust tools and the compatible Microsoft C++ linker/build prerequisites. Check using scripts/bootstrap.ps1. The helper does not install anything; missing tools return a nonzero status with guidance. Pin exact versions when scaffolding.

No build, test, application, or Docker command is advertised as runnable until its implementation exists.

## Available structure checks

```powershell
& 'C:\Users\asharma\Documents\colonist\scripts\milestone.ps1' -All
& 'C:\Users\asharma\Documents\colonist\scripts\milestone.ps1' -Number 1
```

The checker validates record structure/local Markdown links and optional Complete status. It cannot prove test outcomes, screenshot contents, or runtime correctness.

## Implementation workflow

Read AGENTS.md, ROADMAP.md, and the active plan/progress. Refine the active plan against actual code. Keep changes scoped, preserve unrelated edits, run meaningful checks, inspect native UI, and append a numbered report.

Future helper contracts are listed in [scripts/README.md](../scripts/README.md). Screenshots use [photos conventions](../photos/README.md). Report failures honestly.

## Polish workflow

Explicit Polish mode pauses milestone advancement. Maintain the user's flaw checklist, fix underlying causes, inspect before/after output when visual, and append a polish report. Resume roadmap implementation only when requested. [Full rules](../AGENTS.md#polish-mode).
