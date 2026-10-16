# 003 — M04 testing handoff

Date: 2026-10-15 (America/Los_Angeles). Status: Verifying.

Implementation commit: d8441e7a2fc059693b21d31812a24625802d8dfa. [Draft beta PR #11](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/pull/11) is open and attached; no merge or promotion performed. nightly follows the development branch. The current code passed 28 workspace tests, formatting, all-feature Clippy and optimized build. This report adds delivery evidence and whitespace cleanup only.

## Packaging and documentation

Local Windows x86-64 package: resourceville-x86_64-pc-windows-msvc-d8441e7.zip. SHA-256 f7ca1152b1b036be848698705ad4aff901d6d3763e3eec166cd55167775b8a1b. ZIP CRC, sidecar checksum and all 649 file hashes verified. It is an unsigned testing package, not a published release. Package remains under ignored dist/; no release/tag/artifact publication performed.

TESTING PREVIEW wiki was generated with scripts/wiki.ps1, identifying source commit and PR #11, then published through the wiki repository. HTTP 200 and preview labels verified for Home, Building, Game Instructions, getting started, turns/trading, M04 progress and engine guide. The engine guide also showed the source commit and PR. Reviewed source-to-wiki mappings remain script-generated. The first wiki commit for this implementation is d619503; later handoff documentation can refresh the same preview.

Native builds were queued and all six jobs started in [run 38858905244](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/actions/runs/38858905244); documentation CI passed for d8441e7. Native CI/package results remain pending, not assumed successful. Subsequent documentation-only nightly pushes may supersede this run; inspect the [latest nightly runs](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/actions?query=branch%3Anightly).

## Remaining acceptance

Full human/native input and keyboard-only walkthrough remain unverified; the occluded-window helper was unsuitable for further capture and all saved evidence uses the game's own frames. Automated native action bridge and camera events pass. Cross-platform interactive runtime and multiple-screen privacy are unverified. Compact/scaled controls were reduced after inspection; their movable overlay can still cover board targets.

M04 remains Verifying and the beta PR remains draft/unmerged. Seven/robber resolution, trading/cards/awards/victory are M05; offline bots/saves are M06. No M05 work, Supabase work, channel promotion or release was performed. Next: finish six-target artifact/evidence review and human/native acceptance, then request explicit implementation approval through the normal beta workflow.
