# Building

The five-crate Rust workspace includes M03’s native board preview and a liveness-only server. There are no playable games yet. Review builds are unsigned; no milestone release has been published.

## Prerequisites and channels

Local testing is Windows-first: Git, PowerShell 7, Rust 1.90.0 through rustup with rustfmt/Clippy, Microsoft C++ Build Tools with the Desktop development with C++ workload and Windows SDK, and a wgpu-compatible graphics environment. Install tools explicitly using the [Rust instructions](https://rust-lang.org/tools/install/) and [Microsoft guidance](https://learn.microsoft.com/en-us/cpp/build/vscpp-step-0-installation?view=msvc-170). Helpers never install tools.

```powershell
git clone https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville.git
Set-Location colonizers-of-resourceville
git switch nightly
pwsh -NoProfile -File ./scripts/bootstrap.ps1
pwsh -NoProfile -File ./scripts/milestone.ps1 -All
pwsh -NoProfile -File ./scripts/check.ps1 -Release
pwsh -NoProfile -File ./scripts/test.ps1
pwsh -NoProfile -File ./scripts/dev.ps1
```

The pinned toolchain must already be installed: rustup may otherwise download it. Commands stop on errors; check the exit status before continuing. Cargo restores dependencies from Cargo.lock. nightly is latest development work; beta is the approved implementation baseline; stable is approved and polished; main mirrors stable. M03 uses milestone-03/board-preview and adds a reproducible board with native camera/inspection controls. Record `git rev-parse HEAD` when reporting results.

## Native preview and server

`./scripts/dev.ps1 -App Desktop` opens the real egui/eframe window with wgpu. Resize it, check the title and text at your display scaling, then use Close preview or the native close button. There are no gameplay controls, engine, saves, rooms, bots, or account connections yet. M02 adds pure configuration contracts; test them with the workspace test helper.

In a separate terminal, `./scripts/dev.ps1 -App Server` starts the Axum/Tokio process on 127.0.0.1:3000. `Invoke-WebRequest http://127.0.0.1:3000/health` returns `ok`. Ctrl+C shuts it down. Set RESOURCEVILLE_BIND to an explicit socket address to change the binding. /health proves process liveness only. The desktop does not connect to it. No database or Supabase resource is required.

## Evidence and packaging

```powershell
./scripts/capture.ps1 -Milestone 1 -Name '001-native-foundation.png'
./scripts/package.ps1
```

Capture starts a real window, saves only its rendered frame, and exits. It refuses to overwrite evidence. -Width/-Height specify logical window points; -Zoom changes egui zoom for a layout stress check, not Windows display scaling. Inspect images and record actual metadata. Package creates a host-specific ZIP, binary/source identity, dependency notices, and SHA256 checksums in ignored dist/. Existing bundle directories are preserved and cause an error. Package output is a testing artifact, not an approved release or signed installer.

The server container can be built with Docker (not installed by helpers): `docker build -f crates/server/Dockerfile -t resourceville-server:testing .`. It runs as a nonroot user and only serves liveness in M01. On Linux, `./scripts/package.ps1 -ServerContainer` builds and exports it as a .tar.gz with a SHA256 sidecar; the GitHub Linux x86-64 and ARM64 jobs retain those image bundles. No image is pushed to a registry. Container export on a Windows host is explicitly rejected; use the GitHub Linux jobs.

## GitHub builds and clean checkout

[Native CI](../.github/workflows/native.yml) builds and packages on GitHub for Windows, Linux, and macOS on both x86-64 and ARM64. Development runs from nightly and release-candidate builds from stable, using each exact pushed commit; the same commit's checks appear on its PR. Manual dispatch is also available. Each runner verifies its host architecture, formatting, Clippy, tests, and optimized workspace build. Linux jobs also build and export the server container. Artifacts are named by target and commit; SHA256 sidecars accompany ZIPs and container archives. These are nonlocal checks and do not prove interactive runtime behavior. Releases must use GitHub-built artifacts from the approved polished stable revision; no workflow automatically releases or promotes a branch.

For clean-checkout verification, clone the review branch into a fresh folder and run the checks above. A shared Cargo dependency cache is acceptable; source and target build directories must be fresh. Actual results and limitations are tracked in [M01 progress](../milestones/milestone-01/progress.md).

## Troubleshooting

- Cargo missing: explicitly install Rust and reopen the shell, or verify the default Cargo bin directory. Bootstrap checks that directory as well as PATH.
- Linker or missing .lib: verify both the MSVC compiler and Windows SDK; wait for installation to finish before building.
- wgpu initialization error: check graphics drivers and adapter support; keep the actual error when reporting. No browser fallback exists.
- Address already in use: choose an unused RESOURCEVILLE_BIND address; the server returns a nonzero error.
- No gameplay: M01 is a native foundation preview. Follow the [roadmap](../ROADMAP.md).

## M04 turn testing

The native client now includes the [base turn-engine harness](base-turn-engine.md). F4 opens it without permanent board bars. Capture helpers additionally accept `-Scenario layout|setup|production|rejection|contrast|zoomed-out`; scenarios replay real rules transitions, except `contrast`, which is explicitly a renderer fixture, not a legal game. Captures are real native wgpu frames, not mockups. No game save/network payload format is available yet.
