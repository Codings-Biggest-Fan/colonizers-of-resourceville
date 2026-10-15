# Milestone 08 — Private online lobby

Target release: **v0.2.0**. Status: Planned. This is an initial plan; refine against actual code and dependencies before implementation.

[Roadmap](../../ROADMAP.md) · [Progress](progress.md) · [Reports](reports/) · [Screenshots](../../photos/milestone-08/index.md)

## Goal and dependencies

Let friends create and join a consistent native online lobby.

Requires milestone 07 to pass its release gate. Reuse the existing rules/protocol contracts and relevant previously released systems; do not bypass unresolved earlier gates.

## Scope and implementation tasks

- [ ] Implement guest name entry and room creation/joining through codes.
- [ ] Synchronize room configuration, host identity, human/bot/open seats, colors, and readiness.
- [ ] Clear readiness after gameplay settings changes and require a valid configuration before start.
- [ ] Add leave/rejoin behavior, host transfer, and invitation copying.
- [ ] Handle expired guest sessions and invalid/full/missing rooms with useful errors.

## Interfaces, state, persistence, and UI

Shared lobby commands/events carry identity and revisions; membership and authorization are server checked. Client-supplied replacement state is never trusted.

## Tests and acceptance checks

- [ ] Connect separate desktop clients and verify all see identical accepted lobby changes.
- [ ] Test host-only edits, room capacity, stale requests, readiness reset, and host transfer.
- [ ] Check reconnect preserves guest identity and membership.

## Screenshot evidence

Archive paired native lobby screenshots before/after a synchronized settings or readiness change.

Record actual captures and metadata in the linked photo index. Explain nonvisual work and absent captures truthfully.

## Compatibility and migration

Integrate milestone-07 identity/persistence without changing offline behavior.

## Release gate and handoff

Separate clients create/join the same room and consistently observe membership and configuration.

Update progress, append actual implementation reports, index inspected photos, update ROADMAP.md/CHANGELOG.md, and complete evidence review before marking Complete. Then summarize the release and stop for the next request. User-requested Polish mode pauses advancement.
