# Milestone 01 — Foundation

Target release: **v0.0.1**. Status: In progress. This is an initial plan; refine against actual code and dependencies before implementation.

[Roadmap](../../ROADMAP.md) · [Progress](progress.md) · [Reports](reports/) · [Screenshots](../../photos/milestone-01/index.md)

## Goal and dependencies

Establish a reproducible native Rust workspace and the permanent planning, evidence, and release structure.

No prior milestone is required. The latest user request is to establish structure/plans first; do not treat the rest of foundation as completed.

## Scope and implementation tasks

- [ ] Create the five-crate Cargo workspace: rules, protocol, bots, desktop, server; pin the toolchain and commit Cargo.lock once generated.
- [ ] Launch an original native egui/eframe window using wgpu; do not present future game controls as playable.
- [ ] Add Windows CI and script entry points for development, checks, tests, captures, packaging, and milestone validation.
- [ ] Establish all 20 initial plans, progress records, report folders, photo indexes, architecture documentation, and current roadmap.
- [ ] Verify Windows prerequisites without silently installing tools or provisioning services.

## Interfaces, state, persistence, and UI

Define crate responsibilities and dependency direction only. Gameplay configuration, protocol messages, and save schemas are finalized in milestone 02. No database migrations or cloud resources are required.

## Tests and acceptance checks

- [ ] Validate all required documentation paths, milestone folders, and local Markdown links.
- [ ] From a clean checkout, restore dependencies and pass formatting, Clippy, workspace tests, and release build.
- [ ] Launch the real native window; verify close, resize, and Windows scaling behavior.
- [ ] Exercise helper scripts from outside the repository and verify actionable failures for missing prerequisites.

## Screenshot evidence

Capture the real native window and record pixel dimensions, display scaling, build identity, and inspection result. Do not manufacture a window image before the application exists.

Record actual captures and metadata in the linked photo index. Explain nonvisual work and absent captures truthfully.

## Compatibility and migration

There are no existing games or saves. Keep the Windows target explicit and do not promise other desktop packages.

## Release gate and handoff

Clean checkout builds and launches; script entry points and documentation links validate; inspected native-window evidence is archived.

Update progress, append actual implementation reports, index inspected photos, update ROADMAP.md/CHANGELOG.md, and complete evidence review before marking Complete. Then summarize the release and stop for the next request. User-requested Polish mode pauses advancement.
