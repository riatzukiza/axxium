---
uuid: "orgs-open-hax-archived-codex-kanban-orgs-open-hax-archived-codex-spec-toast-word-wrap-md"
title: "Toast word wrap spec"
status: incoming
priority: P3
labels: ["specs", "migrated-spec"]
created_at: "2026-05-29T04:01:16.286Z"
source: "orgs/open-hax/archived/codex/spec/toast-word-wrap.md"
category: "specs"
---

> Source: `orgs/open-hax/archived/codex/spec/toast-word-wrap.md`
> Migrated-to-kanban: `orgs/open-hax/archived/codex/kanban/toast-word-wrap.md`

# Toast word wrap spec

## Code references

- lib/logger.ts:170-195 — `notifyToast` builds toast body and sends via `tui.showToast`, currently sends single-line message.

## Existing issues/PRs

- None found yet in this repository.

## Definition of done

- Toast messages no longer get truncated; long text wraps across lines in the TUI.
- Warning/error toasts still appear with existing title/variant semantics.
- Tests cover the wrapped formatting behavior.

## Requirements

- Introduce safe word-wrapping for toast message bodies before calling `showToast` (keep readable at typical terminal widths).
- Prefer whole-word wrapping; avoid mangling short messages.
- Preserve existing behavior for short messages and existing title/variant fields.
