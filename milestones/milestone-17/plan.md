# Milestone 17 — Maps and configuration completion

Target release: **v0.11.0**. Status: Planned. This is an initial plan; refine against actual code and dependencies before implementation.

[Roadmap](../../ROADMAP.md) · [Progress](progress.md) · [Reports](reports/) · [Screenshots](../../photos/milestone-17/index.md)

## Goal and dependencies

Reconcile the full settings catalogue and finish map/preset authoring and supported combinations.

Requires milestone 16 to pass its release gate. Reuse the existing rules/protocol contracts and relevant previously released systems; do not bypass unresolved earlier gates.

## Scope and implementation tasks

- [ ] Finish remaining officially documented compatible combinations from the pinned inventory.
- [ ] Implement an independently defined accelerated/Rush-style preset.
- [ ] Add a board editor with map import/export and strict topology/setup/objective validation.
- [ ] Add saved/shared custom presets and original map equivalents.
- [ ] Reconcile every pregame catalogue row: implemented evidence or explicit exclusion reason.

## Interfaces, state, persistence, and UI

Version map and preset formats. Import untrusted data through bounded validation; importing a file must never execute code or expose secrets.

## Tests and acceptance checks

- [ ] Reject malformed, oversized, disconnected, or impossible maps and incompatible presets.
- [ ] Round-trip maps/presets and test reproducible generation.
- [ ] Check each catalogue row has behavior evidence and each supported combination completes.

## Screenshot evidence

Capture editor, map validation failures, shared-preset import, compatibility errors, and completed custom-map games.

Record actual captures and metadata in the linked photo index. Explain nonvisual work and absent captures truthfully.

## Compatibility and migration

Promotional scenarios, historical editions, standalone games, and arbitrary unsupported expansion mixtures are outside the agreed roadmap.

## Release gate and handoff

All shipped maps validate and every pregame catalogue item has an explicit verified disposition.

Update progress, append actual implementation reports, index inspected photos, update ROADMAP.md/CHANGELOG.md, and complete evidence review before marking Complete. Then summarize the release and stop for the next request. User-requested Polish mode pauses advancement.
