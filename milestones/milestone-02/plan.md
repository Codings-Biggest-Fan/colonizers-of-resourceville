# Milestone 02 — Rules and configuration contract

Target release: **v0.0.2**. Status: Planned. This is an initial plan; refine against actual code and dependencies before implementation.

[Roadmap](../../ROADMAP.md) · [Progress](progress.md) · [Reports](reports/) · [Screenshots](../../photos/milestone-02/index.md)

## Goal and dependencies

Turn the complete pregame and expansion scope into a source-backed, versioned implementation contract.

Requires milestone 01 to pass its release gate. Reuse the existing rules/protocol contracts and relevant previously released systems; do not bypass unresolved earlier gates.

## Scope and implementation tasks

- [ ] Audit reference pregame controls through ordinary public UI and document observed values, defaults, unavailable options, source dates, and uncertainties.
- [ ] Pin public core rulebooks and enumerate every development/progress card, core scenario, and supported combination.
- [ ] Define GameConfig, ruleset/scenario identifiers, capabilities, command/event envelopes, recipient views, and save versioning.
- [ ] Specify defaults, custom markers, readiness invalidation, immutable in-game configuration, and invalid-combination explanations.
- [ ] Assign every catalogue item to a milestone and define deterministic fixtures and private-information boundaries.

## Interfaces, state, persistence, and UI

Shared Rust serialization types belong in protocol; pure rule definitions belong in rules. Keep observable state distinct from authoritative secrets. Unknown reference behavior remains explicitly unresolved, not a guessed parity claim.

## Tests and acceptance checks

- [ ] Round-trip shared configuration and reject invalid or incompatible configurations.
- [ ] Confirm settings edits clear readiness and starting freezes the accepted configuration contract.
- [ ] Check inventory completeness against pinned sources and give every observed setting a documented disposition.

## Screenshot evidence

Archive any actual native configuration-contract UI introduced. If only types and documentation change, record that no application UI changed; reference research is not native implementation evidence.

Record actual captures and metadata in the linked photo index. Explain nonvisual work and absent captures truthfully.

## Compatibility and migration

Introduce explicit initial schema/protocol versions. Document differences between casual two-seat play, official two-player rules, official 5–6-player rules, and custom 7–8-player rules.

## Release gate and handoff

Every observed setting has a disposition and release assignment; source-backed rules/scenario inventories and shared contracts are checked.

Update progress, append actual implementation reports, index inspected photos, update ROADMAP.md/CHANGELOG.md, and complete evidence review before marking Complete. Then summarize the release and stop for the next request. User-requested Polish mode pauses advancement.
