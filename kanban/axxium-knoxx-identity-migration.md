---
uuid: axxium-knoxx-identity-migration
title: "Migrate Knoxx identity management to Axxium"
status: todo
priority: P0
labels: ["axxium", "knoxx", "identity", "migration", "auth"]
created_at: "2026-06-02T00:00:00Z"
source: "axxium/kanban/axxium-knoxx-identity-migration.md"
points: 8
category: migration
---

# Migrate Knoxx Identity Management to Axxium

## Current State
Knoxx maintains its own auth layer:
- `backend/src/cljs/knoxx/backend/auth_session.cljs` — session management
- `backend/src/cljs/knoxx/backend/authz.cljs` — authorization logic
- `backend/src/cljs/knoxx/backend/routes/auth.cljs` — auth routes
- `frontend/src/pages/AuthContext.tsx` — React auth context

## Goal
Replace Knoxx's standalone identity system with Axxium as the canonical identity provider.

## Requirements
- [ ] Knoxx authenticates users via Axxium OAuth/login API
- [ ] Knoxx receives and validates Axxium bearer tokens
- [ ] Knoxx reads actor capabilities from Axxium for authorization decisions
- [ ] Knoxx UI uses Axxium session cookies for auth state
- [ ] Migrate existing Knoxx user accounts to Axxium actors
- [ ] Remove Knoxx-native auth_session and authz modules

## Acceptance Criteria
- All Knoxx auth flows route through Axxium
- Knoxx can make policy decisions based on Axxium actor capabilities
- Existing users can log in via Axxium without re-registration
- No standalone auth tables remain in Knoxx

## Related
- Knoxx auth: `orgs/open-hax/openplanner/packages/agents/knoxx/backend/src/cljs/knoxx/backend/`
- Axxium auth routes: `orgs/open-hax/axxium/src/cljs/axxium/routes/auth.cljs`
