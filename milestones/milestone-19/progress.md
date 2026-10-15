# Milestone 19 progress — Ranked and operations

Status: Planned  
Target release: v1.0.0-rc.1  
Last updated: 2026-10-15

[Plan](plan.md) · [Reports](reports/) · [Photos](../../photos/milestone-19/index.md) · [Roadmap](../../ROADMAP.md)

## Landed work

No implementation work has started. This initial record establishes scope only.

## Implementation checklist

- [ ] Add separate ranked base 1v1 and four-player queues using server-owned fixed presets.
- [ ] Add ratings, seasons, leaderboards, abandonment policy, and exactly-once result processing.
- [ ] Exclude bots/custom configurations and define competitive disconnect/timeouts.
- [ ] Deploy the containerized Rust service separately from Supabase with TLS and approved actual costs.
- [ ] Add health/metrics, operational logs, backup/restore rehearsal, rollback, and load testing.

## Verification

- [ ] Test duplicate/abandoned results, queue assignment, rating invariants, and competitive permissions.
- [ ] Demonstrate restore, server restart, database failure, deployment rollback, and concurrent-room load.
- [ ] Verify only completed authorized ranked results alter ratings.

Unchecked items are pending, not failed or passed by inference. Structure-check results do not imply application build/runtime verification.

## Remaining work and dependencies

Await completion of milestone 18 and refinement of this plan against the then-current implementation.

## Blockers

No implementation blocker investigated yet; predecessor work remains planned.

## Polish checklist

No active Polish mode request or reported flaw checklist. See [Polish mode](../../AGENTS.md#polish-mode).

## Evidence and next step

No builds, tests, screenshots, reports, or release artifacts exist for this milestone. Next: begin only after the predecessor gate and user request.
