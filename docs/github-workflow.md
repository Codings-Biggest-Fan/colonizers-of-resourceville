# Branches, approval, and GitHub Releases

| Branch | Meaning |
| --- | --- |
| milestone-NN/description | Implementation work; PR targets beta |
| nightly | Latest pushed development commit, including unfinished work |
| beta | Latest user-approved implementation, before polish |
| stable | User-approved implementation and user-approved polish; default branch |
| main | Compatibility mirror advanced only with polished stable |

1. Implement the active milestone and push its dedicated branch plus nightly without force.
2. Open the implementation PR against beta with exact revision, changes, actual checks, manual steps, wiki, evidence, and limitations. Leave beta unchanged while review is pending.
3. Stop for explicit implementation approval before merging into beta.
4. Perform requested polish on the milestone or a dedicated polish branch. Update nightly while beta remains at its approved baseline.
5. Open the polished PR against stable and stop for explicit approval of that revision.
6. Only after polished approval and passing gates merge into stable, fast-forward main, and publish the milestone release from stable. Stop before the next milestone.

A request to finish work or open a PR is not merge/release approval. Preserve divergence and branch-specific history. Never force-push or overwrite published releases/tags. The one-time AGENTS.md channel-policy correction authorized across branches does not approve implementation changes.

M00 initial setup and polish were merged through PRs [#1](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/pull/1) and [#2](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/pull/2). A merge alone is not proof of a published release. M01 begins by explicit user request on 2026-10-15.

## Release artifacts and notes

Build releases on GitHub Actions for Windows/Linux/macOS x86-64 and ARM64, including Windows ARM64. Use artifacts from the approved polished stable revision and verify checksums. Record build, package, test, and actual runtime results separately for each target. Do not imply nonlocal interactive testing from successful compilation. See [native CI](../.github/workflows/native.yml) and [Building](building.md).

M00 targets source/docs v0.0.0; M01 targets v0.0.1. Pre-v1.0.0 releases are prereleases. Include approved PRs/commit, changes, actual validation, manual instructions, wiki/evidence, artifacts/checksums, migrations, and limitations. No workflow automatically publishes a release.

## Wiki

Generate from reviewed sources using scripts/wiki.ps1. Development and approved-but-unpolished beta pages must prominently say TESTING PREVIEW, identifying source channel/commit and PR. Publish approved release docs only from polished stable after approval. The generator does not push. The [Game Instructions hub](game-instructions/README.md) groups player-guide sources in the wiki's flat namespace.
