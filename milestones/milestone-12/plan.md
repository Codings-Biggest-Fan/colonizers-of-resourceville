# Milestone 12 — Open Seas

Target release: **v0.6.0**. Status: Planned. This is an initial plan; refine against actual code and dependencies before implementation.

[Roadmap](../../ROADMAP.md) · [Progress](progress.md) · [Reports](reports/) · [Screenshots](../../photos/milestone-12/index.md)

## Goal and dependencies

Deliver the pinned Open Seas core scenarios with bots and correct nautical rules.

Requires milestone 11 to pass its release gate. Reuse the existing rules/protocol contracts and relevant previously released systems; do not bypass unresolved earlier gates.

## Scope and implementation tasks

- [ ] Implement ships, route connectivity, movement constraints, and road/ship transitions.
- [ ] Implement pirate interactions, exploration, islands, and gold selection.
- [ ] Implement each pinned core scenario's setup, objectives, special rules, and scoring.
- [ ] Extend board rendering, scenario preview, choice dialogs, and bots.
- [ ] Persist hidden exploration data without sending it to unauthorized players.

## Interfaces, state, persistence, and UI

Scenario definitions declare topology, resources, objectives, legal setup, and supported seats. Keep exploration secrets separate from the public board.

## Tests and acceptance checks

- [ ] Cover ship movement, pirate blocking, mixed routes, gold choices, and discovery.
- [ ] Validate every core scenario setup and objective calculation.
- [ ] Complete each supported scenario offline/online and resume mid-exploration.

## Screenshot evidence

Capture ships/pirate, discovery, gold selection, scenario preview, and scenario results.

Record actual captures and metadata in the linked photo index. Explain nonvisual work and absent captures truthfully.

## Compatibility and migration

Enable only supported scenario/player-count combinations; combination with Citadels & Sentinels remains gated by milestone 13.

## Release gate and handoff

Every shipped core scenario completes with correct setup, objectives, and scoring.

Update progress, append actual implementation reports, index inspected photos, update ROADMAP.md/CHANGELOG.md, and complete evidence review before marking Complete. Then summarize the release and stop for the next request. User-requested Polish mode pauses advancement.
## Branch, user testing, and GitHub release

Use a dedicated milestone-NN/description branch and PR against stable for this milestone. nightly follows latest pushed work; beta must match the active unapproved PR head. Leave the milestone Verifying and PR open for the user's testing/explicit approval. Do not merge, advance stable, publish the intended GitHub Release, or begin the next milestone before approval. After approval and passing gates, release the merged stable commit and link PR/release/wiki/evidence in the records. See [delivery workflow](../../docs/github-workflow.md).
