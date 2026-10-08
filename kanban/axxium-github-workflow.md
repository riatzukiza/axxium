---
uuid: axxium-github-workflow
title: "Set up GitHub workflow for Axxium (feat -> PR -> staging -> main)"
status: todo
priority: P1
labels: ["axxium", "github", "ci", "workflow", "deployment"]
created_at: "2026-06-02T00:00:00Z"
source: "axxium/kanban/axxium-github-workflow.md"
points: 5
category: infrastructure
---

# Set up GitHub Workflow for Axxium

## Goal
Implement the same branch promotion and review workflow used by other OpenHax services.

## Requirements
- [ ] GitHub Actions workflow: `feat -> PR -> staging -> PR -> main`
- [ ] Staging deployment on merge to `staging` branch
- [ ] Production deployment on merge to `main` branch
- [ ] CodeRabbit AI review is required (branch protection)
- [ ] OpenCode review is required (branch protection)
- [ ] CI checks: build, typecheck, tests
- [ ] Automatic tag creation on main deploy

## Branch Protection Rules
- `staging`: requires 1 review, passing CI
- `main`: requires CodeRabbit review + OpenCode review, passing CI

## Acceptance Criteria
- Feature branches open PRs to staging
- Staging merges open PRs to main
- CodeRabbit comments block merge until resolved
- OpenCode review approval blocks merge until resolved
- Deployments trigger automatically on branch merge

## Related
- Proxx workflows: `orgs/open-hax/proxx/.github/workflows/`
- Axxium repo: `https://github.com/open-hax/axxium`


## Proposed full-scope planning refinement — 2026-10-07

### Context and outcome

[Issue 10](https://github.com/open-hax/axxium/issues/10) owns this whole workflow,
including seven requirements, five acceptance criteria and both original branch
protection clauses above. Compiler warnings and empty test discovery are defects
within requirement 6; repairing them alone cannot complete the workflow. The
original UUID, Todo/P1/5, scope and history are retained. This body is a proposal
for planning review, with no lifecycle transition or operational admission.

### Proposed route reconciliation

The original `feat -> PR -> staging -> PR -> main` wording remains literal
historical scope. The user now requires development on the mapped personal fork,
then a separately qualified release/deploy into the owning origin. Review must
approve the explicit replacement routing contract in
[the design](../docs/design/github-workflow-planning.md) before implementation:
personal feature PR and qualified merge, Services staging at its exact merge
SHA, then evidence-bound origin promotion/release PR and independent production
qualification. Terminal accepted-upstream synchronization cannot start feature
deployment; unknown purpose fails closed. This proposal does not claim that the
old literal branch clauses already pass or that this route is activated.

### Complete acceptance and verification crosswalk

- R1 and AC1–2: preserve both original branch-promotion outcomes, with an explicit
  reviewed personal-fork/Services/origin mapping, immutable feature/merge/base
  bindings and genuine PR creation/promotion event proofs. No direct origin
  development, default-branch rewrite or eager controller activation.
- R2 and AC5: actual admitted Services staging at the exact qualified feature
  merge; health/migration/ingress evidence and isolated environment ownership.
- R3 and AC5: separate production authorization, immutable artifact/promotion
  binding, health/migration evidence and failure/rollback contract. Axxium
  workflows must not acquire direct host deployment authority.
- R4, R5 and AC3–4: use the canonical pr-flow review/settlement and required-check
  owner, retain CodeRabbit and OpenCode obligations and original protection
  clauses, and review their mapping to the user's current eligible exact-head
  approval policy. All actual findings and required gates remain obligations;
  skipped/quota/old-head evidence is not approval. No alternate local classifier. Both original reviewers remain mandatory;
  optional quota exceptions and Proxx-only OpenCode/Kimi identity cannot waive
  or supply Axxium approval.
- R6: zero-warning typecheck/release, honest nonempty discovered tests, preserved
  meaningful existing tests, warning/empty-suite failure controls, current
  head/base/tested-merge evidence and required JS/deployment boundary checks.
  The accepted malformed login catch and missing test namespaces are grounded in
  the independent immutable technical packet, not repaired here.
- R7: authenticated automatic tag publication only after qualified main
  deployment at its exact artifact/commit, with idempotent same-binding retry,
  conflicting tag refusal and no success tag after failed/skipped deployment.

The complete red/green, hosted and authorized human acceptance matrix is in
[the verification proposal](../docs/verification/github-workflow-planning.md).
No fixture-only success substitutes for original staging/production proofs.

### Scope, ownership and risks

Rooted in accepted Axxium `2439d4d6b8e546cda276f09f5c96db59226ecad6`.
Origin PR13 at `0e0a67299b61ae8ac309385a6400ca603ac3855f` owns proposed
cross-host identity/environment promotion and a pinned Services caller;
personal PR1 at `539409b169302793a0b67f97d75ddd6265cf9d12` owns concrete
identity repair/test changes. Neither is accepted main or imported into this
plan. Fresh qualification, merge/overlap review and full-source gate evidence
are needed before implementation; source/code from those proposals is not
adopted. Existing `axxium-service-deployment`/issue 9 owns Services placement and
live verification. Issue 27 independently owns parallel receipt/reflection
composition. No hard dependency metadata or new child UUID is invented here.

The original five points remain. Review must decide whether the entire workflow,
trusted publisher integration, gate repair and real staging/production proof
fit five points. If not, request a lawful full-scope decomposition/estimate
through Rheos after review; no small CI-only substitute or fabricated children.

### Non-goals and admission holds

No implementation, workflow/configuration edit, secret/protection/App setting,
provider invitation, board event, status change, merge, tag or deployment.
Pure portable shapes/contracts belong in `.cljc` where practical; build/process,
GitHub and Services effects stay in named outer adapters. Canonical Rheos,
Receipt River, pr-flow and trusted promotion/publisher owners are consumed,
never duplicated. A new DRAFT personal planning PR is prospective only and needs
its fresh receiving-event/callee/defaults/auto-off guard. Ready and implementation
require their own native planning and Rheos qualification.

Software/process documentation: GPL-3.0-or-later.
