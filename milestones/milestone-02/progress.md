# Milestone 02 progress — Rules and configuration contract

Status: Verifying

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
- [ ] Clean committed-revision local package and checksum/archive verification.
- [x] All 22 record sets and 614 local links validated; single M21 validation passed after fixing scalar-array handling. Final wiki/source review follows the commit.
- [ ] All six GitHub target builds/packages.
- [ ] Testing-preview wiki publication/navigation and implementation PR against beta.
- [ ] Explicit user implementation approval; keep beta/stable/main unchanged.

## Remaining and limitations

Complete final checks and archive evidence; no gameplay, board generation, bots, saves, live lobby, online commands or native settings introduced. The known public guest audit is complete within its stated boundary; locked/authenticated behaviors remain uncertain and assigned. Future engine milestones implement per-card effects, event-face distribution and per-scenario tests. No native screen changed, so no new runtime screenshots are applicable.

## Blockers

None in implementation. Pending checks are not passes. User review follows the concrete tested PR. M03 remains planned.

## Polish

No active Polish mode. Implementation and polish share this milestone branch.

## Evidence

[Session report](reports/001-contract-and-source-inventory.md), [source manifest](../../docs/rules-sources.json), [contract](../../docs/configuration-contract.md), [public UI audit](../../docs/pregame-options.md). Report records executed checks and remaining gates.
