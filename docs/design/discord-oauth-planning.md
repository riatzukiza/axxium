# Discord OAuth: complete existing Axxium story

Proposed refinement of [issue 5](https://github.com/open-hax/axxium/issues/5) and
existing card `axxium-discord-oauth`, Todo / P0 / 5 points. All six original
requirements and four acceptance criteria remain visible in the unchanged card
prefix. This document decides no accepted policy, readiness, migration or feature
implementation. Base: `2439d4d6b8e546cda276f09f5c96db59226ecad6`.

## Source and policy ownership

AGENTS.md is the actual current code policy: pure domain/shape/law, effectful
infra, named `axxium.extern.*` JS boundaries, explicit validators, modern async
and zero warnings. Portable Clojure-shaped `.cljc` is the default where practical
for new identity, consent, callback, grant and role-policy decisions; runtime
storage, HTTP, entropy, clock, JWT/cookie and DOM effects remain outer adapters.
There is no committed separate SECURITY.md, PROCESS.md or project skill in this
base. Kernel specifications describe broader recovered identity/capability
ambitions; they are not evidence of implemented Discord policy or permission to
copy consumer/domain code from another repository.

The local actual source is authoritative for the current seam:

- `routes/auth.cljs`: email signup/login/logout/me/config; public actor filtering
  only removes `password_hash`. There is no Discord callback registration.
- `auth/session.cljs` and `auth/token.cljs`: Axxium session creation reads
  `:actor/id` and `:actor/entity-id`; current routes pass DB keys `:id` and
  `:entity_id`. The design must normalize and validate the DB/domain boundary,
  not presume a new OAuth actor already yields a working session/JWT.
- `shape/db.cljs` and `db.cljs`: entities, actors, sessions and OAuth *client*
  registration tables. The latter is not a Discord provider credential store.
  Provider bindings, credentials and guild evidence need reviewed persistence.
- `schema.cljs`: actor/auth-context capabilities and roles are existing shapes;
  no executable Discord role authorization engine is present. Add a bounded
  role-derived decision seam without importing the broader speculative kernel.
- `config.cljs` has GitHub flags but no Discord app configuration. The current
  public portal is `resources/public/index.html`; it has no existing Discord
  login component. Preserve email/password API behavior and review the portal
  integration rather than claiming those UI flows already exist.

Native inventory at preparation: origin PR 13
`0e0a67299b61ae8ac309385a6400ca603ac3855f`, origin PR 26
`18f92db5ea106b4f9dd3eb4d84b9da0fc6ee9f04`, personal PR 1
`539409b169302793a0b67f97d75ddd6265cf9d12` (draft). Their full body/path captures
show shared auth/session/token/config/DB/server/portal ownership. PR 13's
transfer/deployment and issues 14–25 handoffs remain distinct. PR 26/personal 1
Google, AT Protocol and agent-credential work does not implement this Discord
outcome. Proposed portal.js/account-manager surfaces in those branches are not
accepted source. Before code, review whether/when qualified work supplies the
shared seam, refresh all head/path guards, resolve serialization and UI conflict
with the owner, and prove retained password, identity/link, session, Google/AT
Protocol and agent behavior where actually admitted. No foreign commit, approval
or hard UUID dependency is imported. Discord credentials and derived guild
grants must not silently migrate through PR 13's host-local identity transfer
contract; integration tests must retain its admitted export restrictions. Ownership beyond native writer evidence is
unknown; no absence of activity is asserted.

## Provider protocol contract and primary sources

The proposed server-side confidential authorization-code flow uses Discord's
registered redirect and form-encoded token exchange. Store transaction-bound
state and validate it at callback. Access grants have scopes and expiry; refresh
and revocation are separate operations. [Discord OAuth2 documentation](https://docs.discord.com/developers/topics/oauth2).

`identify` fetches the current user; `guilds` returns partial guild membership.
Role checks use the current-user guild-member endpoint with
`guilds.members.read`, not a guild-list permission field.
[Discord User resource](https://docs.discord.com/developers/resources/user).
The member's `roles` are role IDs, not capabilities or role names.
[Discord Guild Member structure](https://docs.discord.com/developers/resources/guild#guild-member-object).

Use those three least-privilege scopes for this proposed full outcome; email is
not an identity key or a required scope. Bot installation, `guilds.join`, role
connection writes and role mutations are different contracts and are excluded.
Review the enabled API version and bounded parsing/pagination/rate-limit behavior
before implementation; malformed/oversized/unexpected responses are untrusted.

### PKCE and callback security review decision

RFC 9700 recommends PKCE for confidential clients and requires it for public
clients; relying on it requires verified server support. Its transaction,
redirect and mix-up controls inform the proposed boundary.
[OAuth security BCP, sections 2.1 and 4.4](https://www.rfc-editor.org/rfc/rfc9700.html).
The inspected Discord general OAuth2 page does not establish this registered
web application's PKCE behavior. Do not infer it from Social SDK/game examples
or assume that sending ignored parameters secures a flow. Review application
class and actual provider support; prefer S256 where verified. If the required
chosen contract cannot be established, hold qualification rather than weakening
controls. No provider/SDK request or app setting was performed here.

Regardless of that decision, callback state is high-entropy, short-lived,
one-time and server-bound to the initiating browser context, provider,
registered redirect and intent (new login vs explicit account link). Consume it
atomically. Never trust callback/body parameters to select the actor or exchange
endpoint. Missing/wrong/expired/replayed state, provider mismatch, denied consent,
changed redirect, absent code or failed exchange cannot create a session, link,
credential or capability. Bind each provider to its own callback/endpoint tuple
and permit only reviewed local post-login destinations. Keep secrets/codes out
of URL logs and error payloads; callback completion navigates to a clean local
page. Tests must cover interleaved transactions, concurrent callback consumption
and transaction expiry using supplied time and fictional secrets.

## Pure data and effect boundary

Proposed pure shapes: provider identity `{provider-id, subject-id}`; public
profile `{subject-id, username, avatar}` with optional display fields; callback
transaction/intent; actor binding result; grant *metadata* (scope, expiry,
credential reference); membership/role observations keyed by provider subject,
actor and guild; trusted guild-role capability configuration; public actor/auth
context; typed success/rejection/failure decisions. Validators constrain required
IDs, scalar/string shapes, allowed input sizes, positive expiry and scopes.
Never parse Snowflakes into JS floating-point numbers. Token bytes are not
ordinary domain/public values: the credential adapter handles them, exposing an
opaque credential reference and safe grant metadata to decisions.

Pure domain/law decides admissible callback intent, collision/link disposition,
expiry/freshness and role-derived grants from validated Clojure maps and supplied
time. It produces operations for an outer coordinator; it performs no HTTP/DB,
secret retrieval, clock, random generation or signing. Pure shape conversions
explicitly normalize DB columns to the namespaced actor schema before session
creation and public serialization; they do not grant identity from display data.

Named extern adapters own provider HTTP/host objects, form encoding, randomness,
clock, secret protection, PG transaction and JWT/cookie/portal primitives. Infra
coordinates them and rechecks pure contracts at each boundary. Keep Fastify,
DOM, fetch responses and PG objects out of pure authority. The provider remains
replaceable and untrusted; neither its guild permission bitset nor a claimed
role/capability in a callback becomes local authorization configuration.

## Identity linking and persistence proposal

Unique key: trusted Discord provider identity plus returned immutable user ID.
Username, avatar, email, guild name and role name are display data. OAuth signup
creates a new human entity/actor when the provider subject is unbound; a repeat
login resolves exactly that bound actor. Optional missing email must not cause
an invented address or implicit merge with an email/password account.

Linking to an existing actor is a separate explicit action by a currently
verified/re-authenticated Axxium account. Bind the initiating actor and intent to
the transaction; require same actor at callback. Reject a subject already bound
to another actor and ambiguous/stale authentication; never reassign based on an
email match or a provider display name. Review reciprocal uniqueness and allowed
multi-account policy, recovery and unlink behavior before code. Transactional
create/link must enforce uniqueness under concurrent completions with rollback
of partial actor/binding/session writes; no orphan grant or duplicate actor.
Suspended/retired actors cannot acquire an active login through Discord.

Propose additive reviewed DB migration for provider bindings, separate
server-only credentials and actor-associated membership evidence. Store Discord
access tokens as required for API calls, plus refresh token when granted,
expiry, scopes, credential version and binding reference under an explicit
protected-at-rest/access-controlled mechanism. The secret-key lifecycle and
storage mechanism require operator-policy review; do not claim current columns
are protected or provide a fake key/default. A hashed access token alone cannot
serve subsequent Discord API requests. No real credential is placed in Git,
fixtures or evidence. Enforce least-privilege DB access and cleanup/rollback
without merging token material into publicly selected actor rows.

Public actor/membership DTOs use an allowlist and schema validation; me/list/get
actor, config, portal bootstrap, errors/logs and JWT/session claims must never
emit provider tokens, refresh tokens, client secrets, codes or state. Membership
is persisted *with the actor* by durable relation/key and projected public
membership data, not relegated to ephemeral login-only profile state. Audit
metadata may carry IDs/outcome/time, never credential bytes. No new Clio,
Receipt River runtime or board authority is introduced by this auth feature.

## Guild roles, refresh, revocation and authorization

Persist validated guild membership plus actor/provider/subject provenance and
observation time. For configured policy guilds, fetch the current user's member
record under the granted scope, validate role IDs and identity correspondence,
and persist an admitted observation. Do not convert a partial guild-list response
into roles or equate `owner`/Discord permission bits with local administrator.

A trusted reviewed mapping `(guild ID, role ID) -> local capabilities` and a
bounded evidence-freshness policy define pure role checks. Never accept this
mapping from a provider, user request or stored profile. Review the existing
capability-update route's authorization boundary so arbitrary authenticated
callers cannot replace trusted mappings or inject privileged derived grants;
tests exercise that actual route, not only the pure mapper. Keep locally authorized
baseline grants and Discord-derived grants distinct so removing a Discord role
removes its grant without stripping unrelated lawful capabilities. Review the
existing auth-context/request authorization integration and how every sensitive
request obtains current admitted grants; a login-time JWT must not freeze role
privileges indefinitely. Supply a clock value to pure checks; the infra layer
refreshes evidence. Nonmember, removed role, revoked/expired grant, stale/absent
observation, missing consent and provider timeout/403/429/malformed response
cannot yield or retain an unproven Discord-derived privilege. Denied or
unavailable evidence is explicit, not converted to empty successful counts.
The maximum age and failure/retry behavior require review rather than an
invented TTL. Password authentication can remain available under its own policy.

Refresh before using expired provider access credentials; atomically replace
rotated credential material and account for concurrent refresh, failed storage
and invalid/revoked refresh tokens. Disconnect/unlink/revocation invalidates the
local provider credential and derived role authority and attempts the official
revocation operation. A provider outage must not undo local invalidation or
expose a token; retain a safe retry/outcome. Review whether existing Axxium
sessions must be invalidated/reissued when their authorization depends on that
provider, including self-service unlink/recovery and last-login-method handling.
Session invalidation and role-removal tests bind to that reviewed decision.

## Portal and Axxium Bearer contract

Axxium owns the portal login button, configured enablement and denial/error
feedback in this story. Keep client secrets and provider tokens server-side.
Unavailable/disabled config presents truthful UI; successful callback establishes
the reviewed Axxium session and a clean local completion page. Explicit linking
requires authenticated account context; never repurpose anonymous login as link.
Keyboard/accessibility and independent browser contexts are acceptance inputs.
Resolve the accepted static portal versus pending PR 1/26 account-manager or
portal.js integration at review, without copying their implementation now.

Normalize and validate actor data before existing session/JWT issuance. OAuth
access tokens authorize Discord API calls; Axxium Bearer tokens authorize Axxium
API access under Axxium issuer/audience/expiry/actor status/session and current
authorization rules. Token confusion is rejected. Actual route tests must prove
subject/entity/capabilities, stored session and API verification, and retain
password login/logout/me and active/suspended/retired semantics. This design is
not evidence that the accepted baseline already passes those contracts.

## Full sizing, sequencing and holds

Review whether all six requirements/four original outcomes, safe persistence,
callback/linking, portal and role authorization fit the original 5 points. If
not, approve a complete breakdown through the actual planning/Rheos process;
this proposal changes no estimate, UUID, status or dependency metadata.
Review placement, storage/rollback, PKCE/application class, role freshness,
session revocation and integration/ownership before implementation.

Native planning qualification and a lawful Rheos ready transition remain
required. The personal fork's `.github/workflows/auto-merge.yml` eager same-repo
ready trigger invokes floating `open-hax/eta-mu/.../auto-merge.yml@main`, SQUASH,
contents/PR write and inherited secrets. Existing issue 10/workflow PR 13 owns
workflow alignment; do not duplicate or mutate it in this feature plan. Root must
choose safe draft/ready handling under the existing automation hold before
publication, keep auto-merge off and preserve exact-head guards. Local source
planning does not activate policy, staging or release. No remote mutation is
performed by this lane.

The actual Discord app, registered redirect, test guild/roles, protected secret
storage, authorized live/manual test accounts and deployment are not verified.
Mock proof is not live OAuth/API admission. Deployment remains Services-owned
and outside this Axxium implementation scope; do not invent a deployed slot.
Missing prerequisites remain holds, not narrower acceptance criteria. No backend
implementation/compiler/test, live OAuth/SDK/provider or shared-service execution
has run for this candidate. Documented local Git, Rheos read, NBB owning receipt
and portable BB reflection mechanics are planning preparation only.
