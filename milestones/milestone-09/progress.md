# Milestone 09 progress — Reliable online games

Status: Planned  
Target release: v0.3.0  
Last updated: 2026-10-15

[Plan](plan.md) · [Reports](reports/) · [Photos](../../photos/milestone-09/index.md) · [Roadmap](../../ROADMAP.md)

## Landed work

No implementation work has started. This initial record establishes scope only.

## Implementation checklist

- [ ] Execute online rules and bots on the Rust server.
- [ ] Implement per-game command serialization, revision validation, command receipts, and idempotent retries.
- [ ] Commit snapshots and receipts in one transaction before acknowledging an accepted move.
- [ ] Send recipient-specific views and add token refresh and reconnect snapshots.
- [ ] Recover games, pending choices, offers, and bot execution after server restart or database disruption.

## Verification

- [ ] Complete mixed human/bot games across separate clients.
- [ ] Test duplicate, stale, concurrent, malformed, out-of-turn, and unauthorized commands.
- [ ] Interrupt server/database connections and prove no acknowledged move is lost or executed twice.
- [ ] Inspect every recipient role for private-state leaks.

Unchecked items are pending, not failed or passed by inference. Structure-check results do not imply application build/runtime verification.

## Remaining work and dependencies

Await completion of milestone 08 and refinement of this plan against the then-current implementation.

## Blockers

No implementation blocker investigated yet; predecessor work remains planned.

## Polish checklist

No active Polish mode request or reported flaw checklist. See [Polish mode](../../AGENTS.md#polish-mode).

## Evidence and next step

No builds, tests, screenshots, reports, or release artifacts exist for this milestone. Next: begin only after the predecessor gate and user request.
