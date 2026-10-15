# Deployment and operations

**Status:** no project, hosted service, container, or release bundle exists.

## Planned boundaries

- M01 creates native/build/package foundations; do not claim runtime packaging until exercised.
- M07 creates/connects a new Supabase organization and a separate hearthline-remake project after actual organization-specific cost confirmation.
- Supabase provides Auth/Postgres. The persistent Rust Axum/Tokio service is hosted separately.
- M19 deploys the Docker server with TLS, health/metrics, structured privacy-safe logs, backup/restore rehearsal, rollback, and load verification.
- M20 proves clean-install Windows offline and deployed-online behavior.

Do not reuse Scout App projects or change scouting resources. Access to a connected new organization is a provisioning prerequisite, not an excuse to silently choose another organization.

Keep credentials outside tracked files and desktop packages. Use restricted database roles, private schemas, applicable RLS/grants, TLS, and real permission tests. Provider selection and actual hosting costs must be documented and confirmed before provisioning; no provider resources have been created.

Store deployment/recovery test evidence in the relevant milestone report and photo index where visual. Never equate local two-client testing with verified internet hosting.
