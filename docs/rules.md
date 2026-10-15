# Rules and source register

**Status:** source-backed inventory and rule decisions are scheduled for [M02](../milestones/milestone-02/plan.md). No rules engine is implemented.

## Source policy

Use public specifications, rulebooks, and ordinary gameplay observation. Record title, edition/date, source URL, access date, and applicable interpretation before implementing a family. Do not copy proprietary assets or code.

Starting discovery sources (not yet pinned/archived implementation oracles):

- [Official CATAN core and expansion rulebook library](https://www.catan.com/understand-catan/game-rules)
- [Public Colonist lobby](https://colonist.io/)
- [Public Colonist Cities & Knights explanation](https://colonist.io/catan-rules/cities-and-knights)

M02 must enumerate all base development cards, expansion progress cards, core scenarios, objective variants, supported player counts, and documented combination rules. Record uncertainties rather than guessing.

## Accepted distinctions

- Initial two-seat base play uses the same base rules as a labeled casual variant.
- The official Traders & Barbarians two-player variant is a separate preset at M15.
- Official 5–6-player turn extensions and custom 7–8-player extensions are separate.
- Cities & Knights and Explorers & Pirates intentionally replace some base behavior; do not layer incompatible mechanisms implicitly.
- Victory targets, awards, turn phases, card limits, resource shortages, and forced choices come from the chosen ruleset/scenario.
- Supported expansion combinations require documented interaction rules and their own tests.

A menu item, board render, or successful build does not establish rule correctness.
