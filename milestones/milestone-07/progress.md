# Milestone 07 progress — Supabase foundation

Status: Planned  
Target release: v0.1.1  
Last updated: 2026-10-15

[Plan](plan.md) · [Reports](reports/) · [Photos](../../photos/milestone-07/index.md) · [Roadmap](../../ROADMAP.md)

## Landed work

No implementation work has started. This initial record establishes scope only.

## Implementation checklist

- [ ] Create or connect a new Supabase organization independent of Scout App and confirm organization-specific cost before provisioning the new project.
- [ ] Configure guest authentication and securely stored native refresh credentials.
- [ ] Implement private schemas, restricted server credentials, explicit grants, and applicable RLS.
- [ ] Add versioned migrations and server persistence for rooms, memberships, snapshots, receipts, and events.
- [ ] Provide placeholder environment examples and verify identity/database connectivity without exposing credentials.

## Verification

- [ ] Reproduce the schema on an isolated test database.
- [ ] Test guest token verification and least-privilege database access.
- [ ] Verify unauthenticated clients and other guests cannot read authoritative snapshots or secrets.

Unchecked items are pending, not failed or passed by inference. Structure-check results do not imply application build/runtime verification.

## Remaining work and dependencies

Await completion of milestone 06 and refinement of this plan against the then-current implementation.

A new connected Supabase organization and organization-specific cost confirmation are required before project creation.

## Blockers

No implementation blocker investigated yet; predecessor work remains planned.

## Polish checklist

No active Polish mode request or reported flaw checklist. See [Polish mode](../../AGENTS.md#polish-mode).

## Evidence and next step

No builds, tests, screenshots, reports, or release artifacts exist for this milestone. Next: begin only after the predecessor gate and user request.
