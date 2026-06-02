---
uuid: axxium-openplanner-identity-migration
title: "Migrate Openplanner identity management to Axxium"
status: todo
priority: P0
labels: ["axxium", "openplanner", "identity", "migration", "auth"]
created_at: "2026-06-02T00:00:00Z"
source: "axxium/kanban/axxium-openplanner-identity-migration.md"
points: 8
category: migration
---

# Migrate Openplanner Identity Management to Axxium

## Current State
Openplanner (through Knoxx and related packages) has scattered auth:
- Knoxx auth layer (part of openplanner monorepo)
- Various package-level API keys and tokens
- No unified identity system across graph-weaver, cephalon, etc.

## Goal
Unify Openplanner's identity and authorization under Axxium.

## Requirements
- [ ] All Openplanner services validate requests via Axxium
- [ ] Openplanner actor context uses Axxium DIDs
- [ ] Graph-weaver/cephalon agents authenticate as Axxium actors
- [ ] Epistemic kernel facts are attributed to Axxium actor IDs
- [ ] Remove package-level auth silos in favor of Axxium bearer tokens

## Acceptance Criteria
- Openplanner API endpoints require valid Axxium bearer token
- Agent actions in cephalon/graph-weaver are attributable to Axxium actors
- No standalone auth per package; all go through Axxium
- Users can access Openplanner with the same Axxium identity used for Proxx/Knoxx

## Related
- Openplanner packages: `orgs/open-hax/openplanner/packages/`
- Axxium actor routes: `orgs/open-hax/axxium/src/cljs/axxium/routes/actor.cljs`
