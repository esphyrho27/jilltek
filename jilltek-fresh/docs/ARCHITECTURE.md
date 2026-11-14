# Architecture
- Browser -> Next.js frontend -> Rust API.
- Rust owns business rules, authorization, validation, persistence and event publication.
- PostgreSQL is the source of truth; TimescaleDB stores telemetry; PostGIS handles spatial queries.
- Redis supports caching and ephemeral coordination.
- Python agent should make authenticated outbound connections to the API; do not expose router management ports publicly.

Before production: implement authentication/RBAC, TLS, restricted CORS, secret management, audit logs, backups and restore drills, monitoring, rate limits, and security review.
