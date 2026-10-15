# Milestone 08 progress — Private online lobby

Status: Planned  
Target release: v0.2.0  
Last updated: 2026-10-15

[Plan](plan.md) · [Reports](reports/) · [Photos](../../photos/milestone-08/index.md) · [Roadmap](../../ROADMAP.md)

## Landed work

No implementation work has started. This initial record establishes scope only.

## Implementation checklist

- [ ] Implement guest name entry and room creation/joining through codes.
- [ ] Synchronize room configuration, host identity, human/bot/open seats, colors, and readiness.
- [ ] Clear readiness after gameplay settings changes and require a valid configuration before start.
- [ ] Add leave/rejoin behavior, host transfer, and invitation copying.
- [ ] Handle expired guest sessions and invalid/full/missing rooms with useful errors.

## Verification

- [ ] Connect separate desktop clients and verify all see identical accepted lobby changes.
- [ ] Test host-only edits, room capacity, stale requests, readiness reset, and host transfer.
- [ ] Check reconnect preserves guest identity and membership.

Unchecked items are pending, not failed or passed by inference. Structure-check results do not imply application build/runtime verification.

## Remaining work and dependencies

Await completion of milestone 07 and refinement of this plan against the then-current implementation.

## Blockers

No implementation blocker investigated yet; predecessor work remains planned.

## Polish checklist

No active Polish mode request or reported flaw checklist. See [Polish mode](../../AGENTS.md#polish-mode).

## Evidence and next step

No builds, tests, screenshots, reports, or release artifacts exist for this milestone. Next: begin only after the predecessor gate and user request.
