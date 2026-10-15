# Compatibility matrix

**Status:** planned scope only; no combination or seat count is implemented. M02 builds the source-backed scenario matrix; M17 reconciles final coverage.

| Rules family | Core delivery | Intended seat support | Combination policy |
| --- | --- | --- | --- |
| Base | M04–M06 | 2–4 initially; 5–8 at M14 | Casual two-seat labeled separately |
| Cities & Knights | M11 | Scenario/ruleset validated | Standalone first; Seafarers combination at M13 |
| Seafarers | M12 | Scenario-specific | Ships/route rules distinct from Explorers & Pirates |
| Seafarers + Cities & Knights | M13 | Explicitly supported scenarios only | Follow documented combination notes |
| Extended-player presets | M14 | Official 5–6; custom 7–8 | Distinct turn/board/piece inventories |
| Traders & Barbarians | M15 | Variant/scenario-specific | Official two-player is a separate preset |
| Explorers & Pirates | M16 | Core scenario support | Distinct economy/setup/cargo rules |
| Other documented combinations | M17 | Explicit per-scenario approval | No arbitrary expansion mixtures |

Every supported scenario must have setup/objective tests, bot support, native interaction, save/recovery checks, and a completed-game record before advertised as available for offline solo.

Version rules/config/protocol/save/map/preset contracts. Migrations must preserve supported versions with tests or fail with a clear incompatibility message. Windows is the only promised release package through v1.0.
