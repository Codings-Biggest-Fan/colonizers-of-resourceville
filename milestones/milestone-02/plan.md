# Milestone 02 — Rules and configuration contract

Target version: v0.0.2 (planning identifier, no published release). Status: Complete. The user merged tested PR #7 into beta on 2026-10-15; no stable/polish promotion inferred.

[Roadmap](../../ROADMAP.md) · [Progress](progress.md) · [Reports](reports/) · [Photos](../../photos/milestone-02/index.md)

## Goal and dependencies

Freeze source-backed scope and versioned pure Rust configuration/protocol boundaries before board/engine implementation. M01 acceptance and user merges are recorded in [M01 reconciliation](../milestone-01/reports/007-m02-handoff-reconciliation.md). User requested M02 on 2026-10-15.

## Refined scope against code

The rules/protocol crates were empty boundaries. Add serde contracts to rules; protocol depends on/re-exports those types. Keep desktop, bots and liveness server unchanged. No settings UI, gameplay, room dispatch, cloud service or persistence engine.

- [x] Audit ordinary public guest setup: choices/defaults/locks/date/uncertainties and disposition for every observed setting; do not claim authenticated or paid options were tested.
- [x] Pin 2025 public core books; enumerate development/progress decks, scenarios, variants and combination candidates with original product labels.
- [x] Add versioned GameConfig, family/scenario/seat modes, capabilities, command/event envelopes, recipient views, topology identifiers and save header.
- [x] Specify defaults/custom status, strict validation, readiness invalidation, atomic failure and immutable acceptance.
- [x] Assign catalogue items to milestones; define deterministic fixtures and private-information boundaries.

## Interfaces and formats

[Contract](../../docs/configuration-contract.md), [catalogue](../../docs/pregame-options.md), [inventory](../../docs/rules.md), [compatibility](../../docs/compatibility.md). Config/rules/protocol/save versions initially 1; reject unknown versions, fields and oversized data. No existing save migration. Map/preset schemas and advanced options extend the versioned model only with actual behavior in their owning milestones.

## Verification and evidence

Round-trip all current contract seat modes and default fixture; verify scenario-source registry and deck totals; reject invalid combinations/bounds; preserve state on rejection; reset ready on effective edits; freeze accepted config; separate two synthetic recipient hands. Run formatting, Clippy, workspace tests, optimized build/package, record/link/wiki checks and all six GitHub builds. Record actual results in progress/report; no native screenshot needed because no UI changes. M01 photos stay labelled M01.

## Delivery and handoff

Use milestone-02/rules-contract for implementation and later polish; push nightly. Implementation PR targets beta and remains open for explicit approval. Do not advance beta/stable/main, tag, publish a release, or begin M03 automatically. Gates and implementation review passed; PR #7 was merged by the user. M02 is Complete and M03 awaits the next request. No release publication is due before M06. Broader gameplay, card effects, event-card distribution, expansion layouts and combination execution are assigned later work, not delivered features.
