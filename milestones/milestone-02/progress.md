# Milestone 02 progress — Rules and configuration contract

Status: Planned  
Target release: v0.0.2  
Last updated: 2026-10-15

[Plan](plan.md) · [Reports](reports/) · [Photos](../../photos/milestone-02/index.md) · [Roadmap](../../ROADMAP.md)

## Landed work

No implementation work has started. This initial record establishes scope only.

## Implementation checklist

- [ ] Audit reference pregame controls through ordinary public UI and document observed values, defaults, unavailable options, source dates, and uncertainties.
- [ ] Pin public core rulebooks and enumerate every development/progress card, core scenario, and supported combination.
- [ ] Define GameConfig, ruleset/scenario identifiers, capabilities, command/event envelopes, recipient views, and save versioning.
- [ ] Specify defaults, custom markers, readiness invalidation, immutable in-game configuration, and invalid-combination explanations.
- [ ] Assign every catalogue item to a milestone and define deterministic fixtures and private-information boundaries.

## Verification

- [ ] Round-trip shared configuration and reject invalid or incompatible configurations.
- [ ] Confirm settings edits clear readiness and starting freezes the accepted configuration contract.
- [ ] Check inventory completeness against pinned sources and give every observed setting a documented disposition.

Unchecked items are pending, not failed or passed by inference. Structure-check results do not imply application build/runtime verification.

## Remaining work and dependencies

Await completion of milestone 01 and refinement of this plan against the then-current implementation.

## Blockers

No implementation blocker investigated yet; predecessor work remains planned.

## Polish checklist

No active Polish mode request or reported flaw checklist. See [Polish mode](../../AGENTS.md#polish-mode).

## Evidence and next step

No builds, tests, screenshots, reports, or release artifacts exist for this milestone. Next: begin only after the predecessor gate and user request.
