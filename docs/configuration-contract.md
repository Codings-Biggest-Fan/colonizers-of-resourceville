# Configuration and privacy contract v1

M02 adds pure library contracts, not live rooms, saves, a game engine, or native settings. Desktop/server remain the foundation preview and liveness endpoint.

## Ownership and versions

Rules owns GameConfig and validation; protocol re-exports it and owns wire/save envelopes. Config schema, rules revision, protocol and save versions start at 1. Unknown fields, enum variants, versions and malformed payloads fail closed. No v0 migration exists. Build versions are separate. [Rules inventory](rules.md) pins sources.

Ingress must use CommandEnvelope::decode or SaveEnvelope::decode; direct serde construction alone does not validate versions. Reject wire input above 64 KiB and saves above 1 MiB before parsing. serde's recursion limit stays enabled. Future size changes require reviewed allocation tests. Errors never echo parser payloads. Authentication, identifier membership, deduplication and transactions belong to M08–M09.

## Lifecycle

GameConfig::validate checks contract validity. Capabilities::m02 advertises no playable rulesets; require_playable rejects launch even for valid configurations. Future capabilities must check every scenario/option, not only family names.

ConfigLobby is a pure acceptance model. Config revision starts at zero; effective valid edits increment it and clear all ready bits. No-op edits preserve readiness/revision. Invalid/stale edits change nothing. Ready acknowledgments reference config revision without advancing it. Seats are zero-based and bounded. Freeze requires all configured seats ready, yields immutable FrozenConfig, and rejects later edits, readiness changes or repeat freeze. Freeze accepts a contract; it does not start gameplay.

The model assumes every configured seat is occupied. Future rooms must reject open-seat start, authorize host edits, resolve membership, manage bots through the same command path, and clear ready on occupant changes. Appearance stays separate. Custom status is derived, never caller-supplied. Board seed is public-layout-only.

## Messages and persistence

CommandEnvelope carries version, game ID, command ID, expected revision and typed lobby intent. Authenticated transport resolves identity; no actor is supplied in commands. ReplaceConfig, SetReady and AcceptConfig are contracts, not Axum endpoints. EventEnvelope includes revision, causal command ID, acceptance/readiness or stable rejection codes. Game actions, forced choices, deadlines and receipts extend versioned schemas in their owning milestones.

SaveEnvelope<T> checks bounds, versions, config and future authoritative payload. M06 implements atomic replacement and interruption/recovery/migration tests. M09 persists state, commands/receipts, offers/deadlines, bots and secret randomness before acknowledgment. Never save a recipient view as complete state. Board seed cannot resume secret draws.

VertexId, EdgeId and HexId are integer IDs. M03 defines stable topology and validates membership; screen coordinates never identify shared intersections.

## Privacy

PublicView contains public config, revision, resource counts and public points. PlayerView adds only its own PrivateHand. ObservationState is a projection fixture: it checks seat/count consistency and projects spectator or one member's hand. It implements neither Serialize nor Debug. PrivateHand, PlayerView and SaveEnvelope omit Debug. This fixture is not a gameplay engine.

Later authoritative state includes development/progress cards, private points, secret choices/offers, bot memory, hidden exploration, decks and secret random streams. Only recipient-permitted projections leave that boundary. Never put complete state into events, logs, replays, errors, notifications or spectators. Rejection codes must not disclose guessed secrets. Synthetic two-seat tests demonstrate current hand separation, not future networking privacy.

M06/M09 require separate-person/screen shoulder-surfing checks: deliberate reveal, quick cover, focus loss/handoff, notifications and screenshots. M02 has no private-hand UI and claims no multi-screen runtime evidence or shared-device game mode.

## Fixtures

The checked-in base fixture fixes board seed 42 independently of future draws. Tests cover seat modes, every scenario row, invalid combinations, stale/no-op/invalid edits, reset/freeze, unknown fields/versions, malformed/oversize input and separate recipients. Geometry, gameplay, bot knowledge, secret draws and network/storage recovery remain later work.
