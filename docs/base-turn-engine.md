# M04 base turn engine

TESTING PREVIEW — unapproved M04 development; complete offline play arrives in M06.

Run the [native client](building.md), right-click the board and choose **Turn engine test**, or press F4. Start the four-seat test. Select a ring-marked intersection, then Place settlement; select an adjoining dotted edge, then Place road. Setup follows seats 1, 2, 3, 4, 4, 3, 2, 1. Each seat receives resources beside its second settlement.

The panel shows the current player and phase. After setup, set two test dice (1–6 each) and Apply dice. These are supplied test outcomes, not a random roll. Matching hexes produce one resource per settlement or two per city. The finite bank holds 19 of each resource; multiple recipients get none of a short resource, while a sole recipient gets the remaining supply. Building returns its cost to the bank. End turn advances to the next seat and requires a fresh roll. Seven enters a pending robber phase; Reset test is the available recovery until M05.

Select an edge/intersection to place roads, settlements or upgrade a city. Errors explain the failed condition without changing the state. Costs use resource order timber, wool, grain, clay, ore: road 1 timber + 1 clay; settlement one each except ore; city 2 grain + 3 ore. Piece limits are 15 roads, 5 settlements and 4 cities; upgrading returns a settlement. Opponents' settlements block road continuation. The distance rule applies during setup and ordinary turns.

The panel is a development harness controlling synthetic seats, not a supported shared-device game. It shows public hand counts, never hands or private points. Close, collapse or drag it to reach covered targets; F4 reopens it. Wheel zoom/right-drag pan and keyboard inspection remain. Seed regeneration clears the test. No cloud, saves, bots, trading, cards, awards or victory are enabled. Runtime configuration rejects options beyond the implemented subset.

## State boundary

`GameConfig` v1 and existing protocol/save envelope versions are unchanged. In-memory engine v1 accepts a validated seat permutation and explicit controlled outcomes separately from the board seed. Authoritative state implements neither Debug nor Serialize; public views exclude individual resource types, and hands require a separately resolved seat. This is an API boundary, not authentication or online privacy evidence. No engine save payload or gameplay transport is introduced.

[Progress](../milestones/milestone-04/progress.md) · [Native evidence](../photos/milestone-04/index.md)

## Current graphic HUD (2026-10-16)

The sidebar and bottom dock now appear by default. Hover an icon for its name/cost; keyboard focus and accessible names identify icon buttons. The play triangle starts the real turn test. Select a target on the board, then use the road, house or paired-house icon to place a road/settlement or upgrade a city. The finish arrow ends a build turn. Dice input remains explicit supplied test values; the dice button applies them. Fit/reset live below the player portraits. Active player has an outlined portrait. Production tokens remain numbered; extra player/piece counters and house numerals are hidden. Card stack and trade arrows are disabled, with tooltips explaining M05 availability. This replaces the older floating-panel steps above; no complete-game claim.

Current count follow-up (2026-10-16 user request): resource-card pictogram plus live public total and a development-card pictogram plus unavailable dash below each portrait. Dev-card state arrives at M05; no count is invented. Small build-icon numbers show the active player's remaining road/settlement/city stock. No private card types or fake scores. This supersedes the previous instruction to hide these particular counters.

Current light HUD direction (2026-10-16): Catppuccin Latte supersedes the earlier dark rail. Use horizontal player rows with public card counts, a short bottom action dock and accepted public activity at normal sizes. Compact/scaled views place dice below the heading and hide activity. Keep original artwork, hidden hands private and unimplemented card/trade controls disabled. Appearance settings/persistence are not implemented by this correction.

Current screenshot-led M04 direction (2026-10-16) supersedes earlier layout notes: full-height right activity and unavailable chat region, five-resource bank plus unavailable development deck, compact opponent rows and active seat last; bottom covered hand/build dock with dice above actions; top-left native back/guide/audio/options/support tools. Audio/support/card/trade remain disabled until implemented. Generated terrain v5 uses larger classic resource symbols; desert has no resource/card. Intentional active-seat hand reveal automatically covers on focus loss, new test or accepted end-turn. Keep captures covered and branding original. Report 009 records provenance and limits.
