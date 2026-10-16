# Milestone 12 progress — Open Seas

Status: Planned  
Target release: v0.6.0  
Last updated: 2026-10-15

[Plan](plan.md) · [Reports](reports/) · [Photos](../../photos/milestone-12/index.md) · [Roadmap](../../ROADMAP.md)

## Landed work

No implementation work has started. This initial record establishes scope only.

## Implementation checklist

- [ ] Implement ships, route connectivity, movement constraints, and road/ship transitions.
- [ ] Implement pirate interactions, exploration, islands, and gold selection.
- [ ] Implement each pinned core scenario's setup, objectives, special rules, and scoring.
- [ ] Extend board rendering, scenario preview, choice dialogs, and bots.
- [ ] Persist hidden exploration data without sending it to unauthorized players.

## Verification

- [ ] Cover ship movement, pirate blocking, mixed routes, gold choices, and discovery.
- [ ] Validate every core scenario setup and objective calculation.
- [ ] Complete each supported scenario offline/online and resume mid-exploration.

Unchecked items are pending, not failed or passed by inference. Structure-check results do not imply application build/runtime verification.

## Remaining work and dependencies

Await completion of milestone 11 and refinement of this plan against the then-current implementation.

## Blockers

No implementation blocker investigated yet; predecessor work remains planned.

## Polish checklist

No active Polish mode request or reported flaw checklist. See [Polish mode](../../AGENTS.md#polish-mode).

## Evidence and next step

No builds, tests, screenshots, reports, or release artifacts exist for this milestone. Next: begin only after the predecessor gate and user request.

## User planning addition — 2026-10-15

Mistbound fog reveal on nearby road/settlement construction, source-verified exact adjacency/ship triggers, hidden-state privacy and before/after recovery tests are assigned in the plan. No fog gameplay has been implemented.
