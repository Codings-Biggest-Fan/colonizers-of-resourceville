# Milestone 00 progress — Initial setup

Status: Verifying  
Target release: v0.0.0 (setup-only prerelease, not published)  
Last updated: 2026-10-15

[Plan](plan.md) · [Reports](reports/) · [Photos](../../photos/milestone-00/index.md) · [Roadmap](../../ROADMAP.md)

## Landed work

- Repository structure and 20 implementation plans/progress/photo records established.
- Initial source publication preserved; historical setup reports moved here without rewriting their contents.
- Native Rust product rules and Polish mode documented.
- Branch/PR/approval/release policy added; milestone implementation remains isolated from stable.
- Wiki initialized; generated public preview, Building, and Game Instructions pages prepared.
- Prerequisite, structural validation, wiki-generation helpers and documentation CI added.

## Verification and review

- [x] Source records/local Markdown link targets pass.
- [x] PowerShell helpers parse and wiki generation passes.
- [x] Invalid output/approval attempts fail safely.
- [x] Public wiki preview and channel/PR identity verified.
- [ ] Documentation CI passes.
- [ ] User tests and explicitly approves the current PR revision.
- [ ] Approved PR merged into stable; setup v0.0.0 prerelease published.

Actual executed results and final PR links are recorded in [reports](reports/). Unchecked verification is not claimed complete.

## Remaining work and blockers

Finish review publication/verification, then wait for user testing and approval. No Rust application, native screenshot, game server, database, or cloud project is required for M00. No release exists yet.

## Polish checklist

Polish mode is active for the user's branding correction. Corrections stay in the same M00 PR; do not advance to M01 automatically.

- [x] Adopt The Colonizers of Resourceville and original expansion labels; remove legacy product branding from current source text.
- [x] Rename the repository and update source/wiki remotes, PR identity, documentation links, and contributor rules.
- [x] Republish and visually verify the new preview/wiki; archive the after-image and final report.
- [x] Expand the public game description and README with roadmap, linked documentation images, and honest build/setup instructions; validate and publish through the same PR.

[Review handoff](reports/007-review-handoff.md) · [Open PR #1](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/pull/1) · [Prepared release notes](release-notes.md). Hosted CI has no run yet and is not claimed passed. Polish mode remains active; await user testing/approval before merging, releasing, or advancing.
