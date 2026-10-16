# Milestone 02 progress — Rules and configuration contract

Status: Complete

Target version: v0.0.2 (no release due)

Last updated: 2026-10-15 (America/Los_Angeles)

[Plan](plan.md) · [Reports](reports/) · [Photos](../../photos/milestone-02/index.md) · [Roadmap](../../ROADMAP.md)

## Implemented

- Public guest setup audit, uncertainties/locks/defaults and disposition/delivery for every observed setting.
- Source hashes, 38 development/progress effect rows, 21 core scenario contracts, four caravan variants and 13 event-effect categories; explicit supported/unsupported combinations.
- Strict versioned GameConfig, validation/custom marker, readiness reset/immutable freeze, no-playable-capability guard.
- Versioned command/event/save envelopes, bounded ingress, topology IDs and synthetic recipient projections.
- Fog-map observation retained in rules knowledge, with exact reveal adjacency/ship behavior pending M12 verification.

## Verification

- [x] Initial focused contract/inventory/privacy tests passed (10 tests).
- [x] Formatting, all-target/all-feature Clippy, optimized workspace build and all 13 workspace tests (12 new contract/inventory/privacy tests, one existing server test).
- [x] Clean source fd6c236 local package; independent ZIP/source/architecture checks and all 649 file hashes passed.
- [x] All 22 record sets and final documentation links validated; single M21 check passed after fixing scalar-array handling. Source/wiki review passed.
- [x] All six GitHub target builds/tests/packages passed at fd6c236; actual artifact hashes, 3,894 package file hashes, architectures, Unix executable bits and both Linux container configurations verified.
- [x] TESTING PREVIEW wiki published and HTTP/navigation verified; [PR #7](https://github.com/Codings-Biggest-Fan/colonizers-of-resourceville/pull/7) targeted beta.
- [x] User explicitly confirmed merge; GitHub verifies tested fd6c236 merged into beta at 3f32f09. Stable/main unchanged; no polish approval or release inferred.

## Remaining and limitations

M02 gates and user implementation approval passed. M03 is next and awaits a new request. No gameplay, board generation, bots, saves, live lobby, online commands or native settings introduced. Public guest audit is complete within its stated boundary; locked/authenticated behavior stays uncertain and assigned. Future milestones own per-card effects, event-face distribution and per-scenario runtime tests. Native UI is unchanged; no new screenshot applies. M12/M17 fog tasks and final M21 website/wiki/docs are planned, not implemented.

## Blockers

None in implementation. No pending implementation check; user merge is verified. Further polish/promotion needs separate direction. M03 remains planned.

## Polish

No active Polish mode. Implementation and polish share this milestone branch.

## Evidence

[Session report](reports/001-contract-and-source-inventory.md), [verification and review handoff](reports/002-verification-and-review-handoff.md), [source manifest](../../docs/rules-sources.json), [contract](../../docs/configuration-contract.md), [public UI audit](../../docs/pregame-options.md). Actual results and remaining review gates are distinct; current implementation source is fd6c236 and final handoff changes are documentation only.
