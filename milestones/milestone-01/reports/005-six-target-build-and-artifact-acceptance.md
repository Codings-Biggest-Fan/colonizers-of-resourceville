# 005 — Six-target GitHub build and artifact acceptance

Date: 2026-10-15 (America/Los_Angeles). [Progress](../progress.md) · [Run 38829638371](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/actions/runs/38829638371) · [CI snapshot](../evidence/ci-c2459eb.json) · [Artifact verification and hashes](../evidence/artifacts-c2459eb.json).

## Executed GitHub checks

Source c2459eba09ab0c59338fb4c1ff53f2b4c463d59e passed all six native jobs with overall conclusion success. Each runner verified its Rust host architecture, documentation records, formatting, Clippy on all targets/features, optimized workspace build, workspace tests, native packaging, and artifact upload. Both Linux jobs also built/exported their server container. The earlier baseline run 38828706913 at db7f0b9 also completed successfully on all six targets.

| OS | Architecture | Result |
| --- | --- | --- |
| Windows MSVC | x86-64 | Passed |
| Windows MSVC | ARM64 | Passed |
| Linux GNU | x86-64 | Passed; image archive exported |
| Linux GNU | ARM64 | Passed; image archive exported |
| macOS | x86-64 | Passed |
| macOS | ARM64 | Passed |

## Actual artifact inspection

Downloaded the six actual GitHub artifacts by their recorded IDs. For each, verified the native ZIP against its SHA256 sidecar, extracted it to an independent ignored directory, and checked all 649 files against SHA256SUMS.txt. BUILD.txt identifies the exact c2459eb source. Inspected PE/ELF/Mach-O headers of both desktop and server binaries to confirm the stated architecture. All four Unix ZIPs now record mode 0755 for both binaries, proving the permission correction in the real Linux/macOS outputs.

For both Linux artifacts, also verified the exported .tar.gz image sidecar hash and inspected the saved image configuration: Linux amd64/arm64 respectively, nonroot user 10001, and /usr/local/bin/resourceville-server entrypoint. Image notices are included by the Dockerfile. This verifies exported image configuration and packaging; no production deployment or remote interactive gameplay is claimed.

The container helper performs an actual Docker build/save and gzip export, preserves existing outputs, and pushes no registry image. On Windows it returns exit 1 with the Linux/GitHub instructions. Latest clean Windows native packaging also passed from outside the repository at c2459eb. Native runs now originate from exact nightly/stable commits, avoiding repeated six-target builds for the same branch/PR update. Releases still require GitHub-built artifacts from the approved polished stable revision.

## Handoff and remaining gates

The current Rust UI code was already built/launched/inspected from the clean source at 066b440; no Rust UI changes followed. Source changes after c2459eb are documentation-only: the user's standing design rules, README presentation, and actual acceptance records. Validate that final documentation revision before requesting approval. The GitHub testing artifacts above remain pinned evidence of the actual implementation/package code.

M01 stays Verifying for explicit implementation approval, separate polish approval, stable/main promotion, and the approved v0.0.1 prerelease. No approval is inferred from completed checks or the user asking for README commits. M02 remains planned. No further computer access is needed for these documentation changes.
