# PowerShell helpers

All helpers resolve the repository from PSScriptRoot, work from outside the repository, and return nonzero exit codes on failure. Do not silently install tools, provision infrastructure, delete files, or capture unrelated windows.

## Available now

- bootstrap.ps1 — read-only prerequisite check; reports missing Rust/MSVC prerequisites without installing anything.
- milestone.ps1 — required-document, milestone-folder, and local Markdown link validation. Use -All or -Number 1. Add -RequireComplete to require Complete status; this does not certify evidence truth.

```powershell
& 'C:\Users\asharma\Documents\colonist\scripts\milestone.ps1' -All
& 'C:\Users\asharma\Documents\colonist\scripts\milestone.ps1' -Number 1 -RequireComplete
```

## Remaining milestone-01 helpers

| File | Contract |
| --- | --- |
| dev.ps1 | Start desktop, server, or both with explicit arguments. |
| check.ps1 | Formatting check, Clippy, workspace checks; never rewrite files silently. |
| test.ps1 | Rules, bots, integration, or full suites with failures propagated. |
| capture.ps1 | Capture the selected game window to the selected milestone; record actual metadata. |
| package.ps1 | Windows bundle and server container artifacts. |

These five helpers do not exist yet because no Cargo application or container build exists. They must be implemented and exercised before milestone 01 is Complete; there are no success-reporting stubs.

Secrets stay outside scripts and tracked files. Logs/screenshots belong in their documented destinations only after real verification.
