# Discord OAuth verification proposal

For existing Axxium issue 5, UUID `axxium-discord-oauth`, Todo / P0 / 5 points.
These are future implementation obligations, not executed software tests or an
accepted ready transition. The complete original six requirements and four
acceptance criteria remain in the card. Keep all scope if review requires a
lawful breakdown. Base `2439d4d6b8e546cda276f09f5c96db59226ecad6`.

## Red then green: concrete behavior and meaningful controls

1. Portable pure `.cljc` shape/law tests: stable string IDs, optional profile
   fields, exact actor normalization, one-use callback state/intent/provider/
   redirect/expiry, distinct contexts, absent/wrong/replayed/denied callback,
   actor status, explicit link and conflicting/duplicate identities. Supply
   time, proposed operations and fictional provider records; no entropy or DB
   inside those laws. Where `.cljc` is practical, exercise the same data/laws
   across the selected portable host and compiled CLJS host; any unsupported
   host dependency remains a reported boundary decision, never a false pass. Red must fail because Discord behavior is absent, not
   because a dependency/tool or compiler failed.
2. Mocked adapter/route tests: exact form-encoded code and refresh exchange,
   required scopes, denial/malformed/oversized responses, timeout/403/429,
   callback endpoint binding, safe errors, concurrent state consumption and
   unique transactional create/link. Fault injection at DB, binding, credential
   write and session creation proves rollback/no orphan authorization. Two
   independent contexts cannot overwrite another's transaction or credential.
3. Server credential store and serializer tests: access/refresh are retrievable
   only through the protected adapter for required API use; public me/list/get,
   portal/config, JWT/cookie payloads, errors, recorded logs and diagnostics
   contain no credential bytes. Fictional canaries prove exclusion, not mere
   absence in successful fixtures. Test expiry/refresh rotation, simultaneous
   refresh, invalid grant, local invalidation plus failing remote revocation,
   unlink/takeover conflicts and reviewed migration/backfill/rollback behavior.
4. Persist membership and fetch configured guild member roles. A partial guild
   list cannot grant capabilities. Positive known `(guild ID, role ID)` cases
   and negative unknown/mismatched guild/subject, renamed display data, missing
   scope, nonmember, removed role, stale evidence, revoked credential and
   provider failure cases prove current policy decisions. Distinguish local
   grants from derived grants; requests with stale login-token role claims
   cannot retain removed privileges under the reviewed authorization design.
   Arbitrary capability-update callers must not inject provider-derived grants;
   retained identity-transfer exports must exclude provider credentials and
   unauthorized derived authority.
5. Portal/mock callback and full session/API tests: accessible login button,
   disabled/unavailable config, denied consent/error feedback, new and returning
   login, explicitly authenticated linking, retained password signup/login/
   logout/me, actor lifecycle, correct normalized Axxium subject/entity/context.
   Accept an actual Axxium-issued Bearer token at the protected API and reject a
   Discord access token, wrong audience/issuer, expiry and revoked/suspended
   sessions. Review current portal integration against qualified PR 1/26 rather
   than assuming their branch is accepted. No consumer repository edit needed.
6. Deliberately rejected async mock and failing assertion must make the official
   test command nonzero with truthful counts. Retain existing meaningful tests
   at the chosen qualified integration base; do not filter namespaces, suppress
   errors, waive warning ratchets or report compiler exit 0 as test success.

## Required current commands and integration guard

Accepted package scripts are npm-owned: CI Node 22/Java 21 installs with
`npm install`, then typecheck, build and test. Preserve package manager, locks,
compiler options and CI; no dependency/install/compiler executed by this plan.
Current accepted source has no committed `*-test` namespace. The implementation
must add a reviewed discoverable test source path/runner for the full contracts
under the existing official `test` target; a zero-test compile is not proof.

After native planning/ready admission and qualified source integration:

```sh
npm run typecheck
npm test
npm run build
npm run boundary:check
node scripts/check-deployment-boundary.mjs --self-test
node scripts/check-deployment-boundary.mjs
```

Zero errors, failures and AGENTS.md warning obligations apply, including the
relevant configured clj-kondo checks. A known historical violation or missing
tool remains an explicit owning hold; it is not converted to a green narrowed
suite. New pure `.cljc` namespace discovery/portable host coverage and test
source configuration are review decisions before code, not a second package
policy. Fresh exact main/PR 1/13/26/path guards and owner coordination precede
any shared auth/DB/portal edit. This planning branch adopts none of those heads.

## Separate authorized live acceptance artifact

After app/redirect/guild/roles/storage and isolated test-account prerequisites
are authorized, run only the reviewed Axxium OAuth/API/portal acceptance surface.
No real login/backend/provider/DB/service/deployment is authorized by this plan.
The future artifact binds exact application/source revision, registered redirect
identity, reviewed scopes and safe configuration references, observer/test actor
and guild/role IDs, time, result, command exits and sanitized diagnostic hashes.
Never record callback codes/state, access/refresh/JWT tokens, Authorization
headers, secrets, browser profile exports or opaque provider URLs.

Prove every original outcome: signup and returning Discord login; explicit safe
link and takeover rejection; stored actor-associated guild memberships; real
member-role lookup and role removal/nonmember denial under current policy;
portal button/denial feedback; subsequent authenticated Axxium Bearer API call
and Discord-token rejection. Verify disconnect/refresh/expiry and resulting
local authorization/session invalidation as reviewed. Supply paired isolated
contexts and negative controls. Unavailable app, live account, configuration or
service slot means NOT RUN/HOLD, not an offline live-success substitute. Services
deployment and consumer migration cards remain separate and unmodified.

## Actual planning-only preparation

Evidence under `.ημ/verification/axxium5-discord-planning` records source/fork/
head guards, full scoped native ownership, exact original-card prefix, actual
Rheos original and proposed-body readbacks, typed owning Receipt River validation
and portable Session Mycology writer/reader, lossless capture hashes and full-tip
immutability/hygiene. Local planning preparation provides no provider approval,
review round, ready transition, production build or live auth qualification.
