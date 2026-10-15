# Milestone 11 — Cities & Knights

Target release: **v0.5.0**. Status: Planned. This is an initial plan; refine against actual code and dependencies before implementation.

[Roadmap](../../ROADMAP.md) · [Progress](progress.md) · [Reports](reports/) · [Screenshots](../../photos/milestone-11/index.md)

## Goal and dependencies

Deliver complete Cities & Knights games including all core cards and bot choices.

Requires milestone 10 to pass its release gate. Reuse the existing rules/protocol contracts and relevant previously released systems; do not bypass unresolved earlier gates.

## Scope and implementation tasks

- [ ] Implement commodities, expansion setup, event dice, and barbarian progression/resolution.
- [ ] Implement knight placement, activation, upgrades, movement, displacement, and robber interactions.
- [ ] Implement city walls, three improvement tracks, metropolis ownership, merchant, and scoring.
- [ ] Implement every progress card, response, card-limit rule, and replacement of base mechanisms.
- [ ] Add native expansion panels, applicable pregame options, bot strategy, and durable pending responses.

## Interfaces, state, persistence, and UI

Expansion-specific state is explicit and versioned; module capabilities replace base behavior intentionally. Card access and forced choices preserve recipient privacy.

## Tests and acceptance checks

- [ ] Test every progress-card inventory entry and forced response.
- [ ] Cover barbarian ties/defeats, city loss, metropolis changes, and early robber restrictions.
- [ ] Complete offline/online games with bots and test save/reconnect during expansion decisions.

## Screenshot evidence

Capture city improvements, knights, barbarian resolution, progress-card interactions, and final scoring.

Record actual captures and metadata in the linked photo index. Explain nonvisual work and absent captures truthfully.

## Compatibility and migration

Use the milestone-02 pinned rules edition and document interpretation differences. Seafarers combination is not enabled before milestone 13.

## Release gate and handoff

Card/interaction inventory passes and complete offline/online matches are verified.

Update progress, append actual implementation reports, index inspected photos, update ROADMAP.md/CHANGELOG.md, and complete evidence review before marking Complete. Then summarize the release and stop for the next request. User-requested Polish mode pauses advancement.
## Branch, user testing, and GitHub release

Use a dedicated milestone-NN/description branch and PR against stable for this milestone. nightly follows latest pushed work; beta must match the active unapproved PR head. Leave the milestone Verifying and PR open for the user's testing/explicit approval. Do not merge, advance stable, publish the intended GitHub Release, or begin the next milestone before approval. After approval and passing gates, release the merged stable commit and link PR/release/wiki/evidence in the records. See [delivery workflow](../../docs/github-workflow.md).
