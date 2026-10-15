# Architecture

**Status:** target architecture, no implementation yet. [Overall plan](implementation-plan.md) · [M01](../milestones/milestone-01/plan.md).

| Boundary | Responsibility | Restrictions |
| --- | --- | --- |
| rules | Topology, phases, action validation, state transitions, scenario capabilities | Pure Rust; no UI, database, filesystem, networking, or async runtime |
| protocol | Versioned configuration, commands/events, recipient observations | No secret state exposed by default |
| bots | Decisions using permitted observations/legal choices | No privileged access to opponents or future randomness |
| desktop | egui/eframe + wgpu rendering, input, native identity/saves | No server credentials or authoritative online writes |
| server | Axum/Tokio authentication, membership, commands, private projections, SQLx durability | Validate identity/action; persist before acknowledgment |

Offline execution runs the same engine locally. Online execution runs it on the server. Presentation state does not become game state merely to simplify UI.

Keep authoritative snapshots distinct from public and private player observations. Runtime random outcomes are controlled by the executor; tests inject deterministic fixtures. Board-only seeds do not reveal decks or dice.

M02 finalizes shared schemas/versions. M07 introduces Supabase; M09 proves online durability/recovery. No speculative code or dependencies are claimed to exist.
