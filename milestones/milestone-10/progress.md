# Milestone 10 progress — Base pregame completeness

Status: Planned  
Target release: v0.4.0  
Last updated: 2026-10-15

[Plan](plan.md) · [Reports](reports/) · [Photos](../../photos/milestone-10/index.md) · [Roadmap](../../ROADMAP.md)

## Landed work

No implementation work has started. This initial record establishes scope only.

## Implementation checklist

- [ ] Implement standard/custom presets, victory targets, dice policy, robber protection, and discard thresholds.
- [ ] Implement player trade policy, offer expiry, turn order, timers, and legal timeout decisions.
- [ ] Implement board terrain/number/port randomization and validated fixed/seeded layouts.
- [ ] Implement room passwords and configurable disconnect/pause/substitution behavior.
- [ ] Persist and synchronize presets/settings; display custom-rule markers and conflicts.

## Verification

- [ ] Map each base-applicable catalogue row to a behavior test.
- [ ] Round-trip presets and settings through persistence and reconnect.
- [ ] Exercise deadline expiration during placement, trades, discards, and other pending choices.

Unchecked items are pending, not failed or passed by inference. Structure-check results do not imply application build/runtime verification.

## Remaining work and dependencies

Await completion of milestone 09 and refinement of this plan against the then-current implementation.

## Blockers

No implementation blocker investigated yet; predecessor work remains planned.

## Polish checklist

No active Polish mode request or reported flaw checklist. See [Polish mode](../../AGENTS.md#polish-mode).

## Evidence and next step

No builds, tests, screenshots, reports, or release artifacts exist for this milestone. Next: begin only after the predecessor gate and user request.
