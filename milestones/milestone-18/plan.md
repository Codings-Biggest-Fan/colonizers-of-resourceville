# Milestone 18 — Public casual platform

Target release: **v0.12.0**. Status: Planned. This is an initial plan; refine against actual code and dependencies before implementation.

[Roadmap](../../ROADMAP.md) · [Progress](progress.md) · [Reports](reports/) · [Screenshots](../../photos/milestone-18/index.md)

## Goal and dependencies

Add public discovery and social features without weakening private-room boundaries.

Requires milestone 17 to pass its release gate. Reuse the existing rules/protocol contracts and relevant previously released systems; do not bypass unresolved earlier gates.

## Scope and implementation tasks

- [ ] Add optional email accounts and safe anonymous-guest linking.
- [ ] Add public room discovery, filters, casual matchmaking, profiles, and friend/invitation flows.
- [ ] Add spectators with permitted/delayed views and post-game replay rules.
- [ ] Add chat permissions, mute, block, report, and moderation records.
- [ ] Add abuse controls and rate limits around identity, discovery, joining, chat, and reports.

## Interfaces, state, persistence, and UI

Authorization uses verified identity/membership, never user-editable profile metadata. Private rooms/passwords and blocked identities are respected by discovery and invitations.

## Tests and acceptance checks

- [ ] Verify account linking retains membership and permitted history without identity takeover.
- [ ] Test discovery visibility, private room/password boundaries, blocks, and moderation permissions.
- [ ] Inspect spectator/chat/replay payloads and logs for leaks and unsafe input rendering.

## Screenshot evidence

Capture public listing, account linking, invitation, spectator view, and mute/block/report flows with synthetic accounts.

Record actual captures and metadata in the linked photo index. Explain nonvisual work and absent captures truthfully.

## Compatibility and migration

Guests can still play casual games; persistent accounts are optional outside ranked queues.

## Release gate and handoff

Public/social flows work and visibility, identity, moderation, and spectator boundaries pass.

Update progress, append actual implementation reports, index inspected photos, update ROADMAP.md/CHANGELOG.md, and complete evidence review before marking Complete. Then summarize the release and stop for the next request. User-requested Polish mode pauses advancement.
