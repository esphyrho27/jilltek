# Jilltek ISP Management — Fresh Starter

A clean monorepo foundation, not a complete ISP system. Laravel, PHP, Composer and MariaDB are intentionally excluded.

## Stack
- `apps/web`: Next.js 15, React 19, TypeScript
- `services/api`: Rust + Axum
- `services/mikrotik-agent`: Python starter (no router connection configured)
- PostgreSQL/TimescaleDB, PostGIS and Redis via Docker Compose

## Start
1. `cp .env.example .env`
2. `docker compose up -d`
3. In `services/api`: `cargo run`
4. In `apps/web`: `npm install && npm run dev`
5. API health: http://localhost:8080/health ; web: http://localhost:3000

Install Docker Compose, Node.js 22+, Rust stable and Python 3.11+ first.

The API currently exposes only `/health`. The Python agent is a safe heartbeat placeholder and does not connect to routers. This is a starting foundation, not production-ready software.

Before production: implement authentication/RBAC, input validation, audit logs, restricted CORS, TLS, secret management, backups/restore tests, observability, rate limits and security reviews. Never expose database/Redis ports publicly or commit `.env`.

Database schema changes should be versioned in `database/migrations/`. This is a new schema lifecycle, not a migration of the old Laravel application.
