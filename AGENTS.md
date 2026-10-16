# AGENTS.md — The Colonizers of Resourceville

Read this file first, then README.md, ROADMAP.md, and the active milestone's plan and progress. Follow more specific nested instructions when present.

## Product and scope

### Branding and active polish

Product name: **The Colonizers of Resourceville**. Machine/repository name: **colonizers-of-resourceville**. Use original visuals, labels, copy, and marks everywhere: app/window titles, README/wiki, screenshots, package names, release notes, repository metadata, and social previews. Do not reuse prior working names or competitor product/expansion branding. Expansion names are Citadels & Sentinels, Open Seas, Caravans & Raiders, and Voyagers & Corsairs. Preserve gameplay scope while using original presentation. This is a working project identity, not a claim of trademark clearance.

The user selected this exact title during branding polish. Preserve it in future work; do not substitute a newly invented product name without a user request.

The user resumed roadmap work by explicitly requesting completion of M01 on 2026-10-15. M00 branding polish remains historical; preserve its identity and evidence. Implement only M01 until the next request. Earlier setup report copies were sanitized once under the user's explicit branding correction; original historical text remains in Git history and their actual results/commit IDs are unchanged.

Build an independently implemented Windows-first native Rust hex-board strategy game called The Colonizers of Resourceville. Use original code, artwork, sounds, and branding. Do not copy proprietary source, extracted assets, private APIs, or claim affiliation. A project license has not been selected.

The agreed scope is offline solo with bots, private online rooms, later public casual/ranked play, and 2–4 seats growing to 2–8. Include base rules, Citadels & Sentinels, Open Seas, supported combinations, Caravans & Raiders, and Voyagers & Corsairs with their pinned core scenarios. Standalone games, promotional scenarios, historical editions, and arbitrary expansion mixtures are outside this roadmap. All gameplay modes are available without an in-game purchase system.

The stack is native Rust with egui/eframe and wgpu; a separate Rust Axum/Tokio server; shared Rust protocol types; Supabase Auth/Postgres accessed by the server through SQLx. No browser UI, webview shell, JavaScript/TypeScript application, or Tauri/Electron packaging. The current folder may contain planning structure before the Cargo workspace exists: inspect actual files and never invent runnable commands.

### Product experience and design rules

The main user job is to play this game with friends with almost zero setup. Prioritize a short, understandable create/join/invite flow, useful defaults, and clear recovery over technical configuration or promotional content. Implement these flows in their owning milestones; never add pretend playable controls to an earlier foundation release.

The first viewport must immediately communicate a familiar hex-board resource-trading and settlement-building game through original board, road, settlement, and resource visuals. It should impress through visual craft and a clear path to the user's task. Do not use competitor names, logos, extracted assets, or copied presentation in the product. Keep implemented capability and planned capability distinguishable.

- **Theme:** use Catppuccin color schemes with a user-selectable accent color. Support Latte, Frappé, Macchiato, and Mocha when appearance settings are implemented. Use shared semantic tokens for backgrounds, text, surfaces, focus, selection, and feedback; preserve readability across every flavor/accent combination. Follow the [official Catppuccin palette/style guidance](https://github.com/catppuccin/catppuccin/blob/main/docs/style-guide.md). Color alone must not carry meaning.
- **Settings:** use a native settings layout informed by VS Code: searchable settings, a category sidebar, clear section headings, and labeled setting rows with concise explanations, appropriate controls, and reset/default behavior. Adapt it to native Rust; do not embed or copy VS Code. Every exposed setting must work and persist appropriately. Keep application appearance preferences separate from authoritative gameplay configuration.
- **States:** design applicable default, hover, keyboard focus, pressed, selected, disabled, loading, empty, error, recovery, and success states. Keep feedback close to the affected action, explain actionable failures, and preserve the user's input. Do not fabricate loading or success for absent features.
- **No decorative tags:** do not add ornamental badges, pills, eyebrow tags, repetitive status chips, or developer/framework labels as interface decoration. Required game/room/error state belongs in clear, useful text or controls; it is not an excuse for tag clutter.
- **Copy:** functional game, lobby, and settings screens must use concise task-focused language. Ban exaggerated advertising and repeated feature pitches there. The main screen may contain a restrained, factual feature list, while keeping play with friends as the priority. Avoid making even the main screen overly promotional.
- **Privacy in the same room:** account for opponents looking at nearby screens. Keep hands, private points, pending secret choices, and other hidden information out of public views, notifications, logs, screenshots, and shared-screen presentations. Evaluate intentional reveal, a quick privacy cover, and hiding private content during loss of focus or handoff where appropriate. Test privacy with multiple people/screens and synthetic identities; server-side payload privacy alone does not prevent shoulder surfing. Do not invent an unsupported shared-device game mode.
- **Impeccable:** apply the relevant [official Impeccable guidance](https://github.com/pbakaus/impeccable/blob/main/.agent/skills/impeccable/SKILL.md) to design work: inspect real product context first, choose a clear hierarchy, use consistent native controls and spacing, remove generic template clutter, and verify contrast, keyboard access, resizing, and actual rendered output. Treat game/settings surfaces as tasks, documentation as reading, and the download page as a modest promotional surface. Respect the user's pinned Catppuccin direction. Inspect in bounded batches, fix observed issues together, and avoid open-ended cosmetic loops. Do not install its runtime silently or change the native stack to satisfy web-oriented tooling.

These are standing design requirements, not a claim that themes, settings, gameplay, friend rooms, or privacy controls already exist. The user explicitly requested these rules for AGENTS.md on 2026-10-15; this request alone does not expand M01 into implementing those later features.

### Download page and README presentation

Keep download-page work on a separate dedicated branch, such as codex/download-page, rather than mixing its website implementation into a native milestone branch. A standalone download page may be somewhat promotional and show original product imagery and factual features. Preserve truthful availability, platform/architecture choices, verified artifact links/checksums, and the distinction between testing previews and approved releases. Do not invent download URLs, claim unfinished gameplay is playable, or turn the native game into a website/webview. Creating the branch or page does not authorize deployment or release publication.

In README.md, place the current milestone's inspected main native screenshot directly below the product title and above the description. The Screenshots section should show about five real, inspected images in a compact grid, with links to capture metadata and the full evidence archive. Keep the milestone/build identity honest; never manufacture screenshots to fill the grid. The user authorized this README presentation change through direct commits without a separate PR; that documentation permission does not approve M01 implementation, polish, merges, or releases.

## One milestone at a time

1. ROADMAP.md is the current source of truth; docs/implementation-plan.md preserves the accepted overall plan.
2. Implement only the active milestone unless the user explicitly changes the scope. Milestones can contain many implementation tasks.
3. Read and refine milestones/milestone-NN/plan.md against the actual code before implementation.
4. Keep progress.md current and append a numbered report under that milestone's reports/ after each implementation session.
5. Update ROADMAP.md and CHANGELOG.md when capabilities, blockers, or release status change.
6. Run the release gate and archive evidence before marking a milestone Complete.
7. After a milestone passes, provide its release summary, evidence, limitations, and next milestone; stop for the next request.

Statuses are Planned, In progress, Verifying, Complete, and Blocked. Blocked means a real dependency prevents progress. Do not use estimated percentages or describe planned features as delivered. Keep dates in the user's America/Los_Angeles time context.

## Milestone 0 and GitHub delivery

Milestone 00 is Initial setup: structure, documentation/wiki, contributor rules, helpers, and GitHub workflow. It precedes implementation milestones 01–20 without renumbering them. Native Rust scaffolding stays M01.

### Channel meanings — user clarification

- **nightly** is the current development commit, including unapproved and unfinished changes. Update it with the latest pushed work from the active milestone or polish branch.
- **beta** is the latest user-approved PR before polish. It is an approved implementation baseline, not the unapproved PR/testing branch. Never move beta merely because a development commit or new PR exists.
- **stable** contains user-approved, polished PRs. Both approval of the implementation and approval of its polish are required before promotion to stable. It is the default branch.
- **main** mirrors approved, polished stable for compatibility; advance it only when stable is promoted or for an explicitly authorized rules-only update.
- Dedicated milestone-NN/description and polish branches hold work under review. They may match nightly; they must not automatically update beta or stable.

These meanings supersede older text elsewhere that calls beta an unapproved PR head or requires milestone/nightly/beta to move together. Update older supporting documents when they are next in scope. A rules-only documentation commit does not approve or promote the implementation already on a branch.

### Approval and promotion

1. Implement on the dedicated milestone branch and push current work to nightly. Open/attach the implementation PR targeting beta when it is ready for user testing; include the exact commit, changes, actual checks, manual steps, limitations, wiki, and evidence.
2. Stop for explicit user approval. Only after approval merge the tested implementation into beta; beta is then the approved, unpolished baseline.
3. Perform requested polish on the milestone or a dedicated polish branch, updating nightly with each pushed revision. Keep beta at its approved baseline while polish is unapproved.
4. Open/attach the polished PR targeting stable. Stop for the user's explicit approval of that polished revision. Approval of the implementation alone is not polish approval.
5. Only after polished approval and applicable gates merge into stable, fast-forward main to stable, and publish the milestone GitHub Release from that stable commit. A request to create a PR, finish work, or update nightly is not approval.
6. Existing PRs remain unmerged unless the user approves them; this channel clarification does not authorize merging, retargeting, closing, or releasing them automatically.

Do not force-push, rewrite published history, move refs backward, or promote unrelated work. Inspect divergence and preserve branch-specific changes. In the normal workflow, unapproved corrections stay on the development/polish branch and nightly. Stop after the milestone and its approved release; do not begin the next milestone without the next request.

### Releases and rules-only exception

M00 targets source/docs-only v0.0.0; M01–M20 retain roadmap versions. Mark pre-v1.0.0 releases as prereleases. Release only approved, polished stable commits. Include PRs/commit, changes, actual validation, manual testing, wiki/build/game instructions, evidence, verified artifacts/checksums when produced, migrations, and limitations. Never tag unfinished work or overwrite published tags/releases.

The user explicitly authorized this channel-policy correction to AGENTS.md directly in every existing source branch without a PR. Apply the same updated file while preserving each branch's other content and history. This is a one-time exception for this rules change, not standing permission to bypass implementation or polish approval.

## Wiki and building/game instructions

The GitHub Wiki hosts public documentation. Keep reviewed sources in docs/, milestones/, photos/, ROADMAP.md, and AGENTS.md; wiki repositories lack normal source PR review. Link the wiki, Building, and Game Instructions from README.md. Generate pages with scripts/wiki.ps1 and verify published navigation.

Development and approved-but-unpolished beta wiki previews must prominently say TESTING PREVIEW, identify their channel/source commit and PR when available, and never look like approved, polished release docs. Publish approved release documentation only from polished stable after explicit approval. Maintain source-to-wiki link mappings and pinned source/evidence links.

Maintain docs/building.md: prerequisites, clone/channel selection, commands that work now, guarded future build/run/package instructions, troubleshooting, and clean-checkout verification. Describe the actual workspace availability for the documented revision. Maintain docs/game-instructions/ as the player-guide source folder, published as a grouped Game Instructions wiki section. Cover setup, turns/trading, bots, saves/reconnect, variants, and troubleshooting as implemented; distinguish plans from available controls.

## Permanent project records

- ROADMAP.md includes current release, active milestone, last update, dependencies, M00 plus all 20 implementation releases, status, acceptance summary, plans, progress, reports, photos, PRs, and published releases.
- Every milestone has plan.md, progress.md, reports/, and a matching photos/milestone-NN/index.md.
- Plans record goal, release, dependencies, scope, tasks, interface/data changes, tests, screenshots, compatibility, and migration considerations.
- Progress records distinguish landed work, pending checks, remaining work, blockers, and evidence.
- Reports record actual changes, commands/results, decisions, limitations, screenshots, and exact next steps.
- Number reports sequentially (001-description.md). Preserve historical reports; record corrections in a subsequent report instead of silently rewriting evidence.
- Do not create completed reports, passing results, release tags, or screenshot claims before that work occurs.
- Use relative Markdown links inside project documentation; use absolute file links in chat responses.

## Screenshots and visual verification

Store actual native application screenshots in photos/milestone-NN/ using sequential descriptive filenames, such as 001-native-window.png. Keep evidence separate from assets/.

Each photo index records image link, capture date, build/revision when available, window/image dimensions, display scaling, scenario, and the inspection result. Record unavailable metadata honestly.

Inspect rendered output. Test geometry, overlays, input targets, resizing, and Windows scaling when affected. Include before/after images for material visual corrections and paired client views for multiplayer changes. A passing build is not visual or multiplayer evidence.

Capture only the selected game window or its own rendered frame, using synthetic identities. Do not capture unrelated windows, credentials, or personal information. Preserve prior milestone evidence. If a change has no UI impact, explain why no screenshot is applicable; never substitute a generated mockup for runtime evidence.

## Polish mode

The user may say "polish mode" and identify flaws in screenshots, layout, copy, interaction, performance, or game behavior.

- Explicitly entering Polish mode pauses milestone advancement. Stay in this mode until the user asks to resume the roadmap or otherwise explicitly changes direction.
- Treat the user's reported flaws as the active task. Acknowledge the concrete targets, inspect the affected files and current running behavior, and fix the underlying cause within that scope.
- If a flaw is clear, act without asking the user to repeat it or approve a routine reversible fix. Ask a focused question only when an unresolved ambiguity materially changes the fix.
- Preserve working behavior, unrelated user changes, product constraints, and current information architecture unless the requested correction requires changing them.
- Do not use polishing as permission to add unrelated features, change the stack, skip milestones, or claim a later release.
- Track a concise flaw checklist in the active milestone's progress.md. Associate corrections to previously released work with its owning milestone and cross-link the current report when needed.
- Capture a before image when possible and an inspected after image for visual fixes. If no before image can be obtained, state that. For nonvisual fixes, record meaningful behavioral/test evidence.
- Run checks appropriate to the change; add regression tests for rules, persistence, privacy, networking, or substantive behavior fixes. Do not add implementation-mirroring tests for trivial copy/style changes.
- Append a numbered NNN-polish-description.md report containing the flaw, cause, fix, validation, screenshots, and remaining issues. Update the photo index and relevant roadmap/changelog notes.
- Report the fixes and any unresolved flaws; remain available for the user's next corrections. Completing polish does not automatically complete or advance a milestone.

A specific bug correction without an explicit request to enter Polish mode is still handled as a scoped fix, but does not create an indefinite mode change. User corrections steer the active work immediately.

## Architecture and game correctness

Keep rules pure and deterministic: no UI, filesystem, database, network, async runtime, or hidden dependency on global randomness. Separate authoritative state, permitted player observations, protocol, bots, UI presentation, and persistence.

Use stable board topology identifiers; never derive shared intersections from screen coordinates. Use integer inventories, explicit phases, complete validation, and atomic rejected actions. Use the same command pipeline for humans and bots. Bots only receive their own permitted observations and legal choices.

Pin public rules editions and document interpretations in docs/rules.md. Maintain card/scenario inventories and docs/compatibility.md. Official two-player, casual two-seat, official 5–6-player, and custom 7–8-player modes are distinct.

One versioned GameConfig drives lobby, server validation, persistence, and presets. Gameplay edits clear readiness; starting freezes configuration. Every selectable control must work. Reject unsupported combinations with useful reasons. Board seeds reproduce layouts only; do not disclose future randomness.

Online state is server-authoritative. Authenticate membership and validate commands; serialize per-game writes; deduplicate retries; persist accepted state and receipts atomically before acknowledgment. Protect hidden hands, private points, decks, exploration, and random outcomes in client payloads, spectators, replays, logs, and errors. Restore pending choices, offers, deadlines, and bot state after recovery.

## Supabase and external services

Use a new organization and project separate from Scout App. Do not reuse or alter scouting resources. Confirm connected organization access and actual costs before provisioning. Supabase provides Auth/Postgres, not hosting for the separate Rust service.

Use the Supabase skill and verify current official documentation when doing Supabase work. Never expose database credentials or secret/service-role keys in desktop builds. Apply explicit grants, private schemas, restricted server roles, and RLS where applicable. Verify migrations and access boundaries with actual tests. Keep offline play independent of cloud availability.

## Helper scripts and validation

scripts/ contains PowerShell helpers with documented arguments, repository-relative resolution via PSScriptRoot, actionable errors, and nonzero failure exit codes. Scripts must work when invoked from another directory.

Never silently install tools, provision services, delete files, or capture unrelated windows. Never put credentials in scripts or tracked files. Do not create success-reporting stubs for unimplemented commands.

Release builds must run on GitHub Actions for Windows, Linux, and macOS, covering x86-64 and ARM64 on each OS. Include Windows on ARM64 as well as x86-64. These are nonlocal build/package checks; local testing remains Windows-first. Distinguish successful builds from actual runtime verification on each platform, and record unsupported targets or unavailable runners honestly. Publish release artifacts only after the existing implementation and polish approvals. Do not reduce the release matrix to the local machine architecture.

Use the actual workspace's formatting, Clippy, tests, and release commands once Cargo exists. Match tests to the change. Complete release gates before marking Complete; a link/file checker verifies record structure, not the truth of test or screenshot claims.

Keep local saves/maps/presets/protocols explicitly versioned, validate untrusted input, bound work and allocations, return useful errors, and preserve supported formats with tested migrations or clear incompatibility errors. Attribute third-party assets and dependencies accurately.

## Working practices

Preserve unrelated edits. Re-read shared files before editing. Use focused changes and existing conventions. Do not delegate unless the user or applicable instructions explicitly authorize it. Never claim a build, test, screenshot inspection, network flow, or release succeeded without evidence.

Ask the user before accessing or interacting with their computer/window UI, including native visual testing. Keep permission scoped to the described check. A physical Escape stop ends UI input immediately; ask again before a later access session. Non-UI source inspection, shell builds/tests, and GitHub checks can continue independently.

Current work may be documentation-only at the user's request. Finish that scope first; do not install tools or begin native implementation merely to make planning records look complete.


