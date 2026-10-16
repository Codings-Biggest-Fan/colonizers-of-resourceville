# Milestone 01 progress — Foundation

Status: Complete  
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
- [x] Clean-checkout checks/tests/release build at 066b440; clean optimized native launch/icon/resize/keyboard-close inspection also passed with renewed user authorization.
- [x] Native rendered-frame inspection, resize, Close preview, native close, and Windows 150% scaling; final clean-release icon/window pass completed.
- [x] Helpers exercised from outside repository; missing Cargo, duplicate capture, and invalid suite failures verified. Clean package passed, including archive and 649 extracted file checksums.
- [x] GitHub six-target build/package jobs and both Linux image exports passed at c2459eb; actual ZIP hashes/files, architecture headers, Unix executable bits, and container configuration independently verified.
- [x] Draft PR #3 and labeled TESTING PREVIEW wiki published; Home/Building/Game Instructions HTTP and source identity verified.
- [x] User handoff confirms implementation and polish PRs #3/#4 merged.
- [x] Publication gate corrected: no releases/tags before M06; not required for M01.

## Remaining work and dependencies

M01 acceptance passed and user merges are complete; no release is due before M06. [Reconciliation](reports/007-m02-handoff-reconciliation.md) supersedes stale pending-review prose below while preserving historical evidence. The user requested M02; no gameplay exists yet.

## Blockers

None currently. User approvals are required after a concrete tested PR is ready, not before implementation.

## Polish checklist

M00 polish was resumed into roadmap work by the user's M01 request. The later Catppuccin/settings/design request is standing guidance only, as explicitly clarified by the user; it does not add UI implementation to M01. README hero/grid changes are documentation-only and complete.

## Evidence

[Implementation and local verification](reports/001-native-foundation.md) records actual passes, failures, and corrections. [PR #3](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/pull/3) targets beta for explicit implementation review. [Clean package verification](reports/002-clean-package-and-review-gates.md) passed. [Final native acceptance](reports/004-native-acceptance-and-unix-packages.md) passed. [Six-target CI and artifact verification](reports/005-six-target-build-and-artifact-acceptance.md) passed at c2459eb. [Standing rules and README layout](reports/006-design-rules-and-readme.md) are recorded; final documentation CI and user approvals remain. [Draft PR and wiki publication](reports/003-draft-pr-and-wiki-preview.md) is complete.
