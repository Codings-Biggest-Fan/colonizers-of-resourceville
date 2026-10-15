# Milestone 14 — Five to eight players

Target release: **v0.8.0**. Status: Planned. This is an initial plan; refine against actual code and dependencies before implementation.

[Roadmap](../../ROADMAP.md) · [Progress](progress.md) · [Reports](reports/) · [Screenshots](../../photos/milestone-14/index.md)

## Goal and dependencies

Scale the implemented game families to supported larger player counts.

Requires milestone 13 to pass its release gate. Reuse the existing rules/protocol contracts and relevant previously released systems; do not bypass unresolved earlier gates.

## Scope and implementation tasks

- [ ] Add appropriate larger maps, board inventories, and eight distinguishable player identities.
- [ ] Implement official five/six-player turn extensions as distinct rules presets.
- [ ] Specify and implement a labeled seven/eight-player custom extension.
- [ ] Scale lobby, hands, player summaries, response UI, bots, and reconnect handling.
- [ ] Extend released modes only where the documented seat/scenario compatibility allows.

## Interfaces, state, persistence, and UI

Seat identity is independent of display color and user connection. Turn-extension action permissions are explicit engine phases, not UI special cases.

## Tests and acceptance checks

- [ ] Complete 5–8-seat fixtures and mixed-client games.
- [ ] Test out-of-turn permissions, pending responses, awards, resource shortages, and host changes.
- [ ] Measure large-board interaction and server performance under concurrent rooms.

## Screenshot evidence

Capture 5-, 6-, 7-, and 8-seat layouts, turn-extension choices, and scaled player summaries.

Record actual captures and metadata in the linked photo index. Explain nonvisual work and absent captures truthfully.

## Compatibility and migration

Never label custom 7–8-player behavior official; unsupported larger scenario combinations remain blocked.

## Release gate and handoff

Large games finish with correct permissions, privacy, recovery, and measured acceptable responsiveness.

Update progress, append actual implementation reports, index inspected photos, update ROADMAP.md/CHANGELOG.md, and complete evidence review before marking Complete. Then summarize the release and stop for the next request. User-requested Polish mode pauses advancement.
