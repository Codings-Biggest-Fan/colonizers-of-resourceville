# Milestone 03 — Board and preview

Target release: **v0.0.3**. Status: Planned. This is an initial plan; refine against actual code and dependencies before implementation.

[Roadmap](../../ROADMAP.md) · [Progress](progress.md) · [Reports](reports/) · [Screenshots](../../photos/milestone-03/index.md)

## Goal and dependencies

Deliver a geometrically correct, interactive native board and a truthful local pregame preview.

Requires milestone 02 to pass its release gate. Reuse the existing rules/protocol contracts and relevant previously released systems; do not bypass unresolved earlier gates.

## Scope and implementation tasks

- [ ] Implement stable tile, vertex, and edge identifiers with topology-derived adjacency.
- [ ] Generate valid base boards with reproducible board-only seeds.
- [ ] Render original terrain, number tokens, ports, and sample pieces with a shared visual theme.
- [ ] Implement hit testing, hover/selection, zoom, pan, resize, and display-scaling support.
- [ ] Add a local preview showing only implemented board/configuration controls.

## Interfaces, state, persistence, and UI

Rules owns board topology and generation; desktop owns transforms, hit testing, visual tokens, and transient selection. A board seed must not determine future secret draws.

## Tests and acceptance checks

- [ ] Verify adjacent tiles share exactly the same vertices and edges.
- [ ] Check generated board inventories and constraints across seeded fixtures.
- [ ] Check transformed hit targets, resize behavior, and non-distorted rendering.

## Screenshot evidence

Capture default preview, resized viewport, zoom/pan, and representative Windows scaling. Record geometry and overlay inspection.

Record actual captures and metadata in the linked photo index. Explain nonvisual work and absent captures truthfully.

## Compatibility and migration

Keep board identifiers serializable and independent of viewport dimensions; version future generator changes that alter seeded layouts.

## Release gate and handoff

Topology and generation tests pass; screenshots demonstrate correct geometry and usable targets.

Update progress, append actual implementation reports, index inspected photos, update ROADMAP.md/CHANGELOG.md, and complete evidence review before marking Complete. Then summarize the release and stop for the next request. User-requested Polish mode pauses advancement.
## Branch, user testing, and GitHub release

Use a dedicated milestone-NN/description branch and PR against stable for this milestone. nightly follows latest pushed work; beta must match the active unapproved PR head. Leave the milestone Verifying and PR open for the user's testing/explicit approval. Do not merge, advance stable, publish the intended GitHub Release, or begin the next milestone before approval. After approval and passing gates, release the merged stable commit and link PR/release/wiki/evidence in the records. See [delivery workflow](../../docs/github-workflow.md).
