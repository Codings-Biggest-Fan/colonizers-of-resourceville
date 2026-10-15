# Milestone 00 — Initial setup

Target release: **v0.0.0**, source/docs-only prerelease after user approval. Status: Verifying.

[Roadmap](../../ROADMAP.md) · [Progress](progress.md) · [Reports](reports/) · [Photos](../../photos/milestone-00/index.md)

## Goal and dependencies

Finish repository organization, planning records, contributor rules, documentation/wiki, building/game guides, helper validation, and the milestone branch/PR/release contract. No predecessor is required. Native Rust scaffolding remains M01 and is not part of M00 acceptance.

## Implementation tasks

- [x] Establish all 20 implementation milestone plans, progress records, report folders, and photo indexes.
- [x] Preserve original setup reports here; explain their historical M01 wording without altering them.
- [x] Add initial-setup M00 records, current roadmap, and AGENTS.md including Polish mode.
- [x] Document dedicated milestone branches and nightly/beta/stable meanings.
- [x] Require an open PR for user testing and explicit approval before merge/release.
- [x] Add wiki generation, README links, building guidance, and grouped Game Instructions source/pages.
- [ ] Verify source records, helper syntax, wiki generation/navigation, and documentation CI.
- [ ] Push milestone/nightly/beta at the same head; create the M00 PR against stable and attach it to this chat.
- [ ] Receive user testing approval, then merge and publish v0.0.0 from stable.

## Interfaces and compatibility

No gameplay/protocol/save/database contracts are implemented. Stable initially contains the published pre-workflow baseline. Keep main as historical baseline. M01–M20 versions and numbering stay unchanged.

## Acceptance and evidence

Check all 21 record sets and local links, parse every PowerShell helper, generate preview wiki pages from a clean commit, inspect Home/Building/Game Instructions, verify channel SHAs and PR base/head, and wait for CI. Negative checks must reject incomplete milestones and invalid wiki output/approved publishing from non-stable branches.

Archive wiki screenshots as browser documentation evidence, never as native-game evidence. There is no native window to capture in this milestone.

## Release and handoff

Implementation ready is Verifying, not approved. The user tests the PR; explicit approval is mandatory before merging into stable or publishing the setup-only v0.0.0 prerelease. Release notes must say there is no executable. After approval, update records/wiki/release links and stop for the M01 request.
