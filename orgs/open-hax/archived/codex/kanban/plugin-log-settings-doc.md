---
uuid: "orgs-open-hax-archived-codex-kanban-orgs-open-hax-archived-codex-spec-plugin-log-settings-doc-md"
title: "Plugin log settings docs update"
status: incoming
priority: P3
labels: ["specs", "migrated-spec"]
created_at: "2026-05-29T04:01:16.290Z"
source: "orgs/open-hax/archived/codex/spec/plugin-log-settings-doc.md"
category: "specs"
---

> Source: `orgs/open-hax/archived/codex/spec/plugin-log-settings-doc.md`
> Migrated-to-kanban: `orgs/open-hax/archived/codex/kanban/plugin-log-settings-doc.md`

# Plugin log settings docs update

## Goal

Add logging settings to the README Plugin-Level Settings section so users see rolling log controls alongside existing plugin config options.

## References

- README.md: Plugin-Level Settings section starts at ~49-63.
- docs/configuration.md: Plugin Configuration and Log file management at ~373-420.
- spec/environment-variables.md: notes logging env vars overrideable via ~/.opencode/openhax-codex-config.json (~43).

## Definition of Done

- README Plugin-Level Settings enumerates logging controls (max bytes/files/queue) available via plugin configuration/environment variables.
- Example updated/expanded to show logging block usage.
- Documentation consistent with docs/configuration.md values and defaults.
- No broken markdown formatting.

## Plan

- Phase 1: Align on messaging by pulling log setting names/defaults from docs/configuration.md.
- Phase 2: Update README Plugin-Level Settings bullet list and example to include logging settings.
- Phase 3: Self-review for clarity/consistency; no tests needed (docs-only).
