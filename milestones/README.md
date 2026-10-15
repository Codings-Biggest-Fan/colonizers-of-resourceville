# Milestone records

[Current roadmap](../ROADMAP.md) · [Overall plan](../docs/implementation-plan.md) · [Screenshot archive](../photos/README.md)

Each milestone has plan.md, progress.md, and reports/. Initial plans are scoped implementation handoffs, not claims that later designs are already verified. Refine the active plan against code and pinned dependencies before implementation.

## Plans

Record goal/release, dependencies, scope, tasks, rules/interface/persistence/UI changes, tests, screenshots, release gate, and compatibility/migrations.

## Progress

Use Planned, In progress, Verifying, Complete, or Blocked. Keep actual landed work, unrun checks, remaining tasks, blockers, and evidence distinct. No estimated percentages.

## Reports

Append sequential reports (001-description.md, 002-description.md). A report records actual changes, commands/results, screenshots, decisions, limitations, and exact next steps. Never fabricate implementation or passing tests. Correct historical reports in a new entry.

Polish reports use NNN-polish-description.md and record user-reported flaws, causes, fixes, before/after evidence, checks, and remaining issues. See [Polish mode](../AGENTS.md#polish-mode).

Complete one milestone and its release gate, then stop for the next user request. A structural checker cannot certify runtime behavior.
