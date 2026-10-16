# server

Separate Axum/Tokio Rust service for authoritative online games, authentication, private projections, and durable SQLx persistence.

**Status:** M01 implements Axum/Tokio process liveness at /health, bound to loopback by default. No authentication, rooms, game state, SQLx, or cloud integration exists yet. See [M01 progress](../../milestones/milestone-01/progress.md).
