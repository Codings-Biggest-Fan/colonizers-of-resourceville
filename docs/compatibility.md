# Compatibility matrix

M02 contracts and policy; no gameplay is playable. Configuration validity differs from runtime capability. [Rules](rules.md) and [manifest](rules-sources.json) pin editions.

| Family / combination | Contract scope | Delivery / gate |
| --- | --- | --- |
| Base | Heartland, 3–4 standard; separate casual two | M04–M06; games and save/resume |
| Citadels & Sentinels | Citadel Heartland, 3–4 | M11; 54 progress cards and raid/knight fixtures |
| Open Seas | Eight authored scenarios plus Free Archipelago | M12; per-scenario setup/objective/scoring/bots |
| Open Seas + Citadels | New Horizons and Desert Crossing only | M13; citadels p12; interaction tests |
| Five–six base | Current official paired turns | M14; paired p3; not historical special building |
| Seven–eight base | Explicit custom mode | M14; reviewed turn/piece/layout rules |
| Larger expansion layouts | Rejected now | M14/M17; per-scenario extension contracts |
| Caravans & Raiders | Five scenarios; four variants including Official Two | M15; replacement decks and overrides |
| Voyagers & Corsairs | Four teaching scenarios plus Grand Voyage; own two-seat mode | M16; cargo/harbor/corsair missions |
| Other documented combinations | Rejected now | M17; edition-compatible whitelist and tests |

The 2025 Citadel book p12 documents nautical interactions. Begin with its two recommended scenarios; rejecting others is a product restriction, not a source prohibition. Gold fields do not produce commodities; routes/knights and initial corsair placement require combined tests.

The [official index](https://www.catan.com/understand-catan/game-rules) lists old-edition combination notes, candidates for M17 rather than automatic sixth-edition support. Candidate inventory: all ten pairs among Shoals, Riverlands, Caravan Trail, Raider Siege and Freight Routes; all-five mix; each of the five with Open Seas and with Citadels; Official Two with Citadels; Voyagers with each of Open Seas, Citadels and Caravans. The Voyager/nautical note describes component reuse rather than merging systems. No arbitrary three/four-family mixture is promised. A source can constrain or reject combinations; an index link does not prove support.

Pinned 2025 scenario-local notes take precedence over older notes. Broader composition stays deferred until setup, legality, choices, inventories, scoring, bots, privacy and save/recovery tests pass. OfficialTwo and VoyagerTwo differ from CasualTwo. Seven/eight seats require reviewed rules, not blindly multiplied counts.

Config/rules/protocol/save versions start at 1. Unknown versions produce incompatibility errors; no migration yet. Maps/presets gain schemas with implementation. M01 binaries do not load M02 configs. All six OS/architecture targets remain in CI; current interactive runtime evidence remains Windows M01. Native UI is unchanged.
