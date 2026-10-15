# Milestone 18 progress — Public casual platform

Status: Planned  
Target release: v0.12.0  
Last updated: 2026-10-15

[Plan](plan.md) · [Reports](reports/) · [Photos](../../photos/milestone-18/index.md) · [Roadmap](../../ROADMAP.md)

## Landed work

No implementation work has started. This initial record establishes scope only.

## Implementation checklist

- [ ] Add optional email accounts and safe anonymous-guest linking.
- [ ] Add public room discovery, filters, casual matchmaking, profiles, and friend/invitation flows.
- [ ] Add spectators with permitted/delayed views and post-game replay rules.
- [ ] Add chat permissions, mute, block, report, and moderation records.
- [ ] Add abuse controls and rate limits around identity, discovery, joining, chat, and reports.

## Verification

- [ ] Verify account linking retains membership and permitted history without identity takeover.
- [ ] Test discovery visibility, private room/password boundaries, blocks, and moderation permissions.
- [ ] Inspect spectator/chat/replay payloads and logs for leaks and unsafe input rendering.

Unchecked items are pending, not failed or passed by inference. Structure-check results do not imply application build/runtime verification.

## Remaining work and dependencies

Await completion of milestone 17 and refinement of this plan against the then-current implementation.

## Blockers

No implementation blocker investigated yet; predecessor work remains planned.

## Polish checklist

No active Polish mode request or reported flaw checklist. See [Polish mode](../../AGENTS.md#polish-mode).

## Evidence and next step

No builds, tests, screenshots, reports, or release artifacts exist for this milestone. Next: begin only after the predecessor gate and user request.
