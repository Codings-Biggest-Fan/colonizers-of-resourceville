# Pregame options catalogue

**Status:** planned groups, not a completed reference audit or implemented settings. [M02 audit](../milestones/milestone-02/plan.md) freezes initial behavior contracts; [M17](../milestones/milestone-17/plan.md) reconciles coverage.

For every observed option, record source/date, available choices, default, exact semantics, compatibility, custom-rule marker, assigned milestone, implementation status, and evidence. Add newly observed options rather than silently excluding them. Unknown behavior stays explicit.

| Group | Required capability | Primary delivery |
| --- | --- | --- |
| Room | Name, visibility, code, optional password | M08/M10/M18 |
| Seats | Count, human/bot/open, colors, difficulty, host transfer | M06/M08/M14/M20 |
| Readiness | Ready/unready, host start, reset on gameplay edits | M08 |
| Ruleset | Base, expansion modules, compatible combinations, scenario | M10–M17 |
| Board | Map, size, fixed/random layout, terrain/numbers/ports, preview | M03/M10/M17 |
| Generation | Constraints, optional fair-start generator, board-only seed | M03/M10 |
| Victory | Ruleset/scenario default and custom target | M10–M17 |
| Dice | Independent random or independently documented balanced policy | M10 |
| Robber/pirate | Standard, friendly, early protection where compatible | M10–M17 |
| Discard | Standard/custom base threshold plus explicit expansion modifiers | M10/M11 |
| Trading | Player trades, bank/port behavior, offer expiry | M05/M10 |
| Timing | Untimed/timed turns, setup/response deadlines, disconnect grace | M10 |
| Order | Random or host-assigned starting order; turn variants | M10/M14 |
| Expansion | Scenario setup, missions, awards, special turn rules | M11–M17 |
| Audience | Spectators, delayed public view, chat permissions | M18 |
| Recovery | Disconnect, legal bot substitution, pause/resume | M09/M10 |
| Presets | Standard/casual/fast/expansion/custom, import/export/sharing | M10/M17 |

## Invariants

One versioned GameConfig drives UI, validation, server, persistence, and presets. The server is authoritative online. Gameplay edits invalidate readiness; accepted configuration freezes at start.

No selectable no-op controls. Explain incompatible combinations. Preserve standard presets and visibly label custom rules. Do not enable expansion settings before implemented behavior.

A public board seed is independent of secret deck/dice state. Balanced dice must document its own algorithm without claiming unverified exact reference behavior. Ranked presets are server-owned at M19.

Player counts begin at 2–4; 5–8 arrive at M14 and remain subject to scenario support. Filling empty seats with bots must respect the host-selected total.
