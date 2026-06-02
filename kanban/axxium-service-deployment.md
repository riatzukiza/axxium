---
uuid: axxium-service-deployment
title: "Add Axxium to orgs/open-hax/services deployment"
status: todo
priority: P1
labels: ["axxium", "deployment", "services", "docker", "pm2"]
created_at: "2026-06-02T00:00:00Z"
source: "axxium/kanban/axxium-service-deployment.md"
points: 5
category: infrastructure
---

# Add Axxium to orgs/open-hax/services Deployment

## Goal
Integrate Axxium into the OpenHax services deployment infrastructure alongside Proxx, Knoxx, and Openplanner.

## Requirements
- [ ] Create `services/axxium/` directory with deployment configs
- [ ] Docker Compose setup for Axxium + PostgreSQL
- [ ] PM2 ecosystem config for production
- [ ] Environment variable template (`.env.example`)
- [ ] Health check endpoint integration
- [ ] Database migration strategy

## Acceptance Criteria
- `docker compose up` in services/axxium starts the full stack
- PM2 can manage Axxium process in production
- Database migrations run automatically on startup
- Health check passes for load balancer integration

## Related
- Proxx services: `services/proxx/`
- Axxium server: `orgs/open-hax/axxium/src/cljs/axxium/server.cljs`
