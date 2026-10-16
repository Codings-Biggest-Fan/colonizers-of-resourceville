# Board preview

M03 adds an interactive base island layout preview. There is no playable setup, turn engine, inventory, trading, bot, room or save yet.

Open the native app with scripts/dev.ps1. Right-click opens a context menu with the seed input, Generate island, sample-piece toggle and Fit board. Enter a whole seed from 0 through 18446744073709551615. Invalid input is preserved with an error; the current board stays unchanged. Same seed plus generator version reproduces terrain, number tokens and ports, never future game randomness.

Right-click-drag the sea or island to pan and use the scroll wheel to zoom. There are no permanent bars, Board label or zoom slider. Home or the context menu fits the board. Click tiles, edges, intersections or harbor boats to inspect them. Terrain/harbor artwork has no on-board labels; hover and selection provide names and trade ratios. Those ratios describe the preview layout; no trading command exists.

Tab reaches native controls and the named board surface. With board focus, Enter selects, Ctrl+Right cycles every tile/edge/intersection/port, arrows pan, plus/minus zoom, Home fits and Escape clears selection. Fit resets pan/zoom. Appearance preferences and camera/selection are not persisted in this milestone.

## Geometry and generator contract

Rules owns sorted topology IDs: 19 tiles, 54 vertices, 72 edges and 30 coast edges. IDs derive from an integer axial/lattice model, never screen positions. IDs are identical across seeds and window sizes. Protocol re-exports the same tuple types, preserving their existing wire representation.

Generator version 1 uses a specified wrapping SplitMix64 stream for board setup only. Four forests/pastures/fields, three hills/mountains and one desert; standard 18 token inventory, no production token on desert and no adjacent 6/8. Bounded independent-set search selects the four high-production positions. Nine ports have distinct coastal endpoints, four general and one per producing terrain. This is the implemented random base preview preset; fixed beginner layouts, alternate distributions and larger/scenario boards remain later scope. Random shuffling does not claim cryptographic security or uniform sampling of every valid board.

The serialized board fixture pins generator output; it is not an accepted save format. Board topology is generated locally, not deserialized from untrusted input. A future change to seeded layouts requires a new generator version. Existing config/protocol/save version contracts are unchanged; no save migration or secret RNG stream exists.

## Evidence

[M03 progress](../milestones/milestone-03/progress.md) · [Native photo index](../photos/milestone-03/index.md) · [Building](building.md)
