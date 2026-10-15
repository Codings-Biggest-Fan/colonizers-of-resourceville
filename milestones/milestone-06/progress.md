# Milestone 06 progress — Offline playable

Status: Planned  
Target release: v0.1.0  
Last updated: 2026-10-15

[Plan](plan.md) · [Reports](reports/) · [Photos](../../photos/milestone-06/index.md) · [Roadmap](../../ROADMAP.md)

## Landed work

No implementation work has started. This initial record establishes scope only.

## Implementation checklist

- [ ] Add baseline bots that consume only their permitted observations and legal choices.
- [ ] Implement host-selected 2–4 seats with human/bot choices and one local human for solo games.
- [ ] Add offline autosave after accepted actions, resume, restart, and endgame navigation.
- [ ] Use atomic file replacement and validated versioned save loading.
- [ ] Run seeded bot matches and address illegal-action loops or decisions that cannot progress.

## Verification

- [ ] Complete games at each supported seat count.
- [ ] Test save, close, reopen, resume, corrupt saves, and unsupported save versions.
- [ ] Check bot observations exclude opponents' secrets and future randomness; test bounded decision loops.

Unchecked items are pending, not failed or passed by inference. Structure-check results do not imply application build/runtime verification.

## Remaining work and dependencies

Await completion of milestone 05 and refinement of this plan against the then-current implementation.

## Blockers

No implementation blocker investigated yet; predecessor work remains planned.

## Polish checklist

No active Polish mode request or reported flaw checklist. See [Polish mode](../../AGENTS.md#polish-mode).

## Evidence and next step

No builds, tests, screenshots, reports, or release artifacts exist for this milestone. Next: begin only after the predecessor gate and user request.
