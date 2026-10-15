# Milestone 05 — Complete base rules

Target release: **v0.0.5**. Status: Planned. This is an initial plan; refine against actual code and dependencies before implementation.

[Roadmap](../../ROADMAP.md) · [Progress](progress.md) · [Reports](reports/) · [Screenshots](../../photos/milestone-05/index.md)

## Goal and dependencies

Complete the base-game mechanics required to finish a valid match.

Requires milestone 04 to pass its release gate. Reuse the existing rules/protocol contracts and relevant previously released systems; do not bypass unresolved earlier gates.

## Scope and implementation tasks

- [ ] Implement player offers, acceptance, cancellation, bank trades, and ports.
- [ ] Implement seven discards, robber movement, victim selection, and private random theft.
- [ ] Implement the complete base development-card inventory and timing restrictions.
- [ ] Implement Longest Road, Largest Army, public/private scoring, and victory timing.
- [ ] Add native choice dialogs and recipient-safe public action history.

## Interfaces, state, persistence, and UI

Represent multi-step choices explicitly in engine phases. Public events never include stolen-card identity or private card contents for unauthorized recipients.

## Tests and acceptance checks

- [ ] Complete scripted games with a verified winner.
- [ ] Cover every development card, trade rejection/acceptance, and discard/robber response.
- [ ] Test road branches, loops, blocking, award ties, hidden points, and victory timing.

## Screenshot evidence

Capture trading, discarding, robber choice, development-card actions, and endgame summary using synthetic identities.

Record actual captures and metadata in the linked photo index. Explain nonvisual work and absent captures truthfully.

## Compatibility and migration

Keep base-only rules separate from future expansion replacements such as progress cards and knight pieces.

## Release gate and handoff

Base games finish correctly; mechanic and private-information edge cases have regression coverage.

Update progress, append actual implementation reports, index inspected photos, update ROADMAP.md/CHANGELOG.md, and complete evidence review before marking Complete. Then summarize the release and stop for the next request. User-requested Polish mode pauses advancement.
## Branch, user testing, and GitHub release

Use a dedicated milestone-NN/description branch and PR against stable for this milestone. nightly follows latest pushed work; beta must match the active unapproved PR head. Leave the milestone Verifying and PR open for the user's testing/explicit approval. Do not merge, advance stable, publish the intended GitHub Release, or begin the next milestone before approval. After approval and passing gates, release the merged stable commit and link PR/release/wiki/evidence in the records. See [delivery workflow](../../docs/github-workflow.md).
