# 005 — Polish: original Hearthline branding

Date: 2026-10-15 (America/Los_Angeles)

## User-reported flaw and cause

The setup used reference-product names as its own project/expansion branding. Those labels persisted from the first rewrite into repository metadata, README/wiki, plans, and scripts. The user explicitly required new branding.

## Fix

- Adopted Hearthline as working product identity and hearthline as repository/machine prefix.
- Renamed the existing public repository, preserving its identity, commits, branches, and PR #1; updated local source/wiki remotes and PR attachment/title.
- Replaced current docs/script/title/expansion labels with original Hearthline names. Added a branding guide and persistent AGENTS.md rules.
- Preserved the original report versions in Git history; sanitized current historical report copies under this explicit user instruction without changing their outcomes or SHAs.
- Kept the pre-correction browser image privately in ignored .local; it is not published as branded project evidence. A new inspected after-image will be archived after preview publication.

## Actual checks so far

- Source scan found no legacy product/expansion names in current Markdown, PowerShell, or workflow text after replacement.
- Public repository rename succeeded; default remains stable and PR #1 remains open against it.
- All 21 record sets and local links passed; wiki generation passed with the new site URL/name.
- Unsafe export/approval mode and pending-completion negative checks were previously verified in report 004.

## Remaining verification and boundaries

Publish updated milestone/nightly/beta refs, regenerate/push the clean-commit preview wiki, inspect new Home/Building/Game Instructions pages, capture new browser evidence, and recheck channel SHAs/CI. Record final outcomes in a subsequent report.

Stable/main retain the baseline until explicit user PR approval; no unapproved merge, tag, release, Rust implementation, or next milestone is authorized. Polish mode remains active until the user asks to resume.
