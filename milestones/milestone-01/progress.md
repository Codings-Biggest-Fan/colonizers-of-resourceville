# Milestone 01 progress — Foundation

Status: In progress  
Target release: v0.0.1  
Last updated: 2026-10-15

[Plan](plan.md) · [Reports](reports/) · [Photos](../../photos/milestone-01/index.md) · [Roadmap](../../ROADMAP.md)

## Landed work

- Project structure, all 20 initial plans/progress/photo indexes, roadmap, contributor rules including Polish mode, and planning documents.
- Read-only prerequisite checking and milestone structure/link validation helpers. Actual checks are recorded in the foundation report.
- Planning structure published to the public GitHub repository; see [publication report](reports/003-github-publication.md). No milestone release is implied.

## Implementation checklist

- [ ] Create the five-crate Cargo workspace: rules, protocol, bots, desktop, server; pin the toolchain and commit Cargo.lock once generated.
- [ ] Launch an original native egui/eframe window using wgpu; do not present future game controls as playable.
- [ ] Add Windows CI and script entry points for development, checks, tests, captures, packaging, and milestone validation.
- [ ] Establish all 20 initial plans, progress records, report folders, photo indexes, architecture documentation, and current roadmap.
- [ ] Verify Windows prerequisites without silently installing tools or provisioning services.

The documentation portion of the records task is established. The broad task stays unchecked until native scaffolding/CI/helpers and the remaining foundation requirements are delivered.

## Verification

- [ ] Validate all required documentation paths, milestone folders, and local Markdown links.
- [ ] From a clean checkout, restore dependencies and pass formatting, Clippy, workspace tests, and release build.
- [ ] Launch the real native window; verify close, resize, and Windows scaling behavior.
- [ ] Exercise helper scripts from outside the repository and verify actionable failures for missing prerequisites.

Unchecked items are pending, not failed or passed by inference. Structure-check results do not imply application build/runtime verification.

## Remaining work and dependencies

Finish the active milestone's actual Rust workspace, native window, CI, remaining helpers, release checks, and screenshot inspection. Rust/MSVC tools were not found in the initial targeted checks; verify prerequisites before building. No install has been performed.

## Blockers

No blocker to the requested documentation structure. Toolchain readiness remains unverified for subsequent native implementation.

## Polish checklist

No active Polish mode request or reported flaw checklist. See [Polish mode](../../AGENTS.md#polish-mode).

## Evidence and next step

See [001 — planning structure](reports/001-planning-structure.md) and [002 — actual validation results](reports/002-structure-validation.md). The structure check passed for all 20 milestone sets and 443 local links; both helper scripts parsed successfully. The pending-completion gate rejected M01 as expected. Prerequisite checking reported missing/unverified Rust and MSVC tools without installing anything. Next: continue foundation with prerequisite verification and Cargo/native scaffolding when requested.
