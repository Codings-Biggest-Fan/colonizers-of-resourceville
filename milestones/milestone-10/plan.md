# Milestone 10 — Base pregame completeness

Target release: **v0.4.0**. Status: Planned. This is an initial plan; refine against actual code and dependencies before implementation.

[Roadmap](../../ROADMAP.md) · [Progress](progress.md) · [Reports](reports/) · [Screenshots](../../photos/milestone-10/index.md)

## Goal and dependencies

Make all planned base-game configuration controls functional and verifiable.

Requires milestone 09 to pass its release gate. Reuse the existing rules/protocol contracts and relevant previously released systems; do not bypass unresolved earlier gates.

## Scope and implementation tasks

- [ ] Implement standard/custom presets, victory targets, dice policy, robber protection, and discard thresholds.
- [ ] Implement player trade policy, offer expiry, turn order, timers, and legal timeout decisions.
- [ ] Implement board terrain/number/port randomization and validated fixed/seeded layouts.
- [ ] Implement room passwords and configurable disconnect/pause/substitution behavior.
- [ ] Persist and synchronize presets/settings; display custom-rule markers and conflicts.

## Interfaces, state, persistence, and UI

Extend GameConfig only through versioned, validated fields. Balanced dice is a documented independent algorithm rather than an unverified claim of reference equivalence.

## Tests and acceptance checks

- [ ] Map each base-applicable catalogue row to a behavior test.
- [ ] Round-trip presets and settings through persistence and reconnect.
- [ ] Exercise deadline expiration during placement, trades, discards, and other pending choices.

## Screenshot evidence

Capture settings groups, map preview, custom-rule summary, incompatible settings, and timed-response feedback.

Record actual captures and metadata in the linked photo index. Explain nonvisual work and absent captures truthfully.

## Compatibility and migration

Keep ranked policy out of user-controlled settings; expansion controls remain unavailable until implemented.

## Release gate and handoff

Every selectable base option affects gameplay correctly and persists across reload/reconnect.

Update progress, append actual implementation reports, index inspected photos, update ROADMAP.md/CHANGELOG.md, and complete evidence review before marking Complete. Then summarize the release and stop for the next request. User-requested Polish mode pauses advancement.
