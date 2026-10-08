# Whole Axxium workflow planning proposal

License: GPL-3.0-or-later.

## Controlling scope and accepted source

This refines existing `axxium-github-workflow` / [issue10](https://github.com/open-hax/axxium/issues/10).
All seven original requirements, five acceptance criteria and both protection
clauses remain controlling input. The original card is an unchanged byte prefix,
with native Todo/P1/5. This plan proposes contracts and verification, not code or
activation. Accepted Axxium/main and mapped personal/main are
`2439d4d6b8e546cda276f09f5c96db59226ecad6` at the captured guard.
No proposed PR is substituted for accepted source.

The accepted repository has four workflows: `ci.yml`, `auto-merge.yml`,
`review-resolution-gate.yml`, and `kanban-sync.yml`. CI uses Node22/Java21 and
`npm install`, typecheck, release build and tests. It currently accepts the
compiler/empty-suite results identified below. Auto-merge is a floating
eta-mu/main reusable caller with SQUASH and inherited secrets; its non-draft,
same-repository predicate is an existing guard, not proof of safe readiness or
whole controller trust. Review gate calls a floating eta-mu workflow. Kanban sync
is a native owner bridge; do not add a second board writer/validator here.

## Explicit review decision: original route versus current user policy

The historical feature/staging/main clauses are not silently deleted or reported
satisfied. Review must approve a concrete mapping of every original outcome to
the current personal-fork development and qualified origin release/deploy policy:

1. An authorized feature branch in `riatzukiza/axxium` opens a development PR on
   its qualified personal base. All actual review findings and required gates
   bind the current feature head, current base and tested merge candidate.
2. Only an admitted feature merge with trusted repository, origin and purpose
   evidence admits Services staging at its immutable merge SHA M. A protected
   accepted-upstream sync is terminal and must not initiate feature deployment.
   Missing, stale, ambiguous or unknown-purpose evidence refuses.
3. Exact Services staging proof for M admits a separately reviewed origin
   release/promotion PR and any affected Foresight integration. Promotion inputs
   identify the exact feature/artifact and current origin base; a newer moving
   personal tip cannot substitute for the qualified feature. Changed bases,
   artifacts or receipts require requalification.
4. A qualified owning-origin main release/deployment still requires its separate
   production gate. No direct production effect follows a planning merge or
   mere personal/origin push. Only after the separately qualified production deployment of owning main, the trusted
   publisher creates the exact-binding release tag (R7).

Original R1/AC1–2 correspond to development PR creation and evidence-bound
staging-to-origin promotion, with the historical literal branch target changes
explicitly pending review. R2/R3/AC5 retain actual automatic branch-merge-triggered
staging and production outcomes only for admitted events. The original protection
clauses and R4/R5/AC3–4 require an explicit mapping to current user canonical
review policy; no title/comment regex, reviewer impersonation or local policy
copy supplies that mapping. Do not implement until the owner accepts the route.

The original CodeRabbit AND OpenCode requirements are explicitly mandatory.
The default all-available roster and optional native quota exception cannot waive
either obligation. Review must identify an actually qualified Axxium OpenCode
route, trusted native principal and current-head publication contract; the name
of a host/job alone is not that proof. The Proxx-only `opencode-agent[bot]` Kimi
publication profile does not admit Axxium evidence. A different provider, quota
notice or unconfigured identity cannot stand in for mandatory OpenCode approval.
The original protection clauses stay held until this owner mapping is reviewed
and actual activation/readback proves them; no local classifier or waiver.

## Ownership and integration decisions

Origin PR13 `0e0a67299b61ae8ac309385a6400ca603ac3855f` proposes environment
promotion, Services caller pinning and identity/test changes. Personal PR1
`539409b169302793a0b67f97d75ddd6265cf9d12` owns identity repair and compiled
warning-free/nonempty-suite evidence. They overlap the package, test, shadow,
auth and deployment seams. Their bodies, complete native path sets and exact
heads are preserved in the sealed owner recovery packet; none of their code,
review or operational claims is adopted here. Before implementation, reacquire
current qualified heads, agree source ownership and either consume a qualified
accepted repair or propose an independently owned compatible repair. Do not
silently carry pending identity code into a workflow change.

Native `axxium-service-deployment` / issue 9 retains Services-owned immutable
image, host placement, migration, ingress health and protected host-trust proof.
Services owns host effects; Axxium owns its source/build/runtime contracts.
Eta-mu owns trusted publisher/review/promotion machinery and Rheos board behavior;
root project/fork mapping must be qualified where consumed. These are explicit
owner integration prerequisites, not implementation permission or fabricated
UUID dependencies. Issue 27 owns the separate append-only ADD/ADD composition
contract: preserve both receipt/reflection histories and event identities across
future integration; successful isolated planning does not resolve it.

## Pure data and outer effects

Propose portable `.cljc` data/contracts for Axxium's build result and immutable
release input, with explicit source repository, source/head/base/merge identity,
artifact digest, result status, warning counts, discovered/executed test counts,
assertion/failure/error counts and original producer provenance. Review the
schema/result ABI before code. Missing or inconsistent producer evidence is
unknown/refused, never coerced to success. Positive suite evidence requires
meaningful known tests; a numeric positive count alone cannot excuse omitted
required namespaces or a vacuous replacement suite.

Canonical pr-flow supplies review eligibility and settlement; shared trusted
promotion supplies event purpose/admission. This plan does not recreate those
laws inside Axxium. GitHub API, compiler process/log capture, artifact transport,
tagging and Services calls remain named outer adapters. Raw objects, filesystem,
network/clock effects and credentials cannot enter pure laws. The producer must
report actual exit/result/diagnostics, not derive authoritative admission from
mutable caches, titles or unchecked provider claims.

Candidate builds/dependency hooks run without signing/deployment secrets and with
read-only tokens, including same-repository proposals. Privileged publication
and protection admission must be bound to trusted App/pinned base code that
never executes candidate code; gate/config/App identity must not be candidate
controlled. Explicit least-privilege event matrices must cover push, new DRAFT,
opened/ready/synchronize/edited/retargeted PR, check-suite, merge, release/tag and
manual dispatch. DRAFT alone is not a universal trust proof. Activation requires
real selected-event evidence; no settings/credential change is made here.

## Requirement 6: actual warning and empty-suite cause

Existing MEMBER comments6031359488 and6047124243 preserve hosted CI SUCCESS plus
six undeclared-variable warnings in `src/cljs/axxium/routes/auth.cljs` at
101:7,101:22,102:22,102:46,103:47,104:43 for each server compile/release, and zero
tests/zero assertions. Independent technical intake at accepted2439 uses the
core reader without evaluation: `handle-login` is form7 without catch, and the
catch is separate top-level form8. The accepted source's test selector has no
tracked matching test namespace, and `:autorun true` plus the explicit Node
runner explains repeated empty execution. This is static syntax/discovery proof
and historical hosted observation, not a new compiled RED or repaired contract.
The immutable technical packet is recorded in verification provenance.

A future qualified repair must reproduce the malformed catch failure safely,
restore intended async success/rejection/catch semantics in the owning runtime,
and prove warning rejection independently of compiler exit0. It must discover
and execute meaningful real laws/adapters through the configured test target;
missing/empty suites, zero assertions, suppressed namespaces, deliberate async
rejection and failures/errors cannot pass. Review autorun/explicit-run strategy
and count provenance so output duplication does not create extra suite credit.
Keep current real identity tests when integrating qualified PR1/13; do not port
unqualified code or delete tests just to obtain a green pipeline. All source
changes still require full relevant npm typecheck/test/build and boundary gates.

Local planning preparation also ran the unchanged static JS-boundary scanner:
`node scripts/check-js-boundary.mjs --check` exits 1 and reports 53 pattern findings
across seven accepted files (auth/session, auth/token, config, routes/actor,
routes/auth, routes/health and shape/db). The deployment-boundary self-test/check
exit 0. These are exact recorded scanner outcomes, not full compiler/test proof.
The raw-interop baseline remains an R6 qualification hold. Coordinate qualified
owner repairs/current PR1/13 integration before claiming full required gates;
this documentation proposal does not repair those files, weaken the scanner,
expand into unrelated identity implementation or treat a narrow pass as full
qualification. Future meaningful full gates must expose retained baseline failures.

## Deployment, tag and failure contract

Services staging/production requests bind exact purpose, source/merge/artifact,
configuration/environment, migration and required gate evidence. Isolation must
preserve tenant/service slots and prevent one candidate overwriting another's
artifacts or leases. Missing placement, credentials or host authorization is a
held owner prerequisite; an offline fixture cannot stand in for actual deployment.
Refused/cancelled/failed/partial operations retain their original failure and
available receipt/provenance. No deploy, promotion or tag after gate refusal.
Review retry, concurrency, migration recovery and rollback contracts with Services
before code, retaining identity/session/data compatibility and existing-user
continuity; rollback cannot restore revoked/stale authentication authority.

The trusted tag publisher uses qualified production/main-deployment commit and artifact
bindings. Same tag/same binding retry is idempotent, conflicting existing tag
refuses without force; concurrent requests cannot produce two conflicting tags.
A successful build or promotion alone does not prove a completed deployment/tag.
Actual tag/readback provenance and unaltered failure histories are required.

## Scope, sizing and qualification

All original outcomes remain one governing five-point card. Review whether the
whole CI repair, controller integration, protection/review mapping, promotion,
production and tag/human proof fit five points. If not, propose a complete lawful
breakdown through Rheos after review, retaining every outcome; do not manufacture
children or silently lower the task to a small warning fix. Native status and
estimate stay unchanged. This proposal uses no new board config/state/event.

No implementation, provider request, backend/compiler/test run, live deployment,
new secret/protection/controller setting, Ready, approval or activation is claimed.
Local preparation consists only of source/native inspection, first-class Markdown,
unchanged static boundary checks, native card visibility, immutable accountability
and preservation checks. A prospective personal DRAFT PR remains blocked and
requires a fresh selected receiving route/defaults/callee guard; root is the
sole publisher. Full future verification is in the companion matrix.
