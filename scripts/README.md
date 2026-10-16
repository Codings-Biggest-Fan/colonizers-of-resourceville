# PowerShell helpers

All entry points resolve the repository from PSScriptRoot and work from another directory. Failures return a nonzero exit code. No helper silently installs tools or provisions resources.

| Helper | Arguments and behavior |
| --- | --- |
| bootstrap.ps1 | Read-only Rust/Git/MSVC detection; no build claim. |
| check.ps1 | Formatting and Clippy; -Release additionally builds the optimized workspace. |
| test.ps1 | -Suite All (default), Rules, Bots, or Server. Rules include M02 config/inventory tests; All also checks protocol privacy/versioning. No gameplay/bot engine yet. |
| dev.ps1 | -App Desktop (default) or Server; optional -Release. Use two terminals for both. |
| capture.ps1 | Required -Name NNN-description.png, -Milestone 1; optional -Width 1000 -Height 680 -Zoom 1. Captures only a fresh game frame and exits; never overwrites evidence. |
| package.ps1 | Build/package host binaries, dependency notices, and checksums under dist/. -SkipBuild is for CI after a successful release build. -ServerContainer builds/exports the Linux server Docker image with a SHA256 sidecar. Existing bundles cause an error. |
| milestone.ps1 | -All checks M00–M21; -Number accepts 0–21. Optional -RequireComplete checks records/status only, not evidence truth. |
| wiki.ps1 | Generate TESTING PREVIEW pages under .local. -PullRequestUrl identifies review. -Approved requires clean stable and prior explicit approval. |

common.ps1 is a shared definition file, not a standalone check. See [Building](../docs/building.md) for runnable commands, prerequisites, container instructions, and GitHub's six-target matrix. Native screenshot dimensions and inspection results belong in the [photo index](../photos/milestone-01/index.md).

Examples from outside the repository:

```powershell
& 'C:\path\to\colonizers-of-resourceville\scripts\check.ps1' -Release
& 'C:\path\to\colonizers-of-resourceville\scripts\test.ps1' -Suite Server
```

Wiki generation writes files only. Publication uses its separate Git repository; publish development/beta docs as labeled previews. Releases use GitHub-built artifacts only after implementation and polish approval. Helpers never automatically publish, promote, or install.
