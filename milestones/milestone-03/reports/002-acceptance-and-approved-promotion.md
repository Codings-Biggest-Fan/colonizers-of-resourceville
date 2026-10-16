# 002 — Acceptance gates and approved promotion

Date: 2026-10-15 (America/Los_Angeles). Implementation source: 4ccbf26212a20081e096c5f11ca689b6f1c43f4a. The user explicitly approved implementation and visual polish for beta/stable and requested roadmap updates and PRs. [PR #9](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/pull/9) targets beta and [PR #10](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/pull/10) targets stable, both from milestone-03/board-preview.

## Verified gates

Local formatting, all-target/all-feature Clippy, 20 workspace tests and optimized workspace build passed. Tests cover shared topology, 2,051 seeded layouts, fixed generator fixture, graph/port inventories, camera/hit transforms, atomic seed rejection and egui right-drag/wheel events. Inspected native frames 017/018 show the final default/compact view; earlier wide/scaling evidence and the visual correction history remain in the [photo index](../../../photos/milestone-03/index.md). The user accepted the final visual revision.

[Native CI run 38856644491](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/actions/runs/38856644491) passed all six Windows/Linux/macOS x86-64/ARM64 build/test/package jobs, including both Linux server containers. Each runner checks out the exact source, formats/lints/tests/builds/packages it and verifies host architecture. [Archived job results](evidence/ci-jobs-4ccbf26.json).

Downloaded every actual artifact. Verified outer GitHub digest, nested ZIP CRC/checksum, all 649 file hashes per platform (3,894 total), source identity, both binary architecture headers and Unix executable permissions. Both Linux Docker archives matched sidecar checksums and amd64/arm64 configuration. [Artifact evidence](evidence/ci-artifacts-4ccbf26.json). Local clean-source Windows package also passed CRC, SHA-256 and all 649 file hashes; archive SHA-256 is 43a42ed60822381f4adda2c8823392dff9a06e0fed0fc027f2f104752ebcd28f.

Generated/published TESTING PREVIEW wiki at 164c187, with source/channel/PR markers. Home, Building, Game Instructions and Board preview returned HTTP 200 with preview markers; [HTTP evidence](evidence/wiki-preview-http.json). Approved documentation can replace it only after the approved stable promotion. Record/link checks are run again for the final documentation commit.

## Acceptance and limits

M03 acceptance uses inspected real native frames, deterministic/headless egui interaction tests, all-target builds/artifact checks and explicit user implementation/polish approval. Earlier pending wide/scaling/native camera checklist is reconciled against that evidence, not reported as a human input check. A separate human native right-drag/context-menu session and a final zoomed/panned screenshot were not completed after Computer Use Escape stops. Non-Windows interactive runtime, playable turns, bots, saves, rooms, live trading and cloud remain unverified or unimplemented as documented. These limitations are retained, not inferred from passing CI.

Only documentation changes follow 4ccbf26: acceptance records, roadmap/current guides and evidence. No rule/renderer/artwork changes. Promote the approved source and records through PRs #9/#10, mirror stable to main, then stop. No release/tag before M06; M04 remains Planned until a new request.
