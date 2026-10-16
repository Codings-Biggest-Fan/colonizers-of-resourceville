# M04 base turn engine

TESTING PREVIEW — unapproved M04 development; complete offline play arrives in M06.

Run the [native client](building.md), right-click the board and choose **Turn engine test**, or press F4. Start the four-seat test. Select a ring-marked intersection, then Place settlement; select an adjoining dotted edge, then Place road. Setup follows seats 1, 2, 3, 4, 4, 3, 2, 1. Each seat receives resources beside its second settlement.

The panel shows the current player and phase. After setup, set two test dice (1–6 each) and Apply dice. These are supplied test outcomes, not a random roll. Matching hexes produce one resource per settlement or two per city. The finite bank holds 19 of each resource; multiple recipients get none of a short resource, while a sole recipient gets the remaining supply. Building returns its cost to the bank. End turn advances to the next seat and requires a fresh roll. Seven enters a pending robber phase; Reset test is the available recovery until M05.

Select an edge/intersection to place roads, settlements or upgrade a city. Errors explain the failed condition without changing the state. Costs use resource order timber, wool, grain, clay, ore: road 1 timber + 1 clay; settlement one each except ore; city 2 grain + 3 ore. Piece limits are 15 roads, 5 settlements and 4 cities; upgrading returns a settlement. Opponents' settlements block road continuation. The distance rule applies during setup and ordinary turns.

The panel is a development harness controlling synthetic seats, not a supported shared-device game. It shows public hand counts, never hands or private points. Close, collapse or drag it to reach covered targets; F4 reopens it. Wheel zoom/right-drag pan and keyboard inspection remain. Seed regeneration clears the test. No cloud, saves, bots, trading, cards, awards or victory are enabled. Runtime configuration rejects options beyond the implemented subset.

## State boundary

`GameConfig` v1 and existing protocol/save envelope versions are unchanged. In-memory engine v1 accepts a validated seat permutation and explicit controlled outcomes separately from the board seed. Authoritative state implements neither Debug nor Serialize; public views exclude individual resource types, and hands require a separately resolved seat. This is an API boundary, not authentication or online privacy evidence. No engine save payload or gameplay transport is introduced.

[Progress](../milestones/milestone-04/progress.md) · [Native evidence](../photos/milestone-04/index.md)
