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
