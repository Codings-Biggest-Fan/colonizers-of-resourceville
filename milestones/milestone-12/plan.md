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
- [ ] Implement Mistbound fog maps: initially hidden tiles reveal when roads or settlements are built nearby, as requested on 2026-10-15. Verify exact adjacency, ship triggers, discovery rewards and public/private visibility against the pinned rules before coding; see the [fog knowledge note](../../docs/rules.md#fog-maps--user-observation-2026-10-15).

## Interfaces, state, persistence, and UI

Scenario definitions declare topology, resources, objectives, legal setup, and supported seats. Keep exploration secrets separate from the public board.

## Tests and acceptance checks

- [ ] Cover ship movement, pirate blocking, mixed routes, gold choices, and discovery.
- [ ] Validate every core scenario setup and objective calculation.
- [ ] Complete each supported scenario offline/online and resume mid-exploration.
- [ ] Test fog reveal boundaries, repeated discovery, unrevealed-tile privacy, road/settlement and verified ship triggers, deterministic replay, and save/reconnect before and after reveal. Capture inspected hidden/revealed board states without exposing unrevealed terrain.

## Screenshot evidence

Capture ships/pirate, discovery, gold selection, scenario preview, and scenario results.

Record actual captures and metadata in the linked photo index. Explain nonvisual work and absent captures truthfully.

## Compatibility and migration

Enable only supported scenario/player-count combinations; combination with Citadels & Sentinels remains gated by milestone 13.

## Release gate and handoff

Every shipped core scenario completes with correct setup, objectives, and scoring.

Update progress, append actual implementation reports, index inspected photos, update ROADMAP.md/CHANGELOG.md, and complete evidence review before marking Complete. Then summarize the release and stop for the next request. User-requested Polish mode pauses advancement.
## Branch, user testing, and GitHub release

Use one milestone branch for implementation and polish. Implementation PR targets beta, which changes only after explicit approval; nightly follows current development. Stable promotion requires separate polish approval. Leave the milestone Verifying for user review; do not merge, promote, release or begin the next milestone without the required direction. Publication follows the substantial-feature release cadence in [AGENTS.md](../../AGENTS.md) and [workflow](../../docs/github-workflow.md).
