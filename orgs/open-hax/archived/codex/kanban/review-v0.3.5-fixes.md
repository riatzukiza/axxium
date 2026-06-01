---
uuid: "orgs-open-hax-archived-codex-kanban-orgs-open-hax-archived-codex-spec-review-v0-3-5-fixes-md"
title: "Review v0.3.5 fixes"
status: incoming
priority: P3
labels: ["specs", "migrated-spec"]
created_at: "2026-05-29T04:01:16.295Z"
source: "orgs/open-hax/archived/codex/spec/review-v0.3.5-fixes.md"
category: "specs"
---

> Source: `orgs/open-hax/archived/codex/spec/review-v0.3.5-fixes.md`
> Migrated-to-kanban: `orgs/open-hax/archived/codex/kanban/review-v0.3.5-fixes.md`

# Review v0.3.5 fixes

## Scope

- Handle null/empty cache reads in `lib/prompts/codex.ts` around readCachedInstructions caching logic
- Prevent duplicate tool remap injection in `lib/request/input-filters.ts` addToolRemapMessage

## Existing issues / PRs

- None identified for this branch (review/v0.3.5).

## Definition of done

- safeReadFile null results do not get cached as empty content; fallback logic remains available for caller
- Tool remap message is only prepended once when tools are present; logic handles undefined/null safely
- All relevant tests updated or added if behavior changes; existing suite passes locally if run

## Requirements / notes

- Only cache instructions when actual non-empty content is read; on null either warn and return null or allow existing fallback paths
- removeLastUserMessage should find last user role index and slice; when commandText is falsy reuse originalInput directly in maybeBuildCompactionPrompt
- addToolRemapMessage should fingerprint or compare TOOL_REMAP_MESSAGE and skip if already present (matching role/type/text)
- Preserve existing function signatures and return types throughout
