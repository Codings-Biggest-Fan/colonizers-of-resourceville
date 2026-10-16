# Deployment and operations

**Status:** M01 supplies a liveness-only Rust server container and unsigned testing bundles. No cloud project, hosted service, approved release, or production deployment exists. See [M01 evidence](../milestones/milestone-01/progress.md).

## Planned boundaries

- M01 builds/packages on GitHub for Windows/Linux/macOS x86-64 and ARM64. Local native testing is Windows-first; nonlocal compilation does not prove interactive runtime support. Linux jobs also build the nonroot server container. No image is pushed to a registry.
- M07 creates/connects a new Supabase organization and a separate game project after actual organization-specific cost confirmation.
- Supabase provides Auth/Postgres. The persistent Rust Axum/Tokio service is hosted separately.
- M19 deploys the Docker server with TLS, health/metrics, structured privacy-safe logs, backup/restore rehearsal, rollback, and load verification.
- M20 proves clean-install Windows offline and deployed-online behavior.

Do not reuse Scout App projects or change scouting resources. Access to a connected new organization is a provisioning prerequisite, not an excuse to silently choose another organization.

Keep credentials outside tracked files and desktop packages. Use restricted database roles, private schemas, applicable RLS/grants, TLS, and real permission tests. Provider selection and actual hosting costs must be documented and confirmed before provisioning; no provider resources have been created.

Store deployment/recovery test evidence in the relevant milestone report and photo index where visual. Never equate local two-client testing with verified internet hosting.
