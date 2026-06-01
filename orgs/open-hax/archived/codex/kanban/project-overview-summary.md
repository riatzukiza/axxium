---
uuid: "orgs-open-hax-archived-codex-kanban-orgs-open-hax-archived-codex-spec-project-overview-summary-md"
title: "Project Overview & Remaining Work (2025-11-18)"
status: incoming
priority: P3
labels: ["specs", "migrated-spec"]
created_at: "2026-05-29T04:01:16.281Z"
source: "orgs/open-hax/archived/codex/spec/project-overview-summary.md"
category: "specs"
---

> Source: `orgs/open-hax/archived/codex/spec/project-overview-summary.md`
> Migrated-to-kanban: `orgs/open-hax/archived/codex/kanban/project-overview-summary.md`

# Project Overview & Remaining Work (2025-11-18)

## Context
- User request: explain what this project does and what work remains.
- Repository root: `orgs/open-hax/codex` (OpenHax Codex plugin for OpenCode).

## Code Files & References
- README.md:1-83 — product summary, OAuth-based Codex plugin, GPT-5.1/5 presets, prompt caching, built-in commands.
- docs/code-cleanup-summary.md:1-110 — completed refactors and current code quality state.
- spec/mutation-score-improvement.md:4-62 — mutation testing status; survivors remain in `lib/request/request-transformer.ts`, `lib/prompts/codex.ts`, and `lib/constants.ts`.
- spec/review-response-automation.md:1-56 — planned review-comment automation workflow; assets (`.opencode/agent`, workflow, scripts) still absent.
- spec/review-response-bot-filter.md:1-24 — workflow should whitelist CodeRabbit/maintainers; filter logic not yet implemented.
- spec/review-response-workflow-fix.md:1-27 — YAML indentation fix needed for review-response workflow heredoc.
- spec/codex-metrics-sse-fix.md:1-43 — `/codex-metrics` SSE output must emit typed events to satisfy Responses schema; tests need updates.

## Existing Issues / PRs
- Issue #36 `[BUG] Codex-mini doesn't work`; related PR #37 `feat: Normalize Codex Mini naming to gpt-5-codex-mini` (from mutation score plan).
- Issue #6 "Feature: richer Codex metrics and request inspection commands" (drives `/codex-metrics` SSE work).
- Issue #10 "Review Response Bot Filter Fix" (tighten workflow triggering rules).

## Definition of Done
- Provide the user with a concise description of the project purpose and a snapshot of key outstanding work items.

## Requirements
- Summaries should cite relevant files/specs and align with repository conventions; no code changes required for this response.
