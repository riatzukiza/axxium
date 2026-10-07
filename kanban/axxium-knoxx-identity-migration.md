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


## Proposed complete planning refinement for issue 6

This is authored planning input for [open-hax/axxium issue 6](https://github.com/open-hax/axxium/issues/6), using the existing canonical UUID `axxium-knoxx-identity-migration`. All six original requirements, four acceptance criteria, scope and Todo/P0/8 metadata above remain controlling. No requirement is marked complete. The original Current State paths are historical: the accepted Knoxx snapshot now places auth in `backend/src/cljs/knoxx/backend/infra/auth/` and `infra/routes/auth.cljs`, with a mounted CLJS frontend auth boundary.

### Context and outcome

Axxium accepted and personal main at `2439d4d6b8e546cda276f09f5c96db59226ecad6` owns the identity kernel. Independently read Knoxx accepted `3409977bca4ba35e09967f9a99d50867a679a73c` already has an operator-configured HTTPS Axxium password-login adapter. It deliberately discards the remote cookie, creates a local Knoxx session, binds issuer plus actor subject, and retains local roles. This partial coexistence is accepted source behavior, not proof of the whole migration. The outcome remains Axxium-backed login, Bearer admission, capabilities, browser session state, existing-user continuity and retirement of standalone Knoxx authentication authority.

### Scope and ownership

Axxium owns portable actor/session/capability admission and identity-migration decisions plus its outer login, verification, cookie, database and migration adapters. Knoxx independently owns request authentication, tenant policy integration, actual mounted UI, its old account/storage readers and eventual retirement of its auth modules. Services owns deployment/ingress and operator environments; this plan neither configures nor deploys them. No OpenPlanner companion implementation, fork synchronization, foreign PR adoption or new board engine is included. Knoxx source remains unchanged in this planning PR.

### Complete requirement crosswalk

| Original requirement | Proposed full obligation |
| --- | --- |
| Knoxx authenticates users via Axxium OAuth/login API | Use the actual Axxium login API as the first integration boundary. Inventory GitHub callbacks, local signup/password, invitations, bootstrap and machine callers; review every replacement/forwarding route. OAuth authorization-server/callback behavior must be separately specified and tested if selected; current config flags/schema are not implemented OAuth grants. No hidden local fallback after Axxium refusal. |
| Knoxx receives and validates Axxium bearer tokens | Select exact issuer, audience, algorithm/key trust, expiry and actor/session revocation contract before code. Validate supplied tokens at the trusted edge with Axxium-owned verification/status evidence, normalize to a typed principal, refuse ambiguous cookie/Bearer identities and never admit by decoded claims alone. |
| Knoxx reads actor capabilities from Axxium for authorization decisions | Use authenticated, active, fresh Axxium capability evidence as a required authorization input. Review explicit capability-to-Knoxx permission mapping and combine with tenant/membership/resource policy by restriction; remote role strings never create local administrator rights. |
| Knoxx UI uses Axxium session cookies for auth state | Replace the current local-cookie behavior through a reviewed same-origin Axxium-auth routing contract or a reviewed cross-origin contract. The selected proposal is same-origin proxying of Axxium auth, with Axxium-issued HttpOnly cookies and Axxium-backed context/logout; no JavaScript-readable token or fresh standalone Knoxx session. Cookie scope, CSRF/origin, CORS/credentials, expiry and logout/revocation are mandatory tested decisions. |
| Migrate existing Knoxx user accounts to Axxium actors | Inventory all existing account kinds. Preserve stable subject/resource attribution and existing access using explicit migration bindings, credential compatibility or proof-bound linking. No email-only automatic merge, re-registration requirement or privilege upgrade; collisions, retries, partial failure and concurrent login must be handled before cutover. |
| Remove Knoxx-native auth_session and authz modules | Retire current `infra.auth.auth-session`, local auth/session implementation and `infra.auth.authz` after callers are migrated to reviewed Knoxx policy adapters consuming Axxium context. Preserve their resource/tenant/tool/conversation authorization obligations; deleting authorization checks is not migration. |

### All four original acceptance outcomes remain mandatory

1. **All Knoxx auth flows route through Axxium:** complete route, background and browser inventory with refusal controls; no selectable local credential/session bypass in the qualified final mode.
2. **Knoxx can make policy decisions based on Axxium actor capabilities:** explicit reviewed capability mapping, fresh/revoked evidence, tenant/resource restrictions and audited decisions before effects.
3. **Existing users can log in via Axxium without re-registration:** every supported legacy account kind has a proof-backed mapping and migration/rollback fixture; conflicting accounts remain held rather than silently merged.
4. **No standalone auth tables remain in Knoxx:** inventory actual Mongo collections and every writer/bootstrap/index path, not only SQL tables. Remove redundant passwords, auth tokens and session authority after cutover and rollback-window review; any retained actor-ID or membership/resource projection is non-authenticating and justified explicitly. Relabeling an active auth collection is insufficient.

### Proposed sequencing and review decisions

Retain the eight-point aggregate. An implementation breakdown, if required by actual cross-repository cost, must be reviewed against this full outcome before new operational cards or estimates are admitted; this refinement creates no duplicate umbrella or readiness claim. First review the [design](../docs/design/knoxx-identity-migration-planning.md) and [verification contract](../docs/verification/knoxx-identity-migration-planning.md), including token/cookie trust, capability mapping, existing-account proof and rollback. Then use each owner's lawful planning and native Ready admission. Write portable laws and meaningful dual-host failing tests first, implement owner adapters second, prove coexistence with legacy flows unchanged, migrate and test every account kind, switch qualified auth admission, and only then remove old modules/standalone storage with removal and rollback evidence. The full acceptance gate includes both owning repositories and their current receiving/deployment holds.

### Verification, non-goals and risks

Future verification includes portable JVM plus compiled CLJS admission laws; actual Axxium login/Bearer/session/capability adapters; actual Knoxx request hooks/routes and mounted frontend; repeatable isolated migration/rollback/concurrency tests; non-empty relevant suites, zero warnings, production builds and boundary gates in both owners; and a runnable human Knoxx/Axxium verification script with owned fixture teardown and a browser tour. Required integration is not replaced by mocked responses or successful compilation.

This PR contains no implementation, executed backend suite, provider login, database migration, cookie setting, deployment, token/key change, operational board comment/transition or user-account modification. It does not close issue 6, adopt current foreign kernel proposals, complete Discord issue 5 or deployment/workflow issues 9/10, or grant publication trust. Missing approved trust/credential migration/capability mapping or mandatory integration evidence remains an explicit hold.


## Proposed native planning-review corrections at original head 431d348

Native CodeRabbit review `5448463764` at `431d3481c6d923b44503c4fa80899a423d2e11ec` requested explicit Axxium status/introspection ownership (`4212075340`), post-cutover credential/account-safe rollback (`4212075349`) and the verification wording `Unauthenticated (401)` (`4212075357`). This ordinary planning successor retains the entire prior body and unchanged UUID/Todo/P0/8 plus all six requirements and four acceptance criteria. Findings remain native obligations until root publishes and settles them; this authored proposal supplies no approval, operational admission or implementation.

The design now proposes an Axxium-owned versioned `POST /api/auth/introspect` deliverable, closed portable boundary shapes, authenticated current actor/session/capability/freshness facts before Knoxx effects, and distinct 401 authentication rejection, 403 verified insufficient grant and retryable fail-closed 503 unknown/unimplemented/unavailable evidence. Method/path/version, service trust, freshness bounds and revision protocols require review before code; current actor-only `/api/auth/me`, raw row normalization or decoded claims do not implement the contract. Keep issuer signing keys and all credentials outside consuming authority, logs and browser projections.

The whole migration also requires cutover epoch and authenticated per-account credential/account revision watermarks. The proposed recovery restores a qualified consumer route while retaining Axxium authority, invalidates changed legacy credentials, preserves post-cutover/new and existing users without re-registration, and never back-imports hashes or restores revoked access. Legacy-authority rollback needs separately reviewed complete writer-freeze/change/watermark proof; unknown or stale evidence holds the switch. Added future fixtures cover post-cutover password/reset, disable/revocation/rotation, new signup/invitation, user kinds, concurrent changes and replay before effects. All original full-scope law, adapter, human/browser, nonempty suite, zero-warning and owner integration/removal gates remain mandatory; no narrow pure fixture replaces them.
