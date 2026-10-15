# Milestone 15 — Traders & Barbarians

Target release: **v0.9.0**. Status: Planned. This is an initial plan; refine against actual code and dependencies before implementation.

[Roadmap](../../ROADMAP.md) · [Progress](progress.md) · [Reports](reports/) · [Screenshots](../../photos/milestone-15/index.md)

## Goal and dependencies

Implement the pinned core Traders & Barbarians variants and scenarios.

Requires milestone 14 to pass its release gate. Reuse the existing rules/protocol contracts and relevant previously released systems; do not bypass unresolved earlier gates.

## Scope and implementation tasks

- [ ] Implement friendly robber, event cards, harbormaster, and the official two-player variant.
- [ ] Implement fishermen, rivers, caravans, barbarian attack, and transport scenario systems.
- [ ] Add scenario-specific boards, pieces, resources, awards, goals, and pregame options.
- [ ] Add bots for every shipped scenario and native UI for all decisions.
- [ ] Keep scenario inventory and documented combination compatibility current.

## Interfaces, state, persistence, and UI

Variants and scenarios are capability modules with explicit setup, response, scoring, and persistence contracts. Official two-player rules are separate from casual two-seat base play.

## Tests and acceptance checks

- [ ] Cover each core variant/scenario mechanic and objective.
- [ ] Complete each shipped scenario with bots offline and with separate online clients.
- [ ] Test save/reconnect during scenario-specific pending decisions.

## Screenshot evidence

Capture each core scenario's distinctive board/panels, official two-player preset, and representative results.

Record actual captures and metadata in the linked photo index. Explain nonvisual work and absent captures truthfully.

## Compatibility and migration

Do not assume arbitrary combinations with other expansions; supported documented combinations finish in milestone 17.

## Release gate and handoff

Each inventory entry has rules tests, bot support, inspected UI evidence, and completed games.

Update progress, append actual implementation reports, index inspected photos, update ROADMAP.md/CHANGELOG.md, and complete evidence review before marking Complete. Then summarize the release and stop for the next request. User-requested Polish mode pauses advancement.
