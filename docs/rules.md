# Rules and source register

Status: M02 research and contracts; gameplay remains unimplemented. Access date: 2026-10-15 (America/Los_Angeles).

## Pinned edition

Use the 2025 English sixth-edition core family. The [official source index](https://www.catan.com/understand-catan/game-rules) separates current books from older editions. Source names below are research attribution only; product names, card labels, and scenario labels are original. No source artwork, text, or PDF is shipped. The [source manifest](rules-sources.json) records exact URLs, SHA-256 hashes, page counts, edition, and access date. A changed hash requires a reviewed inventory update, not silent adoption.

| Source ID | Public rulebook | Pages |
| --- | --- | --- |
| base | [CATAN – The Game (CN3081)](https://www.catan.com/sites/default/files/2025-03/CN3081%20CATAN%E2%80%93The%20Game%20Rulebook%20secure%20%281%29.pdf) | 12 |
| seas | [CATAN – Seafarers (CN3083)](https://www.catan.com/sites/default/files/2025-03/CN3083%20CATAN%E2%80%93Seafarers%20Rulebook%202025%20secured%20reduced.pdf) | 20 |
| citadels | [CATAN – Cities & Knights (CN3087)](https://www.catan.com/sites/default/files/2025-03/CN3087%20CATAN%E2%80%93Cities%26Knights_%20Rulebook.pdf) | 16 |
| caravans | [CATAN – Traders & Barbarians (CN3089)](https://www.catan.com/sites/default/files/2025-04/CN3089%20CATAN%20%E2%80%93%20T%26B%20Rulebook.pdf) | 24 |
| voyagers | [CATAN – Explorers & Pirates (CN3085)](https://www.catan.com/sites/default/files/2025-04/CN3085%20CATAN%20%E2%80%93%20E%26P%20Rulebook.pdf) | 16 |
| missions | [CATAN – Explorers & Pirates Mission Guide (CN3085)](https://www.catan.com/sites/default/files/2025-04/CN3085%20CATAN%20%E2%80%93%20E%26P%20Missions.pdf) | 20 |
| paired | [CATAN – 5–6 Player (CN3082)](https://www.catan.com/sites/default/files/2025-03/CN3082%20CATAN%20%E2%80%93%205-6%20Rulebook%202025%20reduced.pdf) | 4 |

## Card inventory

The manifest also pins all four 2025 expansion 5–6-player supplements: seas_paired (12 pages), citadels_paired (4), caravans_paired (12), and voyagers_paired (8). M14/M17 must extract scenario-specific extended layouts and deck/piece changes before enabling those contracts; pinning a supplement is not implementation evidence.

Each row is a distinct effect/type, with deck multiplicity. Source labels are lookup keys, never product UI copy. Exact rule behavior is implemented and tested in its owning milestone. Base honor cards are five equivalent point cards; freight honor cards are three equivalent point cards, with original art still to be authored.

| Original product label | Research source label | Deck | Copies | Source pages | Delivery |
| --- | --- | --- | --- | --- | --- |
| Patrol | Knight | base | 14 | base 3, 8 | M05 |
| Road Crew | Road Building | base | 2 | base 3, 8 | M05 |
| Supply Grant | Invention | base | 2 | base 3, 8 | M05 |
| Resource Levy | Monopoly | base | 2 | base 3, 8 | M05 |
| Civic Honor | Victory Point | base | 5 | base 3, 8 | M05 |
| Dicecraft | Alchemy | science | 2 | citadels 13-14 | M11 |
| Hoist | Crane | science | 2 | citadels 13-14 | M11 |
| Rampart | Engineering | science | 1 | citadels 13-14 | M11 |
| Survey Exchange | Invention | science | 2 | citadels 13-14 | M11 |
| Waterworks | Irrigation | science | 2 | citadels 13-14 | M11 |
| Infirmary | Medicine | science | 2 | citadels 13-14 | M11 |
| Deep Quarry | Mining | science | 2 | citadels 13-14 | M11 |
| Scholarship | Printing | science | 1 | citadels 13-14 | M11 |
| Route Works | Road Building | science | 2 | citadels 13-14 | M11 |
| Armory | Smithing | science | 2 | citadels 13-14 | M11 |
| Dock Exchange | Commercial Harbor | trade | 2 | citadels 14-15 | M11 |
| Guild Claim | Guild Dues | trade | 2 | citadels 14-15 | M11 |
| Trade Envoy | Merchant | trade | 6 | citadels 14-15 | M11 |
| Cargo Charter | Merchant Fleet | trade | 2 | citadels 14-15 | M11 |
| Supply Levy | Resource Monopoly | trade | 4 | citadels 14-15 | M11 |
| Commodity Levy | Trade Monopoly | trade | 2 | citadels 14-15 | M11 |
| Route Accord | Diplomacy | politics | 2 | citadels 15-16 | M11 |
| Intelligence | Espionage | politics | 3 | citadels 15-16 | M11 |
| Rally | Encouragement | politics | 2 | citadels 15-16 | M11 |
| Court Order | Intrigue | politics | 2 | citadels 15-16 | M11 |
| Collection | Taxation | politics | 2 | citadels 15-16 | M11 |
| Civic Charter | Constitution | politics | 1 | citadels 15-16 | M11 |
| Defection | Treason | politics | 2 | citadels 15-16 | M11 |
| Celebration | Wedding | politics | 2 | citadels 15-16 | M11 |
| Disruption | Sabotage | politics | 2 | citadels 15-16 | M11 |
| Apprehend | Capture | siege | 4 | caravans 16-19 | M15 |
| Enlist | Knighthood | siege | 14 | caravans 16-19 | M15 |
| Rapid Patrol | Swift Knight | siege | 4 | caravans 16-19 | M15 |
| Changed Allegiance | Treason | siege | 4 | caravans 16-19 | M15 |
| Route Patrol | Knight | freight | 16 | caravans 20 | M15 |
| Freight Works | Road Building | freight | 3 | caravans 20 | M15 |
| Second Journey | Swift Journey | freight | 3 | caravans 20 | M15 |
| Craft Honor | Victory Point | freight | 3 | caravans 20 | M15 |

Base: 25 cards. Science, trade, and politics: 18 each, 54 total. Siege: 26 replacement cards. Freight: 25 replacement cards. Open Seas uses the applicable base deck with scenario overrides; Voyagers uses no base development or progress deck. Caravans also has a 37-card event deck: 36 production/event cards plus a cycle-reset card. Its 13 distinct event effects are listed in the machine inventory; number/effect multiplicities must be encoded from the physical card inventory at M15, not guessed from dice probabilities.

## Scenario inventory

| Product scenario | Source lookup | Source pages | Delivery |
| --- | --- | --- | --- |
| Heartland | Base game | base 4-12 | M04 |
| Citadel Heartland | Standard game | citadels 4-11 | M11 |
| New Horizons | Heading for New Shores | seas 5-6 | M12 |
| Four Isles | The Four Islands | seas 7-8 | M12 |
| Mistbound | The Fog Islands | seas 9-10 | M12 |
| Desert Crossing | Through the Desert | seas 11-12 | M12 |
| Lost Kin | The Forgotten Tribe | seas 13-14 | M12 |
| Woven Coasts | Cloth for Catan | seas 15-16 | M12 |
| Corsair Strongholds | The Pirate Islands | seas 17-18 | M12 |
| Great Works | The Wonders of Catan | seas 19 | M12 |
| Free Archipelago | New World | seas 20 | M12 |
| Shoals | Fishing on Catan | caravans 9-10 | M15 |
| Riverlands | Rivers of Catan | caravans 11-12 | M15 |
| Caravan Trail | Merchant Trains | caravans 13-15 | M15 |
| Raider Siege | Barbarian Attack | caravans 16-19 | M15 |
| Freight Routes | Traders & Barbarians | caravans 20-24 | M15 |
| First Landfall | Land Ho! | missions 4-7 | M16 |
| Corsair Camps | Pirate Lairs | missions 8-11 | M16 |
| Fishing Grounds | Fish for Catan | missions 12-15 | M16 |
| Spice Routes | Spices for Catan | missions 16-20 | M16 |
| Grand Voyage | Full game | voyagers 4-15 | M16 |

Caravans variants at M15: Gentle Raider (source Friendly Robber), Port Warden (Harbors of Catan), Event Deck (CATAN Event Cards), and Official Two (CATAN for Two). Official Two is independent of the casual base two-seat contract. Voyager missions: corsair camps, fish transport, and spice transport; Grand Voyage combines all three.

## Decisions and fixtures

- Base uses 10 points by default, 7-card discard threshold, independent production dice, ordinary trading, and untimed play. Two seats are explicitly casual. No future-randomness state is derived from a board seed.
- Pin current paired-player 5–6 rules, not the historical special building phase (paired source p3). Custom 7–8 rules require a separately reviewed specification at M14. No historical edition menu is promised.
- Citadel card names and effects changed in this edition. Use this inventory, not an old online card list. The science/trade/politics decks replace base development cards. Scenario default victory conditions remain scenario-owned.
- Combined Open Seas/Citadels contracts initially whitelist New Horizons and Desert Crossing (citadels p12); other combinations stay rejected pending source review and tests. See [compatibility](compatibility.md).
- Deterministic fixtures: default four-seat base; casual two-seat; official two-seat Shoals; paired five-seat; custom eight-seat; fixed seed 42; invalid family/scenario; stale revision; readiness reset; frozen mutation; unknown versions; oversized input; public/private serialization. Tests use no global random source.
- M03 fixtures add stable topology and board-only generation. M04–M06 add action replay and hidden draw streams injected separately. M11–M17 add per-row setup, effect, forced-choice, shortage, scoring, save/recovery, and bot fixtures before enabling any row.

## Limits and follow-up

### Fog maps — user observation, 2026-10-15

The user reports that the reference game includes fog maps: tiles begin hidden and are revealed when a road or settlement is built in their proximity. Preserve this as a gameplay requirement for Mistbound/Open Seas (M12), with reusable hidden-map authoring at M17. Exact proximity/adjacency, ship-trigger behavior, reveal rewards, and visibility to other players require source verification and deterministic tests before implementation. This observation does not mean fog gameplay is implemented in M02.

The register pins scope, not full implemented rules or completed-game evidence. Individual rulebook diagrams, card interactions, event-card face distribution, expansion 5–6 layouts, and old-edition combination notes require implementation-time extraction and tests. These are assigned work, not claims of parity. Promotional, historical, and standalone products remain excluded. The machine inventory is checked for source references, unique IDs, deck totals, scenario enum coverage, and assigned milestone.

## M04 engine extraction

The pinned base PDF SHA-256 was rechecked against the manifest during M04. Its extracted text explicitly supplies second-settlement starting resources, production of one/two per settlement/city, finite-bank shortage behavior including the sole-recipient exception, road connectivity and settlement distance, and piece limits. Engine v1 implements this subset with explicit dice and seat order. Seven remains pending M05; trading/cards/awards/victory are absent. Resource names in product code are original labels mapped to the pinned inventory; no rulebook text or artwork is shipped.
