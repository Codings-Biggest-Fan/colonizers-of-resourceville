# 001 — Native foundation implementation and local verification

Date: 2026-10-15 (America/Los_Angeles). Target v0.0.1 remains unreleased. [Plan](../plan.md) · [Progress](../progress.md) · [Photos](../../../photos/milestone-01/index.md) · [Implementation PR #3](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/pull/3).

## Scope and changes

The user requested M01, resuming roadmap work after M00 polish. Fetched stable at 49bd2c0 and fast-forwarded the milestone base to preserve its merged history. No beta/stable/main promotion was performed. Implemented the five-crate workspace, Rust 1.90.0 pin, Cargo.lock, original native preview, Axum/Tokio liveness server, helpers, and six-target GitHub matrix. Windows/Linux/macOS x86-64 and ARM64 builds were explicitly requested and added to AGENTS.md. No game contracts, rules, bots, saves, rooms, or cloud resources were added.

The user authorized Rust/MSVC/SDK installation. Rustup installed 1.90.0 with rustfmt and Clippy. The direct Build Tools installer command was rejected by execution policy; the safer WinGet catalog installation verified the installer hash and completed successfully. No security settings were changed. Initial compilation started before SDK installation finished and failed on missing dbghelp.lib; after installation the workspace built successfully.

## Executed checks

- bootstrap.ps1: passed after installation (Rust 1.90.0, Cargo, rustup, Git, MSVC detected).
- Initial workspace debug build: passed after SDK installation.
- check.ps1 -Release: passed formatting, Clippy, and optimized workspace build. A later pass also included capture features.
- test.ps1: passed; one substantive server test verifies /health body/status and the absence of /rooms. Empty rules/bots crates contain zero gameplay tests.
- Fresh remote clone under ignored .local/clean-m01 at 328c7ab: check.ps1 -Release and test.ps1 passed, invoked from the system temporary directory. This used the shared dependency download cache but a fresh source tree and target directory.
- Live server at loopback port 38173: /health HTTP 200 with ok; /rooms HTTP 404; a second bind failed with AddrInUse and exit 1. Ctrl+C was sent to stop the session; graceful shutdown still needs final confirmation.
- Scripts invoked from outside the repository: check, tests, capture, and package paths resolved correctly.
- Missing-Cargo simulation: exit 1 with explicit installation/bootstrap guidance. Duplicate capture and unsupported suite each returned exit 1. No files were overwritten.
- milestone.ps1 -All: passed 21 record sets and local documentation links. This is structural evidence only.
- Native frames captured and inspected at Windows 150% scaling and egui zoom 1 / 1.5. See the photo index for before/after failures and corrections.
- Actual native window identity and DPI 144 were checked. Targeted resizing changed client dimensions from 1500 × 1020 to 798 × 664 physical pixels and the process remained responsive. Injected-message button testing was inconclusive and is not claimed passed.
- Computer Use then clicked the observed Close preview accessibility button and observed that specific window disappear; title-bar Close was exercised on the remaining window and a subsequent inventory showed no Resourceville windows. The user interrupted access and required a question before further computer access; final clean-checkout UI access was then explicitly approved.
- Package prototype produced a host ZIP and dependency notices/checksums. Its source tree included uncommitted icon changes, so it is not a release candidate. The helper now rejects dirty sources and a final clean package remains pending.

## Visual fixes and implementation corrections

The first native launch exposed missing wgpu backend features even though compilation passed. Added explicit native DX12/Metal/Vulkan/GLES/WGSL support. eframe's legacy EFRAME_SCREENSHOT_TO shortcut panicked on wgpu; replaced it with viewport screenshot events and PNG encoding of only the real GPU-rendered frame. Initial screenshots exposed OS theme overriding text contrast; pinned the egui theme to Dark. Pinned Close preview in a footer and made compact layouts omit oversized decoration. Added an original procedural window icon; final native icon inspection is pending.

## GitHub, records, and limitations

Pushed implementation commits d3b0509, c5b4a95, and 328c7ab to milestone-01/foundation and nightly without force. Opened and attached draft PR #3 against beta. The connector lacked PR creation permissions; the existing Git credential authorized creation through the GitHub API without printing credentials. Documentation CI passed at 328c7ab. Six-target native build jobs were still running when this report was written. Superseded runs were cancelled to avoid wasting runner capacity; final jobs and Linux container builds still need completion evidence.

No native release, tag, database migration, infrastructure provisioning, signing, nonlocal interactive test, or M02 implementation occurred. A project license remains unselected. Package distributions preserve third-party notices separately.

## Next steps

Commit the original icon, packaging safeguards, and these actual records; update the clean checkout and rerun affected checks. Perform the approved final native-window check and archive selected-window evidence. Verify final package checksums and all six GitHub jobs, generate/publish the labeled PR wiki preview, update the draft PR with exact results, and request explicit implementation approval. Keep M01 Verifying until review/approval and the separate polish/stable release gates are satisfied.
