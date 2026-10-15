# Milestone 01 progress — Foundation

Status: Planned  
Target release: v0.0.1  
Last updated: 2026-10-15

[Plan](plan.md) · [Reports](reports/) · [Photos](../../photos/milestone-01/index.md) · [Roadmap](../../ROADMAP.md)

## Landed work

No native implementation has started. Repository/documentation setup and its historical reports now belong to [M00](../milestone-00/progress.md).

## Implementation checklist

- [ ] Create the five-crate Cargo workspace: rules, protocol, bots, desktop, server; pin the toolchain and commit Cargo.lock once generated.
- [ ] Launch an original native egui/eframe window using wgpu; do not present future game controls as playable.
- [ ] Add Windows CI and script entry points for development, checks, tests, captures, packaging, and milestone validation.
- [ ] Reuse and refine the records established in M00 against the actual native implementation.
- [ ] Verify Windows prerequisites without silently installing tools or provisioning services.

## Verification

- [ ] Validate all required documentation paths, milestone folders, and local Markdown links.
- [ ] From a clean checkout, restore dependencies and pass formatting, Clippy, workspace tests, and release build.
- [ ] Launch the real native window; verify close, resize, and Windows scaling behavior.
- [ ] Exercise helper scripts from outside the repository and verify actionable failures for missing prerequisites.

Unchecked items are pending, not failed or passed by inference. Structure-check results do not imply application build/runtime verification.

## Remaining work and dependencies

Finish the active milestone's actual Rust workspace, native window, CI, remaining helpers, release checks, and screenshot inspection. Rust/MSVC tools were not found in the initial targeted checks; verify prerequisites before building. No install has been performed.

## Blockers

Await M00 testing/approval and a request to begin M01. Toolchain readiness remains unverified.

## Polish checklist

No active Polish mode request or reported flaw checklist. See [Polish mode](../../AGENTS.md#polish-mode).

## Evidence and next step

No native builds, screenshots, or release artifacts yet. See [M00 historical setup reports](../milestone-00/reports/). Begin only after M00 approval and the next user request.
