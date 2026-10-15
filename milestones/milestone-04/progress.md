# Milestone 04 progress — Base turn engine

Status: Planned  
Target release: v0.0.4  
Last updated: 2026-10-15

[Plan](plan.md) · [Reports](reports/) · [Photos](../../photos/milestone-04/index.md) · [Roadmap](../../ROADMAP.md)

## Landed work

No implementation work has started. This initial record establishes scope only.

## Implementation checklist

- [ ] Implement explicit setup and turn phases with snake placement order and starting resource allocation.
- [ ] Implement dice production and finite-bank shortage behavior.
- [ ] Implement road, settlement, and city costs, connectivity, distance checks, and piece limits.
- [ ] Expose legal actions and validate complete transitions before changing state.
- [ ] Drive the native board from engine state with active-player and phase cues.

## Verification

- [ ] Exercise full setup order and placement restrictions.
- [ ] Verify production, shortages, spending, resource conservation, and piece counts.
- [ ] Prove rejected actions do not mutate game state and scripted turns replay deterministically.

Unchecked items are pending, not failed or passed by inference. Structure-check results do not imply application build/runtime verification.

## Remaining work and dependencies

Await completion of milestone 03 and refinement of this plan against the then-current implementation.

## Blockers

No implementation blocker investigated yet; predecessor work remains planned.

## Polish checklist

No active Polish mode request or reported flaw checklist. See [Polish mode](../../AGENTS.md#polish-mode).

## Evidence and next step

No builds, tests, screenshots, reports, or release artifacts exist for this milestone. Next: begin only after the predecessor gate and user request.
