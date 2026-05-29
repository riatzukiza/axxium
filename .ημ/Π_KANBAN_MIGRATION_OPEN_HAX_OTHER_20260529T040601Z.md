# Π handoff: eta-mu kanban migration batch agent_open_hax_other

- timestamp: 20260529T040601Z
- batch_manifest: /tmp/eta-mu-kanban-batches/agent_open_hax_other.json
- migration: node services/eta-mu/kanban/scripts/migrate-specs-to-kanban.mjs --root /home/err/devel --manifest /tmp/eta-mu-kanban-batches/agent_open_hax_other.json
- verification: eta-mu-beta kanban count --tasks-dir for every unique boardDir in the manifest
- concurrency: path-scoped staging only; existing unrelated root/submodule dirt left untouched
- touched root paths: archived codex/museeks/opencode-skills boards plus parent pointers/docs for open-hax migrated repos
