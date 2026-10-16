# Milestone 01 progress — Foundation

Status: Verifying  
Target release: v0.0.1  
Last updated: 2026-10-15 (America/Los_Angeles)

[Plan](plan.md) · [Reports](reports/) · [Photos](../../photos/milestone-01/index.md) · [Roadmap](../../ROADMAP.md)

## Implemented work

- Five-crate Cargo workspace, Rust 1.90.0 pin, generated Cargo.lock.
- Original native egui/eframe + wgpu foundation preview; no future gameplay controls.
- Axum/Tokio liveness-only server, loopback default, explicit bind configuration.
- Dev/check/test/capture/package helpers and six-target GitHub build matrix.
- Building/development/workflow/game-guide updates and dependency notices in packages.
- User-authorized Rust/MSVC/SDK installation completed. No cloud provisioning.

## Verification

- [x] Initial documentation records/link check and helper parsing.
- [x] Prerequisite check after installation.
- [x] Debug workspace build after SDK installation.
- [x] Formatting/Clippy (including capture features), workspace tests (one substantive server test), and release build.
- [x] Clean-checkout checks/tests/release build at 328c7ab; final icon revision and clean native launch pending.
- [x] Native rendered-frame inspection, resize, Close preview, native close, and Windows 150% scaling; final icon/window pass pending.
- [x] Helpers exercised from outside repository; missing Cargo, duplicate capture, and invalid suite failures verified. Final clean package pending.
- [ ] GitHub six-target build/package jobs and Linux container checks.
- [ ] Testing PR and wiki preview published.
- [ ] Explicit implementation approval, then requested polish and explicit polish approval.
- [ ] Approved stable release using GitHub-built artifacts.

## Remaining work and dependencies

Complete actual verification and archive results. Keep beta/stable/main unchanged during unapproved M01 work. No games, bots, saves, identity, multiplayer, or database exist in this foundation. M02 remains planned.

## Blockers

None currently. User approvals are required after a concrete tested PR is ready, not before implementation.

## Polish checklist

M00 polish was resumed into roadmap work by the user's M01 request. No new polish flaws have been reported.

## Evidence

[Implementation and local verification](reports/001-native-foundation.md) records actual passes, failures, and corrections. [PR #3](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/pull/3) is a draft against beta. Six-target CI and final native/package evidence are pending.
