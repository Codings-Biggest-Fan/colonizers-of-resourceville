# Milestone 13 — Combined expansions

Target release: **v0.7.0**. Status: Planned. This is an initial plan; refine against actual code and dependencies before implementation.

[Roadmap](../../ROADMAP.md) · [Progress](progress.md) · [Reports](reports/) · [Screenshots](../../photos/milestone-13/index.md)

## Goal and dependencies

Support documented Seafarers plus Cities & Knights combinations.

Requires milestone 12 to pass its release gate. Reuse the existing rules/protocol contracts and relevant previously released systems; do not bypass unresolved earlier gates.

## Scope and implementation tasks

- [ ] Implement documented setup and pirate/barbarian interactions for supported scenarios.
- [ ] Unify route/knight interactions where official combination rules require them.
- [ ] Implement combined objectives, score thresholds, pending choices, and bot planning.
- [ ] Add combination presets and actionable conflict messages.
- [ ] Record unsupported combinations explicitly rather than silently blending rule systems.

## Interfaces, state, persistence, and UI

Capability composition has explicit precedence and compatibility declarations; combined views and saves identify both expansion versions and the scenario.

## Tests and acceptance checks

- [ ] Exercise cross-expansion knight/route/pirate/barbarian interactions.
- [ ] Complete supported combinations offline/online with bot and human mixtures.
- [ ] Test invalid combinations, save/resume, and privacy during combined responses.

## Screenshot evidence

Capture combined lobby configuration, nautical/knight board, barbarian effects, and combined result summary.

Record actual captures and metadata in the linked photo index. Explain nonvisual work and absent captures truthfully.

## Compatibility and migration

Only documented combinations are supported. Match the pinned rulebook combination notes and preserve base/standalone-expansion behavior.

## Release gate and handoff

Supported combinations finish correctly; incompatible settings are clearly rejected.

Update progress, append actual implementation reports, index inspected photos, update ROADMAP.md/CHANGELOG.md, and complete evidence review before marking Complete. Then summarize the release and stop for the next request. User-requested Polish mode pauses advancement.
## Branch, user testing, and GitHub release

Use a dedicated milestone-NN/description branch and PR against stable for this milestone. nightly follows latest pushed work; beta must match the active unapproved PR head. Leave the milestone Verifying and PR open for the user's testing/explicit approval. Do not merge, advance stable, publish the intended GitHub Release, or begin the next milestone before approval. After approval and passing gates, release the merged stable commit and link PR/release/wiki/evidence in the records. See [delivery workflow](../../docs/github-workflow.md).
