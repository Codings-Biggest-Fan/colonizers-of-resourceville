# 007 — Initial setup review handoff and naming verification

Date: 2026-10-15 (America/Los_Angeles)

## Current deliverable

**The Colonizers of Resourceville**, with public repository [colonizers-of-resourceville](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville). M00 implementation is ready for user review through [PR #1](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/pull/1), targeting stable. It remains Verifying pending explicit user testing/approval; M01 has not started.

## Actual verification

- Source validation passed after the selected-title changes: 21 milestone sets, 121 Markdown files, 524 local links before this handoff report.
- All three PowerShell helper scripts parsed; wiki export and invalid-output/non-stable-approved-mode checks passed.
- Current Markdown/PowerShell/workflow source scan found no reference-product or previous working-product names after replacement.
- The live wiki Home, Building, and Game Instructions pages were opened and inspected with the selected title, preview banner, source commit, and PR link.
- Six actual browser captures are indexed under [M00 photos](../../../photos/milestone-00/index.md). Current-title captures 004–006 show bb321a4; earlier captures are marked historical. Actual JPEG signatures/dimensions were checked: 503×846. OS display scaling was not measured.
- GitHub API verified public visibility, default stable, open/unmerged PR #1, and matching milestone/nightly/beta heads at bb321a4 before this final documentation commit. Stable remained the original d2ce1be baseline.
- GitHub Releases returned an empty list: no tag/release was created. No unapproved merge occurred.
- GitHub Actions permissions are enabled, but the API returned zero hosted workflow runs. The documentation workflow definition is present; hosted CI is **unverified**, not passed. Local checks are recorded separately.

The final documentation/evidence commit will be pushed atomically to the same three review refs, followed by clean-commit wiki regeneration. That changes records/evidence only, not the UI content captured above. Final ref identity and local checks are rechecked during publication.

## User testing

1. Fetch and check out beta, then compare HEAD to PR #1.
2. Run `pwsh -NoProfile -File ./scripts/milestone.ps1 -All`.
3. Run `pwsh -NoProfile -File ./scripts/wiki.ps1`; inspect ignored .local/wiki-export.
4. Read README wiki links, Building, Game Instructions, roadmap, channel definitions, and AGENTS.md including branding/Polish mode.
5. Report flaws or explicitly approve the tested revision. No native game is expected in setup-only M00.

## Boundaries and next steps

Polish mode remains active. Await the user's testing/approval; keep the PR open and stable unchanged. Hosted CI has no result and is not a claimed passing gate. After approval, resolve required gates, merge the approved head, republish approved wiki documentation, and publish prepared source/docs-only v0.0.0 release notes. Do not begin M01 automatically or install Rust/cloud services in this scope.
