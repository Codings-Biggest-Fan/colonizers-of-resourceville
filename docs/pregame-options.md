# Pregame options catalogue

M02 contract and public UI audit, 2026-10-15 (America/Los_Angeles). No native settings controls or gameplay are delivered here. Catalogued does not mean playable.

## Public reference observation

Source: [public reference](https://colonist.io/), mobile web guest path, Rooms → Create Room → Mode/Map/Settings. Visible UI and DOM selected classes inspected with user permission. No match started, purchase made, account signed in, or private API used. Temporary guest setup abandoned by closing the tab. Values describe this session, not universal defaults. No identities or room codes retained. Locked choices recorded, not unlocked or tested.

| Observed item | Choices / session value | Uncertainty | Disposition / delivery |
| --- | --- | --- | --- |
| Bot difficulty | Easy selected; Medium; Hard | Strategy differences untested | Original bot levels M06; stronger bots M20 |
| Match mode | Bots; Casual; Ranked | Ranked requires login | Offline M06; casual M18; ranked M19 |
| Family | Base available; larger base, Citadel, nautical, nautical/Citadel locked | 5–6 and base 7–8 distinct | Base M04–M06; expansions M11–M14; no paywall |
| Fast simultaneous mode | Listed, locked | Exact reference mode unverified | Original accelerated preset M17; no copied simultaneous-mode parity promised |
| Rooms / spectate / join | Open Rooms, Spectate, Join Room | Access/recovery untested | Private M08–M09; public M18 |
| Private game | Unselected | Code/password behavior unobserved | Private default M08; public opt-in M18 |
| Hide bank cards | Unselected | Exact hidden counts untested | Bank visibility policy M10 |
| Friendly robber | Selected | Threshold/timing unobserved | Custom base policy M10; distinct official variant M15 |
| Balanced dice | Selected | Algorithm unknown | Original policy M10; no parity claim |
| No Trolls | Locked | Eligibility/behavior unknown | Social/moderation policy M18 |
| Turn timer | Initially 60s; cycles 30s, 15s, 200m, 120s, 60s | Exact 200m label observed; semantics unknown | Untimed default, bounded seconds M10; no copied 200m preset |
| Max players | Initially 4/8 | Range not exhaustively stepped | 2–4 M06/M08; 5–8 M14 |
| Victory points | Slider 3–20, value 10 | Expansion defaults unobserved | Scenario default/custom M10–M17 |
| Discard limit | Slider 5–20, value 7 | Expansion modifiers untested | M10; walls M11 |
| Base map | Base selected; Base Random locked | Generation unknown | Original Heartland M03/M10 |
| Weekly map | Volcano available | Rotation/terrain unverified | Original maps M17; no copied map |
| Extra maps | Locked: larger base, Earth, USA, UK & Ireland, Diamond, Gear, Lakes, Pond, Twirl, Scramble, Black Forest, Gold Rush | Geometry/rules unverified | Original templates M17; no promised replica |
| Membership / login / color access | Gated hosting/colors advertised | Commercial behavior, not rules | No in-game purchases or copied entitlements |

Paid, authenticated, expansion-specific, in-game and hidden controls remain unobserved. Append discoveries with dispositions; unknown behavior never enables a control.

## Product contract

All rows are planned behavior except M02 validation and readiness/freeze. One GameConfig drives rules, UI, service, presets and saves. Room security and appearance are separate.

| Setting group | Default / contract | Compatibility / custom marker | Delivery |
| --- | --- | --- | --- |
| Family / scenario | Base / Heartland | Explicit family and combination whitelist | M04–M17 |
| Seats / mode | Four; 3–4 standard, casual base two, official caravan two, voyager two, paired 5–6 base, custom 7–8 base | Casual two and 7–8 custom; larger expansions deferred | M06/M14–M17 |
| Occupants / colors / bot levels | Human with bots locally; online human/bot/open | Occupant edits clear ready; open seats cannot start | M06/M08/M14 |
| Ready / accept | All configured occupants accept latest revision | Gameplay edits clear ready; acceptance freezes | M02 model, M08 live flow |
| Layout / seed | Random; board-only seed 0; fixed contract | Never seeds secret draws; fair-start constraints tested later | M03/M10 |
| Map / size / terrain / numbers / ports | Scenario-owned templates | Versioned map authoring later | M03/M17 |
| Victory | None = scenario default; override 3–30 | Any override custom | M10–M17 |
| Dice | Independent; BalancedBagV1 reserved | Balanced custom; incompatible with event deck | M10 |
| Raider | Standard; Friendly reserved | Friendly custom; cannot stack caravan protection | M10/M15 |
| Discard | Threshold 7; overrides 5–20 | Overrides custom; walls separate | M10/M11 |
| Trading | Enabled; standard bank/port rates | Disabled player trades custom; deadlines later | M05/M10 |
| Timing | Untimed; optional 15–600 seconds | Timed custom; setup/response/grace added later | M10 |
| Order | Random; SeatOrder reserved | Assigned order custom; per-seat list later | M10/M14 |
| Caravan variants | Off: protection/event deck/port award; official two via mode | Caravans only initially; event deck replaces production dice | M15/M17 |
| Expansion objectives / awards / missions | Scenario-owned | Explicit combinations only | M11–M17 |
| Room name / visibility / code / password / invite | Private friends; unguessable code; optional password | Server room policy, outside GameConfig | M08/M18 |
| Host transfer / recovery / substitution / pause | Preserve config and pending decisions | Authenticated membership; same command path for bots | M08–M10 |
| Audience / delay / chat / bank visibility | Private views; spectators M18 | Server validates; hands never broadcast | M10/M18 |
| Presets / import / export / sharing | Standard; derived custom marker | Version/size checked; ranked server-owned | M10/M17/M19 |
| Appearance / privacy cover | Local preferences, separate from game | Never clears readiness; no shared-device mode | Native settings when implemented; privacy M06/M09 |

BalancedBagV1 reserves a secret shuffled bag of all 36 ordered outcomes of two six-sided dice, consumed without replacement and independently reshuffled. This is an original policy, not the reference algorithm. M10 must inject/persist independent secret randomness, test distribution/refill/recovery, and label custom before enabling it. Citadel event dice need separate draws/tests. No generator is implemented here.

See [contract](configuration-contract.md), [inventory](rules.md), and [compatibility](compatibility.md). No selectable no-op controls are introduced.
