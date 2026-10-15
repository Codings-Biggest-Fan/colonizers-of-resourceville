# Milestone 06 — Offline playable

Target release: **v0.1.0**. Status: Planned. This is an initial plan; refine against actual code and dependencies before implementation.

[Roadmap](../../ROADMAP.md) · [Progress](progress.md) · [Reports](reports/) · [Screenshots](../../photos/milestone-06/index.md)

## Goal and dependencies

Release complete offline solo games against bots for 2–4 seats.

Requires milestone 05 to pass its release gate. Reuse the existing rules/protocol contracts and relevant previously released systems; do not bypass unresolved earlier gates.

## Scope and implementation tasks

- [ ] Add baseline bots that consume only their permitted observations and legal choices.
- [ ] Implement host-selected 2–4 seats with human/bot choices and one local human for solo games.
- [ ] Add offline autosave after accepted actions, resume, restart, and endgame navigation.
- [ ] Use atomic file replacement and validated versioned save loading.
- [ ] Run seeded bot matches and address illegal-action loops or decisions that cannot progress.

## Interfaces, state, persistence, and UI

Local execution uses the same command/observation boundary intended for the server. Bot memory and pending choices required for resume belong in the save contract.

## Tests and acceptance checks

- [ ] Complete games at each supported seat count.
- [ ] Test save, close, reopen, resume, corrupt saves, and unsupported save versions.
- [ ] Check bot observations exclude opponents' secrets and future randomness; test bounded decision loops.

## Screenshot evidence

Capture offline setup, a mixed human/bot game, resume flow, save-load failure, and results.

Record actual captures and metadata in the linked photo index. Explain nonvisual work and absent captures truthfully.

## Compatibility and migration

Publish the first supported offline save version and a documented future migration policy.

## Release gate and handoff

A person can complete and resume an offline game at every supported seat count.

Update progress, append actual implementation reports, index inspected photos, update ROADMAP.md/CHANGELOG.md, and complete evidence review before marking Complete. Then summarize the release and stop for the next request. User-requested Polish mode pauses advancement.
