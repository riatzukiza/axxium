---
uuid: axxium-discord-oauth
title: "Add Discord OAuth to Axxium"
status: todo
priority: P0
labels: ["auth", "oauth", "discord", "axxium"]
created_at: "2026-06-02T00:00:00Z"
source: "axxium/kanban/axxium-discord-oauth.md"
points: 5
category: auth
---

# Add Discord OAuth to Axxium

## Goal
Enable users to create accounts and authenticate with Discord OAuth, alongside existing email/password auth.

## Requirements
- [ ] Discord OAuth2 callback route (`/api/auth/discord/callback`)
- [ ] Discord user info fetch (username, avatar, guild memberships)
- [ ] Link Discord identity to Axxium actor (create or link existing)
- [ ] Store Discord access token for API calls (guild role checks)
- [ ] UI: Discord login button on portal
- [ ] Policy: Derive capabilities from Discord guild roles

## Acceptance Criteria
- Users can sign up/login with Discord
- Discord guild membership is stored on actor
- Policy engine can check Discord roles for authorization decisions
- Bearer tokens work for API access after OAuth login

## Related
- Axxium auth routes: `src/cljs/axxium/routes/auth.cljs`
- Axxium session: `src/cljs/axxium/auth/session.cljs`
- Axxium config: OAuth client ID/secret in env/config


## Proposed current-source planning refinement

This is a proposal for the complete existing [Axxium issue 5](https://github.com/open-hax/axxium/issues/5),
not implementation or accepted authentication policy. The original body above,
UUID `axxium-discord-oauth`, Todo, P0 and 5 points remain unchanged. Actual
Rheos reads the committed configuration and this card; that does not qualify a
planning review or a ready transition. No new card or artificial UUID dependency
replaces this story.

### Context

Accepted source `2439d4d6b8e546cda276f09f5c96db59226ecad6` has email/password
routes and Axxium JWT/session primitives, but no Discord callback, provider
subject binding, provider-credential store or stored guild-member role evidence.
The portal is presently an Axxium-owned static page. Existing actor serializers
remove only `password_hash`; provider secrets cannot be added to actor/public
maps. DB rows and the actor shape used by session/token creation also differ;
normalization and genuine Axxium Bearer verification need explicit coverage.
See the [design](../docs/design/discord-oauth-planning.md) and
[verification proposal](../docs/verification/discord-oauth-planning.md).

Origin PRs 13 and 26 and personal PR 1 already own intersecting auth, token,
session, DB, server/config and portal changes. Their full native scope is
preserved as predecessor evidence, not adopted code or approval. Review must
decide the integration base and portal implementation after those heads are
qualified, then repeat path/head and compatibility guards before implementation.
The Discord feature is not described by their current bodies. This is an
outcome distinction, not a claim that the eventual files are disjoint.

### Outcome and scope

The complete user outcome remains Discord signup/login, safely linking an
existing actor, storing membership on that actor and a server-only provider
credential for guild-role API calls, displaying a portal login button, evaluating
role-derived capabilities, and authenticating API requests with Axxium Bearer
tokens. No profile-only substitute or exclusion of token storage is proposed.

| Original requirement | Complete proposed implementation and acceptance proof |
| --- | --- |
| Callback `/api/auth/discord/callback` | Server-side authorization code exchange, provider-specific redirect/state, denial/replay/mismatch controls and no session on failure. |
| User info: username, avatar, memberships | Validate stable user ID plus optional display data; fetch and persist the user's guild memberships, separately validate member-role evidence. |
| Create/link identity to actor | Unique provider/subject binding, authenticated explicit linking, transactional create/link and takeover/conflict/concurrency controls. |
| Store access token for API/guild roles | Separate protected server-only access/refresh credential storage with expiry, refresh, disconnect/revocation and zero public/log/JWT exposure. |
| Portal Discord login button | Accessible Axxium-owned login/denial/error/link UI and preserved email/password experience; resolve current portal seam at the integration review. |
| Guild roles derive capabilities | Portable pure mapping over trusted guild/role IDs and fresh admitted evidence; missing scopes, removal, stale/unavailable evidence and untrusted role claims cannot grant privilege. |

### Acceptance criteria: retain all four original outcomes

1. Users can sign up and subsequently log in with Discord, while existing
   email/password behavior remains covered. Invalid, expired, replayed,
   wrong-context and denied callbacks fail safely; independent transactions
   cannot cross-link identities or sessions.
2. Discord guild membership is persisted with the linked actor, provider and
   observation provenance. Optional profile fields and mutable names cannot
   replace stable IDs. A member-role lookup for each configured policy guild
   supplies role IDs; a guild-list response alone supplies no role grants.
3. Authorization checks use an explicit trusted guild-ID/role-ID capability
   mapping, validated actor/auth context and defined freshness/revocation
   semantics. Positive and negative cases cover removed roles, nonmembership,
   missing consent, provider failures and stale caches. Locally authorized
   capabilities remain distinguishable from provider-derived grants.
4. The completed callback yields the reviewed Axxium session/Bearer contract;
   a real route verification accepts the Axxium token and rejects a Discord
   access token, expired/suspended actor sessions and malformed claims. No
   provider access/refresh token appears in responses, cookies, browser storage,
   JWT claims, logs, diagnostics or planning artifacts.

### Non-goals

Consumer migrations in Knoxx, Proxx or OpenPlanner; deployment/Services changes;
Discord bot installation, guild joining, role mutation or self-bot behavior;
Google/AT Protocol/agent-credential implementation owned by existing PRs; an
Axxium-wide auth rewrite; new board/event-ledger authority. Reviewing security
and integration contracts does not transfer foreign branch ownership.

### Verification and review decisions

Review the full five-point estimate and all ten original obligations. If secure
persistence, callback/UI and role evaluation do not fit, propose a lawful
breakdown retaining the entire outcome; do not silently change points, status,
criteria or declare the smaller profile slice complete. Review proposed pure
`.cljc` shapes/laws and outer Discord HTTP/clock/randomness/DB/secret/portal
adapters under AGENTS.md, plus provider-specific PKCE applicability, public
serialization, migration/backfill/rollback and same-actor concurrency.

Future red/green covers shape validation, callback/link/credential/role laws,
mocked HTTP/transaction/portal adapters and full route/session/token regressions,
then the unchanged relevant typecheck, test, build, JS/deployment boundaries and
zero-warning obligations. Injected async failures must make the official test
command fail. See the verification proposal for exact current commands and the
separate authorized live artifact; no backend compiler/test/login/provider or
shared service runs for this planning change.

### Risks and admission holds

The OAuth app registration, exact redirects, controlled test guild/roles,
protected credential-storage mechanism, expiry/freshness policy and authorized
live verification have not been supplied or verified. Current official docs
support `identify`, `guilds`, and `guilds.members.read` for the proposed outcome;
PKCE behavior must be verified for the actual registered application, not
assumed from another Discord SDK. These are implementation/live-qualification
holds. Planning review, dependency/integration/size decisions and a lawful Rheos
ready transition still precede implementation. Existing floating eager squash
caller and source/review-gate history require coordinated safe publication;
no workflow/settings/trust change is included here.
