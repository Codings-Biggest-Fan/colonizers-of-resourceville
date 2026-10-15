# Milestone 09 — Reliable online games

Target release: **v0.3.0**. Status: Planned. This is an initial plan; refine against actual code and dependencies before implementation.

[Roadmap](../../ROADMAP.md) · [Progress](progress.md) · [Reports](reports/) · [Screenshots](../../photos/milestone-09/index.md)

## Goal and dependencies

Make base-game multiplayer authoritative, durable, and recoverable.

Requires milestone 08 to pass its release gate. Reuse the existing rules/protocol contracts and relevant previously released systems; do not bypass unresolved earlier gates.

## Scope and implementation tasks

- [ ] Execute online rules and bots on the Rust server.
- [ ] Implement per-game command serialization, revision validation, command receipts, and idempotent retries.
- [ ] Commit snapshots and receipts in one transaction before acknowledging an accepted move.
- [ ] Send recipient-specific views and add token refresh and reconnect snapshots.
- [ ] Recover games, pending choices, offers, and bot execution after server restart or database disruption.

## Interfaces, state, persistence, and UI

Versioned commands include command ID, game ID, expected revision, and authenticated identity. Reconnect returns a permitted snapshot at a committed revision.

## Tests and acceptance checks

- [ ] Complete mixed human/bot games across separate clients.
- [ ] Test duplicate, stale, concurrent, malformed, out-of-turn, and unauthorized commands.
- [ ] Interrupt server/database connections and prove no acknowledged move is lost or executed twice.
- [ ] Inspect every recipient role for private-state leaks.

## Screenshot evidence

Capture paired game views, reconnect feedback, and matching recovered game revisions without exposing hidden hands.

Record actual captures and metadata in the linked photo index. Explain nonvisual work and absent captures truthfully.

## Compatibility and migration

Preserve offline save behavior; document network protocol upgrade behavior and database migration requirements.

## Release gate and handoff

Online matches complete with consistent state and demonstrated retry/restart/failure recovery.

Update progress, append actual implementation reports, index inspected photos, update ROADMAP.md/CHANGELOG.md, and complete evidence review before marking Complete. Then summarize the release and stop for the next request. User-requested Polish mode pauses advancement.
## Branch, user testing, and GitHub release

Use a dedicated milestone-NN/description branch and PR against stable for this milestone. nightly follows latest pushed work; beta must match the active unapproved PR head. Leave the milestone Verifying and PR open for the user's testing/explicit approval. Do not merge, advance stable, publish the intended GitHub Release, or begin the next milestone before approval. After approval and passing gates, release the merged stable commit and link PR/release/wiki/evidence in the records. See [delivery workflow](../../docs/github-workflow.md).
