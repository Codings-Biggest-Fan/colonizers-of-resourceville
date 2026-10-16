# Milestone 21 — Website and documentation completion

Status: Planned. Target: final website/docs delivery after M20; no new game version assigned. Added by explicit user request on 2026-10-15.

[Roadmap](../../ROADMAP.md) · [Progress](progress.md) · [Reports](reports/) · [Photos](../../photos/milestone-21/index.md)

## Goal and dependencies

Create the product website and finish the public wiki/documentation around verified M20 capabilities and approved release artifacts. Requires M20 acceptance and approved source/release identities. Native gameplay remains Rust; the website is a separate download/documentation surface.

## Scope and tasks

- [ ] Inspect final product/release state and design a restrained original website with a clear download path, real product imagery and concise factual features.
- [ ] Provide truthful availability, supported platform/architecture choices, verified artifact links/checksums, and preview versus approved-release distinctions. No placeholder downloads or unsupported playable claims.
- [ ] Complete Building, installation/update, game setup/turns/trading, bots, saves/reconnect, expansion/scenario, accessibility/privacy, compatibility, troubleshooting and operating guides based on shipped behavior.
- [ ] Reconcile every milestone, source inventory, release note, limitation and evidence link; preserve historical records and source-to-wiki mappings.
- [ ] Generate and review final wiki pages and navigation from versioned sources; publish approved documentation only from approved polished stable.
- [ ] Verify website keyboard access, contrast, responsive layouts, images, downloads/checksums, broken links, and wiki navigation in rendered output.

## Branch and approval

Keep website implementation on a separate dedicated branch such as codex/download-page, with its own review. A milestone-21/documentation branch may own documentation-only changes; do not mix website code into a native milestone branch. This explicit website separation supersedes the normal single milestone source branch only for this separately reviewed surface. Preserve implementation/polish history within each source branch. PRs target beta for implementation review, then stable only after polish approval. Starting/finishing a page does not authorize deployment, domain purchases, releases, merges or publishing. Request explicit authorization for final deployment/publication after a concrete preview is reviewable.

## Interfaces, compatibility and migration

Website consumes public release metadata and static documentation, never game credentials/private state. No new gameplay/protocol/save format is planned. Use only approved artifact locations and original product labels. Native platform matrix is governed by verified artifacts and runtime limitations, not assumed from the website host. Versioned documentation should remain accessible for supported older formats.

## Tests and screenshot evidence

Run website checks appropriate to the chosen implementation, inspect desktop/mobile/narrow views and keyboard navigation, verify release URLs and checksums against real artifacts, validate documentation links and generated wiki navigation. Archive actual website captures as website evidence, clearly separate from native game screenshots. Get scoped computer/window permission before UI testing. Preserve M00–M20 evidence; do not generate mockups as runtime proof.

## Acceptance and handoff

Website and final wiki/docs accurately describe approved shipped capabilities, links/checksums work, accessibility/resizing checks and inspected evidence are archived, and required review/publication approvals are recorded. Update progress, append numbered reports, photo index, roadmap and changelog. Stop after M21; no additional milestone is authorized automatically. Documentation-only delivery does not automatically require a game release or new tag.
