# Milestone 04 progress — Base turn engine

Status: Planned  
Target release: v0.0.4  
Last updated: 2026-10-15

[Plan](plan.md) · [Reports](reports/) · [Photos](../../photos/milestone-04/index.md) · [Roadmap](../../ROADMAP.md)

## Landed work

No implementation work has started. Recorded the user’s M04 visual follow-up requirements on 2026-10-15; see [planning report](reports/001-visual-follow-up-requirements.md).

## Implementation checklist

- [ ] Implement explicit setup and turn phases with snake placement order and starting resource allocation.
- [ ] Implement dice production and finite-bank shortage behavior.
- [ ] Implement road, settlement, and city costs, connectivity, distance checks, and piece limits.
- [ ] Expose legal actions and validate complete transitions before changing state.
- [ ] Drive the native board from engine state with active-player and phase cues.

## Requested visual work

- [ ] Flat 2D ship graphics for all ports; resource cargo identifies specific ports.
- [ ] Question-mark graphic for generic 3:1 ports; no permanent port text labels.
- [ ] Much stronger settlement/road contrast across all terrain, zoom and scaling cases.
- [ ] Preserve light sea, uncluttered board, wheel zoom and right-drag pan.

## Verification

- [ ] Exercise full setup order and placement restrictions.
- [ ] Verify production, shortages, spending, resource conservation, and piece counts.
- [ ] Prove rejected actions do not mutate game state and scripted turns replay deterministically.

Unchecked items are pending, not failed or passed by inference. Structure-check results do not imply application build/runtime verification.

## Remaining work and dependencies

M03 is Complete and approved through PRs #9/#10. M04 remains Planned; refine the engine plan against that accepted code and begin only after a new implementation request.

## Blockers

No known planning blocker; implementation dependencies have not been investigated in this documentation-only session.

## Polish checklist

No active Polish mode request. The requested visual follow-ups are recorded above for M04. See [Polish mode](../../AGENTS.md#polish-mode).

## Evidence and next step

No builds, tests, screenshots, reports, or release artifacts exist for this milestone. The planning report records requirements only; begin implementation only after the next user request.
