# Milestone 04 — Base turn engine

Target release: **v0.0.4**. Status: Planned. This is an initial plan; refine against actual code and dependencies before implementation.

[Roadmap](../../ROADMAP.md) · [Progress](progress.md) · [Reports](reports/) · [Screenshots](../../photos/milestone-04/index.md)

## Goal and dependencies

Implement deterministic base-game setup and ordinary building turns.

Requires milestone 03 to pass its release gate. Reuse the existing rules/protocol contracts and relevant previously released systems; do not bypass unresolved earlier gates.

## Scope and implementation tasks

- [ ] Implement explicit setup and turn phases with snake placement order and starting resource allocation.
- [ ] Implement dice production and finite-bank shortage behavior.
- [ ] Implement road, settlement, and city costs, connectivity, distance checks, and piece limits.
- [ ] Expose legal actions and validate complete transitions before changing state.
- [ ] Drive the native board from engine state with active-player and phase cues.

## Interfaces, state, persistence, and UI

Rules accepts explicit actions and controlled random outcomes, returns new state/events or actionable errors, and has no UI, network, database, or async dependency.

## Tests and acceptance checks

- [ ] Exercise full setup order and placement restrictions.
- [ ] Verify production, shortages, spending, resource conservation, and piece counts.
- [ ] Prove rejected actions do not mutate game state and scripted turns replay deterministically.

## Screenshot evidence

Capture setup placement, legal-target highlighting, production state, and a rejected-action explanation.

Record actual captures and metadata in the linked photo index. Explain nonvisual work and absent captures truthfully.

## Compatibility and migration

Use the milestone-02 contract; add documented version changes if the state shape must change.

## Release gate and handoff

Deterministic setup and ordinary-turn tests pass; rejected actions are atomic.

Update progress, append actual implementation reports, index inspected photos, update ROADMAP.md/CHANGELOG.md, and complete evidence review before marking Complete. Then summarize the release and stop for the next request. User-requested Polish mode pauses advancement.
## Branch, user testing, and GitHub release

Use a dedicated milestone-NN/description branch and PR against stable for this milestone. nightly follows latest pushed work; beta must match the active unapproved PR head. Leave the milestone Verifying and PR open for the user's testing/explicit approval. Do not merge, advance stable, publish the intended GitHub Release, or begin the next milestone before approval. After approval and passing gates, release the merged stable commit and link PR/release/wiki/evidence in the records. See [delivery workflow](../../docs/github-workflow.md).
