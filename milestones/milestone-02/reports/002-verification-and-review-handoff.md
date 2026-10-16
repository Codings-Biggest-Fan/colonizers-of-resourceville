# 002 — M02 verification and review handoff

Date: 2026-10-15 (America/Los_Angeles). Implementation source: fd6c23657fa6c8a54d3c8b42fb42f1fb91ec4d2a. Subsequent handoff edits are documentation/evidence only; they do not change Rust, dependency, fixture, source-inventory or helper behavior tested at that revision.

## Outcome

M02 is Complete. The user explicitly confirmed merging [implementation PR #7](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/pull/7); GitHub verifies tested source fd6c236 merged into beta at 3f32f09c2900000173e5baff0073ec17a384fba2 on 2026-10-15 (America/Los_Angeles). Beta is the approved implementation baseline. Completion records are subsequent documentation/evidence on milestone-02/rules-contract and nightly; stable/main were not promoted. No polish approval is inferred. No release/tag is due before M06, and none was created. M03 has not started.

Delivered configuration/default/custom/validation contracts; readiness invalidation and immutable acceptance; versioned command/event/save boundaries with bounded ingress; synthetic public/private recipient projections; source-backed core inventories, public guest audit and milestone assignments. Added requested fog knowledge and M12/M17 tasks, plus planned M21 website/wiki/docs records. No gameplay or native UI was added.

## Executed local checks

| Check | Actual result |
| --- | --- |
| scripts/check.ps1 -Release | Formatting, all-target/all-feature Clippy with warnings denied, optimized workspace build passed |
| scripts/test.ps1 | 13 tests passed: 6 config/lifecycle/fixture, 1 source inventory, 5 protocol/privacy, 1 existing server |
| scripts/milestone.ps1 -All | 22 record sets and all local links passed; final handoff link count is recorded in current progress |
| scripts/milestone.ps1 -Number 21 | Passed after fixing the observed scalar Count error; no milestone completion inferred |
| scripts/package.ps1 -SkipBuild | Clean committed source fd6c236 packaged successfully; unsigned Windows x86-64 preview |
| Independent local archive review | ZIP CRC, ZIP SHA-256, 649 included file hashes, BUILD.txt source and both PE architecture headers passed |
| scripts/wiki.ps1 -PullRequestUrl .../pull/7 | 141 generated pages, TESTING PREVIEW; mapping/sidebar includes M21 |
| git diff --check | Passed before source commit; later handoff checked separately |

Local archive SHA-256: cfebe467e85831e945c6ee2ed2dd03e799c30a9b7b26626b48847fe9cf864aa2. [Machine result](evidence/local-package-fd6c236.json). The package is a testing preview, not a published release or new native UI/runtime validation.

Initial failures/corrections remain recorded in report 001: test Clippy style, refused dirty-tree package, missing optional PyMuPDF with existing Poppler fallback. The GitHub connector could read but could not create/ready the PR (integration 403); existing repository Git credentials completed those operations through the GitHub API without printing or storing credentials. No new permission or account was created.

## GitHub six-target gate

[Native builds run 38853803033](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/actions/runs/38853803033) passed at fd6c236. [Documentation run 38853803019](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/actions/runs/38853803019) also passed. [Archived job/step results](evidence/ci-jobs-fd6c236.json).

| Runner / architecture | Build, tests, package | Artifact ID |
| --- | --- | --- |
| Windows x86-64 | Passed | 12026541036 |
| Windows ARM64 | Passed | 12026341803 |
| Linux x86-64, including server image | Passed | 12025354958 |
| Linux ARM64, including server image | Passed | 12026486196 |
| macOS x86-64 | Passed | 12026287563 |
| macOS ARM64 | Passed | 12026137820 |

Downloaded and independently inspected every actual GitHub artifact: outer artifact digest matched GitHub, nested package SHA-256 and ZIP CRC passed, all 649 file hashes per package passed (3,894 total), BUILD.txt matched source, both binary architecture headers matched target, and Unix executable bits were present. Both Docker archives matched SHA-256 and Linux amd64/arm64 image configuration. [Machine artifact/hash results](evidence/ci-artifacts-fd6c236.json). No nonlocal interactive runtime testing is inferred.

## Wiki and evidence

Published [TESTING PREVIEW wiki](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/wiki), initially from source fd6c236 in wiki commit 8f71b3a, with PR #7, source commit and channel in each banner. Home, Building, Game Instructions and M21 plan returned HTTP 200 with the expected source/preview/navigation markers; [HTTP evidence](evidence/wiki-http-fd6c236.json). Final handoff sources are regenerated after their commit, preserving those preview labels. No approved-release documentation claim is made.

No new native screenshots apply: native code and controls are unchanged. [M02 photo index](../../../photos/milestone-02/index.md) records this; README keeps honest M01 images. Research PDFs/rendered pages remain ignored local inputs, not copied product assets.

## Limitations and exact next step

No playable rules, board generation, bots, save writer, live room dispatch, authentication or cloud persistence. Initial format versions reject incompatible data with no migration. Catalogue locks, paid/authenticated behavior, exact fog proximity/ship triggers, event-card face distribution, larger expansion layouts and broader combination execution remain assigned later work. Synthetic privacy tests are not multiplayer or shoulder-surfing evidence.

The user approved and merged PR #7 after the implementation and six-target gates passed. The native screen remains the M01 preview. M03 is next and awaits a new request. Requested polish continues on this same milestone branch and nightly, with separate approval before stable/main promotion. M21 is planning only; website deployment/publication is not authorized. Do not start M03 automatically.
