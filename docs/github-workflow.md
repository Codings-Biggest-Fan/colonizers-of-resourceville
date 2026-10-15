# Branches, approval, and GitHub Releases

| Branch | Meaning |
| --- | --- |
| milestone-NN/description | Work for one milestone, reviewed through its PR against stable |
| nightly | Latest pushed active-milestone commit, possibly unfinished |
| beta | Current unapproved milestone PR head for user testing |
| stable | Latest explicitly approved, merged PR; default branch |
| main | Compatibility mirror of the latest approved stable commit |

Before the first approval, stable contains the initial baseline, not a released milestone. M00 is Initial setup; M01–M20 keep their existing scope/versions.

Current review history: initial setup merged through [PR #1](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/pull/1) at 624223e. Later README/polish changes are reviewed separately in [PR #2](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/pull/2); this new PR does not authorize its own merge.

## Workflow

1. Branch from stable; refine the plan and implement one milestone.
2. Push increments to its branch and nightly. Open one PR against stable when ready for testing.
3. Point beta to that PR head. Push further milestone/nightly/beta updates atomically without force; beta must match the tested PR revision.
4. Record actual checks, manual steps, limitations, and evidence. Publish clearly labeled preview wiki pages.
5. Leave the milestone Verifying and PR open while the user tests. Never merge, advance stable, release, or begin the next milestone without explicit user approval of the revision.
6. After approval and passing gates, merge the approved head, publish approved wiki docs, and create its GitHub Release from the resulting stable commit.
7. Record approval/PR/release links and completion; stop for the next request.

Corrections during testing belong to the same PR. Post-release polish uses its own branch/PR. Do not rewrite published history or overwrite releases.

After explicit approval use a merge commit that preserves the milestone commits, so later milestone branches created from stable remain descendants of the current nightly/beta refs and can advance them without force.

## Release notes

M00 targets v0.0.0, a source/documentation-only setup prerelease after approval. M01–M20 use [roadmap versions](../ROADMAP.md). All pre-v1.0.0 releases are prereleases. No release currently exists.

Include milestone, approved PR/head, merged commit, changes, actual validation, user testing instructions, evidence, build/game/wiki links, limitations, and migrations. Attach verified artifacts/checksums when produced. M00 has no native executable; automatic source archives are not native builds.

## Wiki

The wiki is generated from PR-reviewed source. scripts/wiki.ps1 writes pages only; it never pushes, merges, or publishes releases. Unapproved documentation is labeled TESTING PREVIEW with its source commit. Republish from stable after approval.

GitHub wikis use a flat page namespace; Game Instructions is a hub/sidebar group backed by the real [source folder](game-instructions/README.md). Initialize the first wiki page through GitHub before cloning its separate Git repository. [Official wiki guidance](https://docs.github.com/en/communities/documenting-your-project-with-wikis/adding-or-editing-wiki-pages)
