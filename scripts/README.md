# PowerShell helpers

All helpers resolve the repository from PSScriptRoot, work from outside the repository, and return nonzero exit codes on failure. Do not silently install tools, provision infrastructure, delete files, or capture unrelated windows.

## Available now

- bootstrap.ps1 — read-only prerequisite check; reports missing Rust/MSVC prerequisites without installing anything.
- wiki.ps1 — generates wiki pages under ignored .local without committing/pushing; accepts -OutputPath, -PullRequestUrl, and -Approved (stable-only after approval).
- milestone.ps1 — required-document, milestone-folder, and local Markdown link validation. Use -All (M00–M20) or -Number 0. Add -RequireComplete to require Complete status; this does not certify evidence truth.

```powershell
& 'C:\path\to\colonist\scripts\milestone.ps1' -All
& 'C:\path\to\colonist\scripts\milestone.ps1' -Number 1 -RequireComplete
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

## Wiki generation

```powershell
pwsh -NoProfile -File ./scripts/wiki.ps1
pwsh -NoProfile -File ./scripts/wiki.ps1 -OutputPath '.local/wiki' -PullRequestUrl 'https://github.com/Codings-Biggest-Fan/colonist/pull/1'
```

The output is generated public Markdown with pinned source links, an explicit unapproved preview banner, and grouped Game Instructions navigation. Publishing requires a separate explicit git commit/push to the initialized wiki repository. No credentials are embedded. Use -Approved only from clean stable after user approval; it does not itself approve or merge anything.
