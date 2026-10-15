# Milestone 16 — Voyagers & Corsairs

Target release: **v0.10.0**. Status: Planned. This is an initial plan; refine against actual code and dependencies before implementation.

[Roadmap](../../ROADMAP.md) · [Progress](progress.md) · [Reports](reports/) · [Screenshots](../../photos/milestone-16/index.md)

## Goal and dependencies

Implement Voyagers & Corsairs as its own rules family with every pinned core scenario.

Requires milestone 15 to pass its release gate. Reuse the existing rules/protocol contracts and relevant previously released systems; do not bypass unresolved earlier gates.

## Scope and implementation tasks

- [ ] Implement distinct setup/economy, ships, crews, settlers, and cargo.
- [ ] Implement exploration, pirate interaction, fish, spice trade, and pirate lairs.
- [ ] Implement mission tracks, rewards, objectives, scenario progression, and victory.
- [ ] Build dedicated pregame settings, choice UI, map overlays, and bot decisions.
- [ ] Persist cargo, mission, exploration, and response state for save/reconnect.

## Interfaces, state, persistence, and UI

Do not reuse Open Seas ship rules by assumption. Explicit rules-family capabilities identify replacement mechanics and scenario-specific state.

## Tests and acceptance checks

- [ ] Cover cargo loading/unloading, movement, exploration, mission rewards, and objective transitions.
- [ ] Complete every supported core scenario offline/online with bots.
- [ ] Test concealed exploration, save compatibility, and interrupted mission choices.

## Screenshot evidence

Capture cargo management, exploration, each mission type, pregame configuration, and final mission scoring.

Record actual captures and metadata in the linked photo index. Explain nonvisual work and absent captures truthfully.

## Compatibility and migration

Reject unsupported cross-expansion combinations; document the pinned rules edition and supported seat counts.

## Release gate and handoff

Mission/cargo transitions pass and every shipped scenario completes correctly.

Update progress, append actual implementation reports, index inspected photos, update ROADMAP.md/CHANGELOG.md, and complete evidence review before marking Complete. Then summarize the release and stop for the next request. User-requested Polish mode pauses advancement.
## Branch, user testing, and GitHub release

Use a dedicated milestone-NN/description branch and PR against stable for this milestone. nightly follows latest pushed work; beta must match the active unapproved PR head. Leave the milestone Verifying and PR open for the user's testing/explicit approval. Do not merge, advance stable, publish the intended GitHub Release, or begin the next milestone before approval. After approval and passing gates, release the merged stable commit and link PR/release/wiki/evidence in the records. See [delivery workflow](../../docs/github-workflow.md).
