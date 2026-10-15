# 004 — Milestone 0, channels, approval, and wiki

Date: 2026-10-15 (America/Los_Angeles)

## Scope and history

The user separated Initial setup as M00, retained the 20 implementation milestones, requested a branch/PR per milestone with approval after testing, defined nightly/beta/stable channels, and requested wiki/build/player instructions. Original setup reports 001–003 were moved here without rewriting historical M01 wording. Native Rust foundation remains M01.

## Changes

- Added M00 plan/progress/photo records; reset unstarted native M01 to Planned.
- Updated AGENTS.md, all 20 implementation plans, roadmap, and developer guidance with the user approval gate and per-milestone GitHub Release policy.
- Defined milestone branches and nightly/latest-work, beta/unapproved-PR, stable/last-approved-merged meanings; initial stable contains only the pre-workflow baseline.
- Added wiki generation, README links, Building guidance, and the Game Instructions source folder and grouped wiki navigation.
- Added a PR template, documentation CI, and text/binary attributes.

## Executed validation

- Source validator passed: 21 record sets, 116 Markdown files, 517 local links before this report.
- All three PowerShell helper scripts parsed without syntax errors.
- Wiki exporter generated 119 Markdown files; 583 wiki page links resolved to generated pages.
- Rejected export outside ignored .local and rejected approved-mode export from the milestone branch; both returned exit 1 as expected.
- M00 RequireComplete returned exit 1 as expected because user approval is pending.
- git diff --check passed after adding Markdown/text line-ending attributes.

The exporter initially encountered PowerShell's reserved HOME variable; the variable was renamed before successful generation. No generated pages from that failed attempt were published.

## Publication and approval status

The wiki was initialized through GitHub's ordinary editor. Full preview publication, channel/PR checks, and CI results will be recorded after publishing. No native screenshot, game build, merge, tag, or release has occurred. M00 is Verifying, not approved.

## Next steps

Push the milestone/nightly/beta refs, open the M00 PR against stable, verify CI, publish the source-pinned preview wiki, inspect its Home/Building/Game Instructions pages, and record the exact PR/head/evidence. Then wait for the user's testing and explicit approval before merging or publishing source-only v0.0.0.
