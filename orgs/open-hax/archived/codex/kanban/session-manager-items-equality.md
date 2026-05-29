---
uuid: "orgs-open-hax-archived-codex-kanban-orgs-open-hax-archived-codex-spec-session-manager-items-equality-md"
title: "Session Manager item equality helper"
status: incoming
priority: P3
labels: ["specs", "migrated-spec"]
created_at: "2026-05-29T04:01:16.272Z"
source: "orgs/open-hax/archived/codex/spec/session-manager-items-equality.md"
category: "specs"
---

> Source: `orgs/open-hax/archived/codex/spec/session-manager-items-equality.md`
> Migrated-to-kanban: `orgs/open-hax/archived/codex/kanban/session-manager-items-equality.md`

# Session Manager item equality helper

## Scope and references

- lib/session/session-manager.ts: longestSharedPrefixLength (around lines 22-37) uses JSON.stringify comparison
- lib/session/session-manager.ts: findSuffixReuseStart (around lines 92-103) uses JSON.stringify comparison

## Existing issues

- None observed related to this refactor (no issue referenced in request)

## Existing PRs

- None observed; task requested directly by review comment

## Definition of done

- Shared helper (e.g., itemsEqual) added to centralize equality check with safe JSON stringify in try/catch
- longestSharedPrefixLength and findSuffixReuseStart use the helper instead of inline JSON.stringify comparisons
- Build/test commands remain passing or noted if not run

## Requirements

- Avoid duplicated JSON.stringify comparisons; centralize error handling for failed stringify
- Apply helper in both functions mentioned in review
- Create new branch and open PR targeting dev when changes are complete
