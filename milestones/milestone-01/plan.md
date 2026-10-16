# Milestone 01 — Foundation

Target release: **v0.0.1**. Status: In progress. Refined against the documentation-only checkout on 2026-10-15.

[Roadmap](../../ROADMAP.md) · [Progress](progress.md) · [Reports](reports/) · [Screenshots](../../photos/milestone-01/index.md)

## Goal and dependencies

Establish a reproducible native Rust workspace and the permanent planning, evidence, and release structure.

The user explicitly requested M01 on 2026-10-15, resuming roadmap implementation. M00 polish is merged in stable at 49bd2c0; no release is inferred from that merge. M01 implements the native foundation.

## Scope and implementation tasks

- [ ] Create the five-crate Cargo workspace: rules, protocol, bots, desktop, server; pin the toolchain and commit Cargo.lock once generated.
- [ ] Launch an original native egui/eframe window using wgpu; do not present future game controls as playable.
- [ ] Add GitHub build/package CI for Windows, Linux, macOS on x86-64 and ARM64; local native testing remains Windows-first.
- [ ] Add script entry points for development, checks, tests, captures, packaging, and milestone validation.
- [ ] Reuse the M00 records and refine/update them against the actual native implementation.
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

There are no existing games or saves. GitHub builds cover Windows/Linux/macOS x86-64 and ARM64 by user request; nonlocal runtime support is unverified until tested.

## Release gate and handoff

Clean checkout builds and launches; script entry points and documentation links validate; inspected native-window evidence is archived.

Update progress, append actual implementation reports, index inspected photos, update ROADMAP.md/CHANGELOG.md, and complete evidence review before marking Complete. Then summarize the release and stop for the next request. User-requested Polish mode pauses advancement.
## Branch, user testing, and GitHub release

Use milestone-01/foundation and an implementation PR against beta. nightly follows pushed work; beta stays at its user-approved baseline. Polish requires a separate approved PR against stable. Leave the milestone Verifying and PR open for the user's testing/explicit approval. Do not merge, advance stable, publish the intended GitHub Release, or begin the next milestone before approval. After approval and passing gates, release the merged stable commit and link PR/release/wiki/evidence in the records. See [delivery workflow](../../docs/github-workflow.md).

