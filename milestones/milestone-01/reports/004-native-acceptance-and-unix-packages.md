# 004 — Clean native acceptance and Unix bundle correction

Date: 2026-10-15 (America/Los_Angeles). [Progress](../progress.md) · [Photos](../../../photos/milestone-01/index.md) · [Prepared release notes](../release-notes.md).

## Final native check

The user explicitly approved computer access again. Initialized a fresh Computer Use session and launched only the clean-checkout optimized Windows executable built from 066b4401fec1d17ad0f19e1537a3fcd9804167f0. This contains the final Rust UI/icon code; subsequent source changes have been documentation and helper/package work.

Inspected the exact product title, original procedural title-bar icon, readable preview, proportional emblem, and visible footer. Archived only stable selected-game captures 009 and 010. Their actual JPEG dimensions are 1003 × 711 and 1707 × 1019 pixels. These are Computer Use logical screenshot dimensions; no additional physical-window size is inferred. Prior full-resolution frames recorded native pixels/point 1.5 and Windows DPI 144; no OS scaling was changed.

Native Maximize produced a settled full-window layout with no content/footer overlap. Restore encountered a transient minimized-state response; recovered the returned game window, and observed its native control back at Maximize. Initial accessibility mouse activation did not establish that Close preview fired, so it is not counted as a final mouse result. Keyboard Tab focused the observed Close preview button; Enter activated it; a later window inventory was empty. The executable launched, rendered, resized, and closed from the clean checkout. Earlier native Close and compact/zoom evidence remains in report 001 and the photo index.

Transient system-menu/resize captures returned unreliable bounds. They were not saved or published. No further computer access is needed for the current source changes.

## Package correction

Inspected a real GitHub-built macOS ARM64 archive from successful build run 38827890569 (artifact 12015992478, source 328c7ab). Both native binary entries had an executable permission mask of zero. This showed that Compress-Archive stripped Unix executable bits, even though the build and packaging jobs passed.

Updated package.ps1 to mark the desktop and server ZIP entries as regular executable files with Unix mode 100755 on non-Windows hosts before calculating the ZIP sidecar hash. Windows packaging is unchanged. This is an archive metadata correction; file bytes and the internal file hashes are unchanged. Final Linux/macOS artifacts must be inspected to prove this fix; a locally mirrored unit test would not establish actual archive behavior.

Added .dockerignore to keep build outputs, local files, evidence, Git metadata, and environment files out of Docker's build context. Updated stale crate/player/deployment/report-index text to reflect the actual M01 implementation, and prepared v0.0.1 release notes without publishing anything. Updated the PR template to preserve the approved-beta/separate-polish workflow.

## Checks and next steps

All helper scripts parse; the record/local-link check passes. In [run 38828706913](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/actions/runs/38828706913) at db7f0b9, Linux x86-64/ARM64, Windows ARM64, and macOS ARM64 completed successfully. Windows x86-64 and macOS x86-64 were still building at this checkpoint. The package correction requires a newer six-target run and actual archive verification. Keep M01 Verifying and PR #3 draft until that passes, then present the exact tested revision for explicit implementation approval. No promotion, release, or M02 work was performed.
