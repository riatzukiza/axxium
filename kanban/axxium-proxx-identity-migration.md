---
uuid: axxium-proxx-identity-migration
title: "Migrate Proxx identity management to Axxium"
status: todo
priority: P0
labels: ["axxium", "proxx", "identity", "migration", "auth"]
created_at: "2026-06-02T00:00:00Z"
source: "axxium/kanban/axxium-proxx-identity-migration.md"
points: 8
category: migration
---

# Migrate Proxx Identity Management to Axxium

## Current State
Proxx has a multi-provider auth system:
- `src/lib/auth/` — auth types, SQL persistence, GitHub allowlist
- `src/lib/factory-oauth.ts` — Factory OAuth flow
- `src/lib/openai-oauth.ts` — OpenAI OAuth flow
- `src/lib/native-auth.ts` — Native token auth
- `src/lib/request-auth.ts` — Request authentication
- `src/routes/credentials/` — OAuth UI routes

## Goal
Replace Proxx's native identity and OAuth management with Axxium as the canonical identity provider.

## Requirements
- [ ] Proxx validates all requests via Axxium bearer tokens
- [ ] Proxx reads actor capabilities from Axxium for policy decisions
- [ ] Factory/OpenAI OAuth credentials remain in Proxx (provider-specific)
- [ ] User identity (who) moves to Axxium; provider tokens (what) stay in Proxx
- [ ] Proxx policy engine queries Axxium for actor roles/capabilities
- [ ] Remove Proxx-native user/session tables where redundant

## Acceptance Criteria
- Proxx API requests authenticate via Axxium tokens
- Policy decisions use Axxium actor capabilities as input
- Provider OAuth flows (Factory, OpenAI) still work but identity is Axxium-backed
- No duplicate user identity tables between Proxx and Axxium

## Related
- Proxx auth: `orgs/open-hax/proxx/src/lib/auth/`
- Axxium token verification: `orgs/open-hax/axxium/src/cljs/axxium/auth/token.cljs`
