# Milestone 20 — Release

Target release: **v1.0.0**. Status: Planned. This is an initial plan; refine against actual code and dependencies before implementation.

[Roadmap](../../ROADMAP.md) · [Progress](progress.md) · [Reports](reports/) · [Screenshots](../../photos/milestone-20/index.md)

## Goal and dependencies

Ship a verified Windows release with complete scope and evidence accounting.

Requires milestone 19 to pass its release gate. Reuse the existing rules/protocol contracts and relevant previously released systems; do not bypass unresolved earlier gates.

## Scope and implementation tasks

- [ ] Close release blockers and reconcile every milestone's acceptance/evidence records.
- [ ] Improve bot difficulty without hidden-information advantages.
- [ ] Complete accessibility, Windows display scaling, performance, and clean-install verification.
- [ ] Package the Windows application and server container; publish installation, migration, backup, and operating guidance.
- [ ] Publish final feature, scenario, pregame catalogue, compatibility, and limitations coverage.

## Interfaces, state, persistence, and UI

Freeze and document supported protocol/save/map/preset versions and upgrade behavior. Keep platform packages limited to Windows for this release.

## Tests and acceptance checks

- [ ] Run the full rules/card/scenario/configuration and privacy matrix.
- [ ] Install on a clean Windows environment and complete offline and deployed online flows.
- [ ] Test save migrations, corrupt inputs, identity recovery limitations, accessibility, performance, and backup/restore.

## Screenshot evidence

Capture final home, setup, representative expansion games, accessibility/scaling cases, online play, and release bundle launch.

Record actual captures and metadata in the linked photo index. Explain nonvisual work and absent captures truthfully.

## Compatibility and migration

No unsupported platform/package claims; retain explicit exclusions and original asset attribution. No in-game purchase system is required.

## Release gate and handoff

Clean-install acceptance passes; all release evidence is linked and the final coverage/limitations report is published.

Update progress, append actual implementation reports, index inspected photos, update ROADMAP.md/CHANGELOG.md, and complete evidence review before marking Complete. Then summarize the release and stop for the next request. User-requested Polish mode pauses advancement.
## Branch, user testing, and GitHub release

Use a dedicated milestone-NN/description branch and PR against stable for this milestone. nightly follows latest pushed work; beta must match the active unapproved PR head. Leave the milestone Verifying and PR open for the user's testing/explicit approval. Do not merge, advance stable, publish the intended GitHub Release, or begin the next milestone before approval. After approval and passing gates, release the merged stable commit and link PR/release/wiki/evidence in the records. See [delivery workflow](../../docs/github-workflow.md).
