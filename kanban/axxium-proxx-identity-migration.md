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


## Whole-outcome planning refinement (2026-10-07)

This is a proposal for the existing issue 8 and UUID, preserving the six requirements and four acceptance criteria above. Todo, P0 and 8 points remain unchanged. The current owning/personal Axxium main is `2439d4d6b8e546cda276f09f5c96db59226ecad6`; the independently inspected Proxx owning/personal main is `88471efd7f8cd27f64fc733a1c31369f00a6d432`. These source observations do not promote child pins, an open kernel PR, a plan, or this card to qualified runtime authority.

### Outcome and scope

Axxium becomes the canonical identity and token/session authority for Proxx API callers, UI users and machine principals; Proxx consumes authenticated actor capabilities for its own tenant/provider policy. Preserve every existing user's access and tenant memberships, every tenant API credential's scoped continuity, and every provider account kind. Factory/OpenAI browser/device OAuth, refresh, API-key and local provider transport credentials stay with Proxx. Provider identity metadata is not proof of the caller's identity. Locally privileged billing/provider secrets must never enter identity claims, public fixtures or cross-repository migration payloads.

The complete migration includes login/callback/refresh/logout/me; HTTP API and administration; independent SSE/WebSocket ingress and federation/bridge admission; the actual browser token/cookie UI; tenant selection/membership/role and API-key management; storage migration, retired local identity/session writers, rollback and operational verification. No partial ingress or interface-only result satisfies this card. `src/lib/native-auth.ts` currently implements native Ollama transport authentication, so it is not automatically a redundant user/session module.

### Admission decisions required before implementation

Review the exact issuer/subject identity mapping, audience/key trust, session revocation/refresh, tenant-scoped capability vocabulary, machine/API-key exchange or equivalent Axxium-backed admission, and bootstrap/public-route contract. The original “all requests via Axxium bearer tokens” remains the target. Existing tenant keys, administrator shared tokens, UI cookies, public/auth bootstrap routes and trusted-local bridge headers need an explicit reviewed disposition; they are not permanent bypasses. Login/callback/health/static/preflight exceptions must be individually justified as protocol bootstrap/non-identity surfaces and reviewed against that requirement, rather than silently narrowing it. If reviewers require a change to the literal requirement, obtain an explicit owner decision before implementation, not a hidden exception.

Axxium's accepted email/password and HS256 JWT/session source does not itself prove complete GitHub/federated login, machine-principal or online revocation integration. Coordinate accepted kernel work with existing origin PRs 26/13 and personal PR 1; do not adopt their unmerged branches or infer whole migration coverage. Knoxx issue 6 and Discord issue 5 remain separate consumers. Proxx issue 13 telemetry and receiving-route trust/controller holds remain separate obligations.

### Acceptance and verification crosswalk

All six requirements and four original acceptance criteria are mapped in [the design](../docs/design/proxx-identity-migration-planning.md) and [the verification matrix](../docs/design/proxx-identity-migration-verification.md). Require source-bound positive and adversarial evidence for every ingress and credential kind, stable migration identities, tenant isolation, downgrade refusal, cookie/CSRF/origin/revocation, provider callback replay/ownership and retirement/rollback. Pure laws have zero I/O; authorized resolution reads are explicit, and denied admission forbids unauthorized reads and all prohibited writes/provider/session effects.

Run all actual owning backend, boundary, schema, CLJS, frontend and human browser/API gates in isolated resources at the future implementation head. No tests, database migration, provider login, board transition, deployment or hosted approval are executed or claimed by this planning change. Operational credentials and release/controller qualification are separate prerequisites; never enable candidate-controlled secret execution to unblock a test.

### Breakdown and risks

Eight points remains the existing estimate. Proposed review batches are identity/admission and account continuity; Proxx ingress/policy/browser/provider integration; and retirement/rollback/full verification. Review dependencies and fair sizing before any story creation or implementation; do not pretend each batch is independently complete or auto-create child UUIDs. Missing machine/federated kernel admission, ambiguous old login identities, cookie trust, revocation behavior, consumer ownership and unsafe receiving workflows are explicit holds. Only qualified planning plus a lawful native Rheos Ready transition can admit implementation. This refinement does not change card state or claim exclusive ownership.

### Proposed recovery/status clarification (2026-10-07)

The whole outcome also requires owner-reviewed recovery authority and authenticated migration change provenance: keep Axxium identity/session/status/capability authority during rollback or fail closed until forward repair; never revive a pre-cutover local login/session/role/key snapshot. Preserve post-cutover created, invited and Axxium-only users and all product attribution without forced re-registration or repeat administrator bootstrap. Reconcile linking/account/credential/session/status/capability/membership/tenant API-key changes through versioned epochs and qualified owning watermarks, refusing stale/conflicting/unknown provenance. Factory/OpenAI/local/provider secrets and product state remain Proxx-owned.

The design and matrix append proposed positive/concurrent/hostile recovery controls plus an Axxium-owned versioned verification/introspection contract. They distinguish proposed authentication refusal (401), verified-but-denied authorization (403), and unavailable/unverifiable mandatory authority (503), with explicit suspended/retired status decisions and no allow fallback. These are future owner decisions and acceptance proofs, not an implemented API or a claim of operational readiness. All original six requirements/four AC, Todo/P0/8, future full gates and ownership/receiving holds remain unchanged.
