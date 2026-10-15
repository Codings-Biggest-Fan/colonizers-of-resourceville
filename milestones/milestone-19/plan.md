# Milestone 19 — Ranked and operations

Target release: **v1.0.0-rc.1**. Status: Planned. This is an initial plan; refine against actual code and dependencies before implementation.

[Roadmap](../../ROADMAP.md) · [Progress](progress.md) · [Reports](reports/) · [Screenshots](../../photos/milestone-19/index.md)

## Goal and dependencies

Prepare competitive play and demonstrate production operation and recovery.

Requires milestone 18 to pass its release gate. Reuse the existing rules/protocol contracts and relevant previously released systems; do not bypass unresolved earlier gates.

## Scope and implementation tasks

- [ ] Add separate ranked base 1v1 and four-player queues using server-owned fixed presets.
- [ ] Add ratings, seasons, leaderboards, abandonment policy, and exactly-once result processing.
- [ ] Exclude bots/custom configurations and define competitive disconnect/timeouts.
- [ ] Deploy the containerized Rust service separately from Supabase with TLS and approved actual costs.
- [ ] Add health/metrics, operational logs, backup/restore rehearsal, rollback, and load testing.

## Interfaces, state, persistence, and UI

Competitive results and rating changes are transactionally idempotent. Ranked presets are independently versioned and cannot be edited by the lobby host.

## Tests and acceptance checks

- [ ] Test duplicate/abandoned results, queue assignment, rating invariants, and competitive permissions.
- [ ] Demonstrate restore, server restart, database failure, deployment rollback, and concurrent-room load.
- [ ] Verify only completed authorized ranked results alter ratings.

## Screenshot evidence

Capture ranked queue, fixed rules summary, leaderboard, disconnect policy feedback, and finished ranked result.

Record actual captures and metadata in the linked photo index. Explain nonvisual work and absent captures truthfully.

## Compatibility and migration

Keep ranked 1v1 rules distinct from the earlier casual two-player preset. External provisioning requires actual cost confirmation.

## Release gate and handoff

Ranked results apply once; restrictions, deployment, recovery, rollback, and load checks pass.

Update progress, append actual implementation reports, index inspected photos, update ROADMAP.md/CHANGELOG.md, and complete evidence review before marking Complete. Then summarize the release and stop for the next request. User-requested Polish mode pauses advancement.
## Branch, user testing, and GitHub release

Use a dedicated milestone-NN/description branch and PR against stable for this milestone. nightly follows latest pushed work; beta must match the active unapproved PR head. Leave the milestone Verifying and PR open for the user's testing/explicit approval. Do not merge, advance stable, publish the intended GitHub Release, or begin the next milestone before approval. After approval and passing gates, release the merged stable commit and link PR/release/wiki/evidence in the records. See [delivery workflow](../../docs/github-workflow.md).
