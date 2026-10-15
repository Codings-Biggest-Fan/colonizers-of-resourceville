# Current roadmap

Last updated: 2026-10-15 (America/Los_Angeles).

- **Current shipped release:** none.
- **Source repository:** [Codings-Biggest-Fan/colonizers-of-resourceville](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville), public; planning structure published on main.
- **Active milestone:** [00 — Initial setup](milestones/milestone-00/plan.md), Verifying; user testing and approval pending.
- **Review PR:** [#2 — README and documentation polish](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/pull/2). [Initial setup PR #1](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/pull/1) merged at 624223e before the later README updates. [Prepared release notes](milestones/milestone-00/release-notes.md); no new merge/release before approval.
- **Current phase:** setup documentation/GitHub workflow ready for review; native implementation remains M01.
- **Execution mode:** Polish mode active for original The Colonizers of Resourceville branding; roadmap advancement paused. [Rules](AGENTS.md#polish-mode).
- **Next task:** user tests M00's PR/beta. Wait for explicit approval before merge, release, or M01.
- **Verification:** planning structure/link checks passed; both helper scripts parse and the pending-completion gate correctly rejects M01. [Actual results](milestones/milestone-00/reports/002-structure-validation.md). No application build, runtime inspection, multiplayer verification, or cloud provisioning has occurred.

[Accepted implementation plan](docs/implementation-plan.md) · [Milestone conventions](milestones/README.md) · [Photo conventions](photos/README.md) · [Changelog](CHANGELOG.md)

## Releases and dependencies

Implement one milestone at a time. Dependencies below are sequential release gates; plans additionally identify subsystem prerequisites. Dates follow verified gates, not calendar promises.

| Milestone | Target release | Status | Depends on | Records | Acceptance gate |
| --- | --- | --- | --- | --- | --- |
| 00 | v0.0.0 — Initial setup | Verifying | None | [Plan](milestones/milestone-00/plan.md) · [Progress](milestones/milestone-00/progress.md) · [Reports](milestones/milestone-00/reports/) · [Photos](photos/milestone-00/index.md) | Records/wiki/helpers/CI validated; PR tested and explicitly approved before merge and setup-only release. |
| 01 | v0.0.1 — Foundation | Planned | M00 | [Plan](milestones/milestone-01/plan.md) · [Progress](milestones/milestone-01/progress.md) · [Reports](milestones/milestone-01/reports/) · [Photos](photos/milestone-01/index.md) | Clean checkout builds and launches; script entry points and documentation links validate; inspected native-window evidence is archived. |
| 02 | v0.0.2 — Rules and configuration contract | Planned | M01 | [Plan](milestones/milestone-02/plan.md) · [Progress](milestones/milestone-02/progress.md) · [Reports](milestones/milestone-02/reports/) · [Photos](photos/milestone-02/index.md) | Every observed setting has a disposition and release assignment; source-backed rules/scenario inventories and shared contracts are checked. |
| 03 | v0.0.3 — Board and preview | Planned | M02 | [Plan](milestones/milestone-03/plan.md) · [Progress](milestones/milestone-03/progress.md) · [Reports](milestones/milestone-03/reports/) · [Photos](photos/milestone-03/index.md) | Topology and generation tests pass; screenshots demonstrate correct geometry and usable targets. |
| 04 | v0.0.4 — Base turn engine | Planned | M03 | [Plan](milestones/milestone-04/plan.md) · [Progress](milestones/milestone-04/progress.md) · [Reports](milestones/milestone-04/reports/) · [Photos](photos/milestone-04/index.md) | Deterministic setup and ordinary-turn tests pass; rejected actions are atomic. |
| 05 | v0.0.5 — Complete base rules | Planned | M04 | [Plan](milestones/milestone-05/plan.md) · [Progress](milestones/milestone-05/progress.md) · [Reports](milestones/milestone-05/reports/) · [Photos](photos/milestone-05/index.md) | Base games finish correctly; mechanic and private-information edge cases have regression coverage. |
| 06 | v0.1.0 — Offline playable | Planned | M05 | [Plan](milestones/milestone-06/plan.md) · [Progress](milestones/milestone-06/progress.md) · [Reports](milestones/milestone-06/reports/) · [Photos](photos/milestone-06/index.md) | A person can complete and resume an offline game at every supported seat count. |
| 07 | v0.1.1 — Supabase foundation | Planned | M06 | [Plan](milestones/milestone-07/plan.md) · [Progress](milestones/milestone-07/progress.md) · [Reports](milestones/milestone-07/reports/) · [Photos](photos/milestone-07/index.md) | Authentication, migrations, persistence smoke checks, and permission tests pass. |
| 08 | v0.2.0 — Private online lobby | Planned | M07 | [Plan](milestones/milestone-08/plan.md) · [Progress](milestones/milestone-08/progress.md) · [Reports](milestones/milestone-08/reports/) · [Photos](photos/milestone-08/index.md) | Separate clients create/join the same room and consistently observe membership and configuration. |
| 09 | v0.3.0 — Reliable online games | Planned | M08 | [Plan](milestones/milestone-09/plan.md) · [Progress](milestones/milestone-09/progress.md) · [Reports](milestones/milestone-09/reports/) · [Photos](photos/milestone-09/index.md) | Online matches complete with consistent state and demonstrated retry/restart/failure recovery. |
| 10 | v0.4.0 — Base pregame completeness | Planned | M09 | [Plan](milestones/milestone-10/plan.md) · [Progress](milestones/milestone-10/progress.md) · [Reports](milestones/milestone-10/reports/) · [Photos](photos/milestone-10/index.md) | Every selectable base option affects gameplay correctly and persists across reload/reconnect. |
| 11 | v0.5.0 — Citadels & Sentinels | Planned | M10 | [Plan](milestones/milestone-11/plan.md) · [Progress](milestones/milestone-11/progress.md) · [Reports](milestones/milestone-11/reports/) · [Photos](photos/milestone-11/index.md) | Card/interaction inventory passes and complete offline/online matches are verified. |
| 12 | v0.6.0 — Open Seas | Planned | M11 | [Plan](milestones/milestone-12/plan.md) · [Progress](milestones/milestone-12/progress.md) · [Reports](milestones/milestone-12/reports/) · [Photos](photos/milestone-12/index.md) | Every shipped core scenario completes with correct setup, objectives, and scoring. |
| 13 | v0.7.0 — Combined expansions | Planned | M12 | [Plan](milestones/milestone-13/plan.md) · [Progress](milestones/milestone-13/progress.md) · [Reports](milestones/milestone-13/reports/) · [Photos](photos/milestone-13/index.md) | Supported combinations finish correctly; incompatible settings are clearly rejected. |
| 14 | v0.8.0 — Five to eight players | Planned | M13 | [Plan](milestones/milestone-14/plan.md) · [Progress](milestones/milestone-14/progress.md) · [Reports](milestones/milestone-14/reports/) · [Photos](photos/milestone-14/index.md) | Large games finish with correct permissions, privacy, recovery, and measured acceptable responsiveness. |
| 15 | v0.9.0 — Caravans & Raiders | Planned | M14 | [Plan](milestones/milestone-15/plan.md) · [Progress](milestones/milestone-15/progress.md) · [Reports](milestones/milestone-15/reports/) · [Photos](photos/milestone-15/index.md) | Each inventory entry has rules tests, bot support, inspected UI evidence, and completed games. |
| 16 | v0.10.0 — Voyagers & Corsairs | Planned | M15 | [Plan](milestones/milestone-16/plan.md) · [Progress](milestones/milestone-16/progress.md) · [Reports](milestones/milestone-16/reports/) · [Photos](photos/milestone-16/index.md) | Mission/cargo transitions pass and every shipped scenario completes correctly. |
| 17 | v0.11.0 — Maps and configuration completion | Planned | M16 | [Plan](milestones/milestone-17/plan.md) · [Progress](milestones/milestone-17/progress.md) · [Reports](milestones/milestone-17/reports/) · [Photos](photos/milestone-17/index.md) | All shipped maps validate and every pregame catalogue item has an explicit verified disposition. |
| 18 | v0.12.0 — Public casual platform | Planned | M17 | [Plan](milestones/milestone-18/plan.md) · [Progress](milestones/milestone-18/progress.md) · [Reports](milestones/milestone-18/reports/) · [Photos](photos/milestone-18/index.md) | Public/social flows work and visibility, identity, moderation, and spectator boundaries pass. |
| 19 | v1.0.0-rc.1 — Ranked and operations | Planned | M18 | [Plan](milestones/milestone-19/plan.md) · [Progress](milestones/milestone-19/progress.md) · [Reports](milestones/milestone-19/reports/) · [Photos](photos/milestone-19/index.md) | Ranked results apply once; restrictions, deployment, recovery, rollback, and load checks pass. |
| 20 | v1.0.0 — Release | Planned | M19 | [Plan](milestones/milestone-20/plan.md) · [Progress](milestones/milestone-20/progress.md) · [Reports](milestones/milestone-20/reports/) · [Photos](photos/milestone-20/index.md) | Clean-install acceptance passes; all release evidence is linked and the final coverage/limitations report is published. |

## Verified capabilities and current limitations

- The repository contains the roadmap, M00 records plus all 20 implementation plans/progress/photo indexes, report folders, and contributor instructions.
- Read-only prerequisite and record-validation helpers are provided; executed results are recorded in the report.
- No Cargo manifests, Rust application, CI workflow, native window, game rules, server container, or PNG evidence exists yet.
- The read-only prerequisite check reported rustc, cargo, rustup, and MSVC build tools missing/unverified. No installation was performed; native build prerequisites need to be established next.
- No Supabase organization/project has been created. New-organization access and actual cost confirmation are prerequisites at M07.
- Separate Rust hosting is required for online play; production deployment belongs to M19.

## Scope and operating rules

Windows-first native Rust; original assets/branding; offline plus online human/bot games; all four core expansion families and their documented compatible combinations. Initial 2–4 seats expand to 2–8. Public/ranked play is later scope. No in-game purchases, browser application, unverified parity claims, or silent infrastructure provisioning.

Status vocabulary: Planned → In progress → Verifying → Complete. Use Blocked only for a real dependency. Keep screenshots and reports linked; preserve history. Completion requires manual evidence review as well as automated checks.

## GitHub delivery channels

[Wiki](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/wiki) · [Building](docs/building.md) · [Game Instructions](docs/game-instructions/README.md) · [Workflow](docs/github-workflow.md)

Each milestone uses its own branch and PR against stable. nightly is the latest pushed milestone commit; beta matches the active unapproved PR head; stable is the latest approved/merged PR. main mirrors approved stable. Initial setup was merged through PR #1; subsequent polish is in PR #2. No milestone release exists. The user tests and approves before any new merge or release. M00 adds initial setup without renumbering M01–M20.
