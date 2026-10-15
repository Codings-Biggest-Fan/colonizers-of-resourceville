# Building on Windows

## Current availability

M00 is repository/docs/GitHub setup. There is no Cargo.toml, native executable, server, or container build yet. Native scaffolding and tested launch/package helpers belong to M01. Planning files and source ZIPs are not playable game builds.

## Prerequisites

- Git and PowerShell 7.
- For M01 onward: rustup with x86_64-pc-windows-msvc, rustfmt, and Clippy.
- Visual Studio C++ Build Tools: Desktop development with C++ workload, MSVC compiler/linker, and Windows SDK.
- A compatible Windows graphics environment for the planned wgpu renderer.

Install explicitly through the official [Rust instructions](https://rust-lang.org/tools/install/) and [Microsoft C++ Build Tools guidance](https://learn.microsoft.com/en-us/cpp/build/vscpp-step-0-installation?view=msvc-170). Scripts never silently install software. Pin exact Rust/dependency versions when scaffolding.

## Clone and choose a channel

```powershell
git clone https://github.com/Codings-Biggest-Fan/hearthline.git
Set-Location hearthline
git switch stable
```

stable is the last approved PR (initial baseline until approval), beta is the active unapproved PR, and nightly is the latest pushed work. A milestone branch identifies that milestone's implementation. Check the tested commit before reporting results.

```powershell
git fetch origin
git switch beta
git rev-parse HEAD
pwsh -NoProfile -File ./scripts/milestone.ps1 -All
pwsh -NoProfile -File ./scripts/bootstrap.ps1
```

The structure check should pass now. Bootstrap checks prerequisites and currently reports Rust/MSVC missing or unverified in the recorded environment. It does not install or build.

## Future Cargo build and verification

These commands apply only after M01 supplies the workspace and lockfile. They have not passed for the current docs-only revision. Stop after any nonzero command.

```powershell
if (-not (Test-Path -LiteralPath ./Cargo.toml)) {
    throw 'This revision contains setup/docs only. Native builds start in Milestone 01.'
}
cargo build --workspace --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build --workspace --release --locked
```

M01 must add verified desktop/server launch and packaging commands here before advertising them. Do not invent package names, executable paths, URLs, or container images.

## Troubleshooting

- Rust missing: check rustup and reopen the shell after PATH changes.
- Linker/SDK errors: verify the C++ workload and Windows SDK.
- No Cargo.toml: expected before M01, not an application build failure.
- Structure errors: follow the reported path and fix the record/link.
- No playable download: no native release exists yet. Check verified assets in GitHub Releases after approval.

Native releases require clean-checkout builds/tests, a real launch, and inspected screenshots. Record actual evidence in milestone reports. See [delivery workflow](github-workflow.md).
