# v0.0.1 — Foundation (prepared, not published)

**The Colonizers of Resourceville.** M01 adds the native Rust foundation preview. Gameplay is planned for later milestones; this is not a playable game release.

## Changes

- Five-crate workspace: pure rules boundary, shared protocol boundary, bots boundary, egui/eframe + wgpu desktop, and separate Axum/Tokio server.
- Rust 1.90.0 pin and committed Cargo.lock; original procedural emblem/window icon.
- Responsive native preview with Close preview, readable theme, and opt-in application-frame evidence capture.
- Loopback process-liveness endpoint at /health; no game, account, or room routes.
- Windows/Linux/macOS x86-64 and ARM64 GitHub builds, tests, optimized binaries, checksum bundles, and Linux server container builds.
- PowerShell launch/check/test/capture/package helpers; updated building and player-guide sources.

## Validation and evidence

See [progress](progress.md), [numbered reports](reports/), [actual native images](../../photos/milestone-01/index.md), and [Building](../../docs/building.md). Clean Windows source at 066b440 passed formatting, Clippy, workspace tests, release build, package/checksum verification, and native launch/icon/resize/close inspection. Later script/package changes require their own actual CI evidence in the reports.

The [implementation PR #3](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/pull/3) targets beta. Review its current exact commit and final green six-target workflow before approval. Testing artifacts are unsigned. Each GitHub-built ZIP has a SHA256 sidecar and an internal file manifest; use the artifacts from the approved polished stable commit for publication. Local packages are development evidence only.

## Manual checks and limitations

Follow the prerequisites and commands in Building. Launch the native preview, check title/icon, resize at your display scaling, and close it. Run the server separately to check /health; it proves process liveness only. The desktop does not connect to this server yet.

No gameplay, bots, game contracts, saves, identity, multiplayer rooms, database, cloud project, signing, or nonlocal interactive runtime acceptance is included. No migrations are needed. The project license remains unselected; third-party dependency/font notices are packaged separately.

## Publication gate

This document is prepared release content, not approval. Obtain explicit implementation approval before beta, and separate explicit polish approval before stable. Only after the applicable gates promote stable/main and publish v0.0.1 as a prerelease using GitHub-built artifacts from that stable commit. Record actual PRs, approved commit, workflow/artifact identities, checksums, wiki, release URL, and limitations. No tag or release has been created.
