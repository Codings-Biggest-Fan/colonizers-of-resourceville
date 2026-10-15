# Milestone 07 — Supabase foundation

Target release: **v0.1.1**. Status: Planned. This is an initial plan; refine against actual code and dependencies before implementation.

[Roadmap](../../ROADMAP.md) · [Progress](progress.md) · [Reports](reports/) · [Screenshots](../../photos/milestone-07/index.md)

## Goal and dependencies

Create isolated online identity and durable storage foundations.

Requires milestone 06 to pass its release gate. Reuse the existing rules/protocol contracts and relevant previously released systems; do not bypass unresolved earlier gates.

## Scope and implementation tasks

- [ ] Create or connect a new Supabase organization independent of Scout App and confirm organization-specific cost before provisioning the new project.
- [ ] Configure guest authentication and securely stored native refresh credentials.
- [ ] Implement private schemas, restricted server credentials, explicit grants, and applicable RLS.
- [ ] Add versioned migrations and server persistence for rooms, memberships, snapshots, receipts, and events.
- [ ] Provide placeholder environment examples and verify identity/database connectivity without exposing credentials.

## Interfaces, state, persistence, and UI

Rust server uses SQLx and TLS; desktop receives only publishable authentication configuration. No authoritative game tables or service-role secrets are available to clients.

## Tests and acceptance checks

- [ ] Reproduce the schema on an isolated test database.
- [ ] Test guest token verification and least-privilege database access.
- [ ] Verify unauthenticated clients and other guests cannot read authoritative snapshots or secrets.

## Screenshot evidence

Capture native guest/connection UI if introduced, with synthetic identities and no tokens. For database-only work, document the absence of a UI change.

Record actual captures and metadata in the linked photo index. Explain nonvisual work and absent captures truthfully.

## Compatibility and migration

Keep local saves independent of Supabase availability. Require connected new-organization access and actual cost confirmation before external creation.

## Release gate and handoff

Authentication, migrations, persistence smoke checks, and permission tests pass.

Update progress, append actual implementation reports, index inspected photos, update ROADMAP.md/CHANGELOG.md, and complete evidence review before marking Complete. Then summarize the release and stop for the next request. User-requested Polish mode pauses advancement.
